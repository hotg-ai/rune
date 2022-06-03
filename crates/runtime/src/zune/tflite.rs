use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use anyhow::{Context, Error};
use hotg_rune_compiler::parse::ModelStage;
use hotg_rune_core::TFLITE_MIMETYPE;
use hotg_runecoral::{
    AccelerationBackend, ElementType as RuneCoralElementType, InferenceContext,
    Tensor as RuneCoralTensor, TensorDescriptor as RuneCoralTensorDescriptor,
    TensorMut as RuneCoralTensorMut,
};

use crate::zune::{
    get_buffer_size, key, proc_block::Dimensions, ElementType, GraphContext,
    State, TensorConstraint, TensorResult,
};

pub(crate) struct ModelNode {
    context: InferenceContext,
    input_tensors: HashSet<usize>,
    output_tensors: HashSet<usize>,
    shared_state: Arc<Mutex<State>>,
}

impl ModelNode {
    #[tracing::instrument(
        skip(
            node_data,
            model_data,
            shared_state,
            input_tensors,
            output_tensors
        ),
        level = "debug"
    )]
    pub(crate) fn load(
        node_id: &str,
        node_data: &ModelStage,
        model_data: &[u8],
        shared_state: &Arc<Mutex<State>>,
        input_tensors: &HashMap<String, usize>,
        output_tensors: &HashMap<String, usize>,
    ) -> Result<ModelNode, Error> {
        // Create Inference Context
        let context = InferenceContext::create_context(
            TFLITE_MIMETYPE,
            &model_data,
            AccelerationBackend::NONE,
        )
        .with_context(|| {
            format!(
                "Error Instantiating model from zune for stage: {}",
                &node_id
            )
        })?;

        let tensor_from_descriptor =
            |t: &RuneCoralTensorDescriptor| -> TensorResult {
                let element_type = get_element_type(t);
                let dimensions = t.shape.iter().map(|&x| x as u32).collect();
                let buffer_size = get_buffer_size(element_type, &dimensions);

                TensorResult {
                    element_type,
                    dimensions,
                    buffer: vec![0; buffer_size],
                }
            };

        let tensor_constraint_from_descriptor =
            |t: &RuneCoralTensorDescriptor,
             tensor_id: usize|
             -> TensorConstraint {
                let element_type = get_element_type(t);
                let dimensions = t.shape.iter().map(|&x| x as usize).collect();

                TensorConstraint {
                    tensor_id: Some(tensor_id),
                    element_type,
                    dimensions: Dimensions::Fixed(dimensions),
                }
            };

        // Returns the list of tensor indices in the State's tensors
        let allocate_tensors = |tensor_type: &str,
                                model_tensors: &mut dyn Iterator<
            Item = RuneCoralTensorDescriptor,
        >,
                                pipeline_tensors: &HashMap<String, usize>|
         -> Result<
            (HashSet<usize>, HashMap<String, TensorConstraint>),
            Error,
        > {
            let mut tensor_indices: HashSet<usize> = HashSet::new();
            let mut tensor_constraints: HashMap<String, TensorConstraint> =
                HashMap::new();
            let mut i = 0;
            let mut s = shared_state.lock().unwrap();

            while let Some(model_tensor) = model_tensors.next() {
                let tensor_key = key(&node_id, Some(i));
                let tensor_id =
                    *pipeline_tensors.get(&tensor_key).ok_or_else(|| {
                        anyhow::anyhow!(
                            "Unable to find pipeline_tensor for {} tensor \
                             with key {}",
                            &tensor_type,
                            &tensor_key
                        )
                    })?;

                let tensor_name = model_tensor.name.to_str().ok();
                let tensor_name = match tensor_name {
                    Some(tensor_name) if tensor_name.len() > 0 => {
                        tensor_name.to_string()
                    },
                    _ => format!("{}", i).to_string(),
                };
                let tensor_constraint =
                    tensor_constraint_from_descriptor(&model_tensor, tensor_id);
                let model_tensor = tensor_from_descriptor(&model_tensor);

                match s.tensors[tensor_id] {
                    Some(ref t)
                        if t.dimensions != model_tensor.dimensions
                            || t.element_type != model_tensor.element_type =>
                    {
                        anyhow::bail!(
                            "Pipeline tensor for {} with key {} doesn't match \
                             model tensor",
                            &tensor_type,
                            &tensor_key
                        );
                    },
                    Some(_) => {},
                    ref mut other => {
                        *other = Some(model_tensor);
                    },
                }

                tensor_indices.insert(tensor_id);
                //FIXME: 2 tensors share same name (/empty name)
                //then tensor_indices.len() != tensor_constraints.len()
                tensor_constraints.insert(tensor_name, tensor_constraint);

                i += 1;
            }

            Ok((tensor_indices, tensor_constraints))
        };

        let (input_tensors, input_tensor_constraints) =
            allocate_tensors("input", &mut context.inputs(), &input_tensors)?;

        let (output_tensors, output_tensor_constraints) = allocate_tensors(
            "output",
            &mut context.outputs(),
            &output_tensors,
        )?;

        let graph_context = GraphContext {
            arguments: node_data
                .args
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
            input_tensors: input_tensor_constraints,
            output_tensors: output_tensor_constraints,
        };

        shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .insert(node_id.to_string(), graph_context);

        Ok(ModelNode {
            context,
            input_tensors,
            output_tensors,
            shared_state: shared_state.clone(),
        })
    }

    #[tracing::instrument(skip_all, level = "debug")]
    pub(crate) fn run(&mut self) -> Result<(), Error> {
        // We are recreating the input_tensors and output_tensors every time
        // before predict because wasm linear memory might have changed
        // the locations TODO: There's an optimization that can happen
        // here.. but just not yet
        let mut inputs: Vec<RuneCoralTensor> = Vec::new();
        let mut outputs: Vec<RuneCoralTensorMut> = Vec::new();
        let mut state = self.shared_state.lock().unwrap();

        state.tensors.iter_mut().enumerate().for_each(|(i, t)| {
            if self.input_tensors.contains(&i) {
                let pipeline_tensor = t.as_mut().unwrap();
                unsafe {
                    inputs.push(RuneCoralTensor {
                        element_type: get_runecoral_element_type(
                            &pipeline_tensor.element_type,
                        ),
                        shape: Cow::Borrowed(std::slice::from_raw_parts(
                            pipeline_tensor.dimensions.as_ptr() as *const i32,
                            pipeline_tensor.dimensions.len(),
                        )),
                        buffer: &pipeline_tensor.buffer,
                    })
                }
            } else if self.output_tensors.contains(&i) {
                let pipeline_tensor = t.as_mut().unwrap();
                unsafe {
                    outputs.push(RuneCoralTensorMut {
                        element_type: get_runecoral_element_type(
                            &pipeline_tensor.element_type,
                        ),
                        shape: Cow::Borrowed(std::slice::from_raw_parts(
                            pipeline_tensor.dimensions.as_ptr() as *const i32,
                            pipeline_tensor.dimensions.len(),
                        )),
                        buffer: &mut pipeline_tensor.buffer,
                    })
                }
            } else {
                // Do nothing
            }
        });

        self.context
            .infer(&inputs, &mut outputs)
            .map_err(Error::from)
    }
}

fn get_element_type(t: &RuneCoralTensorDescriptor) -> ElementType {
    match t.element_type {
        RuneCoralElementType::UInt8 => ElementType::U8,
        RuneCoralElementType::Int8 => ElementType::I8,
        RuneCoralElementType::Int16 => ElementType::I16,
        RuneCoralElementType::Int32 => ElementType::I32,
        RuneCoralElementType::Float32 => ElementType::F32,
        RuneCoralElementType::Int64 => ElementType::I64,
        RuneCoralElementType::Float64 => ElementType::F64,
        RuneCoralElementType::String => ElementType::Utf8,
        // TODO: Implement support for all the element types
        _ => ElementType::U8,
    }
}

fn get_runecoral_element_type(t: &ElementType) -> RuneCoralElementType {
    match t {
        ElementType::U8 => RuneCoralElementType::UInt8,
        ElementType::I8 => RuneCoralElementType::Int8,
        ElementType::I16 => RuneCoralElementType::Int16,
        ElementType::I32 => RuneCoralElementType::Int32,
        ElementType::F32 => RuneCoralElementType::Float32,
        ElementType::I64 => RuneCoralElementType::Int64,
        ElementType::F64 => RuneCoralElementType::Float64,
        ElementType::Utf8 => RuneCoralElementType::String,
        // TODO: Implement support for all the element types
        _ => RuneCoralElementType::NoType,
    }
}
