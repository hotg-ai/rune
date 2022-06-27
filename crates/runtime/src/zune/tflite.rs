use std::{borrow::Cow, collections::HashMap};

use anyhow::{anyhow, Context, Error};
use hotg_rune_core::TFLITE_MIMETYPE;
use hotg_runecoral::{
    AccelerationBackend, ElementType as RuneCoralElementType, InferenceContext,
    Tensor as RuneCoralTensor, TensorDescriptor as RuneCoralTensorDescriptor,
    TensorMut as RuneCoralTensorMut,
};
use indexmap::IndexMap;

use crate::zune::{
    DimensionsConstraint, ElementType, ElementTypeConstraint, GraphNode,
    Tensor, TensorConstraint, TensorConstraints,
};

pub(crate) struct ModelNode {
    node_id: String,
    context: InferenceContext,
    tensor_constraints: TensorConstraints,
}

impl GraphNode for ModelNode {
    #[tracing::instrument(skip(node_data), level = "debug")]
    fn load(
        node_id: &str,
        args: &HashMap<String, String>,
        node_data: &[u8],
    ) -> Result<Box<dyn GraphNode>, Error>
    where
        Self: Sized,
    {
        // Create Inference Context
        let context = InferenceContext::create_context(
            TFLITE_MIMETYPE,
            &node_data,
            AccelerationBackend::NONE,
        )
        .with_context(|| {
            format!(
                "Error Instantiating model from zune for stage: {}",
                &node_id
            )
        })?;

        let get_tensor_constraints =
            |model_tensors: &mut dyn Iterator<
                Item = RuneCoralTensorDescriptor,
            >|
             -> IndexMap<String, TensorConstraint> {
                let mut result = IndexMap::new();
                let mut i = 0;

                while let Some(model_tensor) = model_tensors.next() {
                    // Not all tensors in the tensorflow models might have names.
                    // In such cases, we give them our own names based on their index
                    let tensor_name = model_tensor.name.to_str().ok();
                    let tensor_name = match tensor_name {
                        Some(tensor_name) if tensor_name.len() > 0 => {
                            tensor_name.to_string()
                        },
                        _ => i.to_string(),
                    };
                    result.insert(
                        tensor_name,
                        TensorConstraint {
                            element_types: get_element_type_constraint(
                                &model_tensor.element_type,
                            ),
                            dimensions: DimensionsConstraint::Fixed(
                                model_tensor
                                    .shape
                                    .iter()
                                    .map(|&x| x as u32)
                                    .collect(),
                            ),
                        },
                    );
                    i += 1;
                }

                result
            };

        let tensor_constraints = TensorConstraints {
            inputs: get_tensor_constraints(&mut context.inputs()),
            outputs: get_tensor_constraints(&mut context.outputs()),
        };

        Ok(Box::new(ModelNode {
            node_id: node_id.to_string(),
            context,
            tensor_constraints,
        }))
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn node_id(&self) -> &str {
        return &self.node_id;
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn tensor_constraints(&self) -> &TensorConstraints {
        return &self.tensor_constraints;
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn run(
        &mut self,
        inputs: HashMap<&str, &Tensor>,
    ) -> Result<HashMap<&str, Tensor>, Error> {
        let runecoral_inputs: Result<Vec<RuneCoralTensor>, _> = self
            .tensor_constraints
            .inputs
            .iter()
            .map(|(name, _)| -> Result<RuneCoralTensor, anyhow::Error> {
                let tensor = *inputs.get(name.as_str()).ok_or_else(|| {
                    anyhow!("Unable to find input tensor: {}", name)
                })?;
                unsafe {
                    Ok(RuneCoralTensor {
                        element_type: get_runecoral_element_type(
                            &tensor.element_type,
                        ),
                        shape: Cow::Borrowed(std::slice::from_raw_parts(
                            tensor.dimensions.as_ptr() as *const i32,
                            tensor.dimensions.len(),
                        )),
                        buffer: &tensor.buffer,
                    })
                }
            })
            .collect();
        let runecoral_inputs = runecoral_inputs?;
        let mut runecoral_outputs: Vec<RuneCoralTensorMut> = Vec::new();

        self.context
            .infer(&runecoral_inputs, &mut runecoral_outputs)
            .map_err(Error::from)?;

        let result: Result<HashMap<&str, Tensor>, Error> = self
            .tensor_constraints
            .outputs
            .iter()
            .enumerate()
            .map(
                |(index, (name, _))| -> Result<(&str, Tensor), anyhow::Error> {
                    let tensor =
                        runecoral_outputs.get(index).ok_or_else(|| {
                            anyhow!("Unable to find output tensor: {}", name)
                        })?;
                    Ok((
                        name,
                        Tensor {
                            element_type: get_element_type(
                                &tensor.element_type,
                            ),
                            dimensions: tensor
                                .shape
                                .iter()
                                .map(|&x| x as u32)
                                .collect(),
                            buffer: tensor.buffer.to_owned(),
                        },
                    ))
                },
            )
            .collect();

        result
    }
}

fn get_element_type_constraint(
    t: &RuneCoralElementType,
) -> ElementTypeConstraint {
    match t {
        RuneCoralElementType::UInt8 => ElementTypeConstraint::U8,
        RuneCoralElementType::Int8 => ElementTypeConstraint::I8,
        RuneCoralElementType::Int16 => ElementTypeConstraint::I16,
        RuneCoralElementType::Int32 => ElementTypeConstraint::I32,
        RuneCoralElementType::Float32 => ElementTypeConstraint::F32,
        RuneCoralElementType::Int64 => ElementTypeConstraint::I64,
        RuneCoralElementType::Float64 => ElementTypeConstraint::F64,
        RuneCoralElementType::Complex64 => ElementTypeConstraint::COMPLEX_64,
        RuneCoralElementType::Complex128 => ElementTypeConstraint::COMPLEX_128,
        RuneCoralElementType::String => ElementTypeConstraint::UTF8,
        // TODO: Implement support for all the element types
        _ => ElementTypeConstraint::U8,
    }
}

fn get_element_type(t: &RuneCoralElementType) -> ElementType {
    match t {
        RuneCoralElementType::UInt8 => ElementType::U8,
        RuneCoralElementType::Int8 => ElementType::I8,
        RuneCoralElementType::Int16 => ElementType::I16,
        RuneCoralElementType::Int32 => ElementType::I32,
        RuneCoralElementType::Float32 => ElementType::F32,
        RuneCoralElementType::Int64 => ElementType::I64,
        RuneCoralElementType::Float64 => ElementType::F64,
        RuneCoralElementType::Complex64 => ElementType::Complex64,
        RuneCoralElementType::Complex128 => ElementType::Complex128,
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
        ElementType::Complex64 => RuneCoralElementType::Complex64,
        ElementType::Complex128 => RuneCoralElementType::Complex128,
        ElementType::Utf8 => RuneCoralElementType::String,
        // TODO: Implement support for all the element types
        _ => RuneCoralElementType::NoType,
    }
}
