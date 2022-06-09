mod proc_block;
#[cfg(feature = "tflite")]
mod tflite;

use std::{
    fmt::{self, Display, Formatter},
    io::{Cursor, Read},
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Context, Error};
use hotg_rune_compiler::parse::yaml::*;
use indexmap::IndexMap;
use zip;

pub use self::{proc_block_v1::*, runtime_v1::*};
use crate::{
    zune::proc_block::{GraphContext, ProcBlockNode, TensorConstraint, Dimensions},
    LoadError,
};

wit_bindgen_wasmer::export!("../../wit-files/rune/runtime-v1.wit");
wit_bindgen_wasmer::import!("../../wit-files/rune/proc-block-v1.wit");

pub(crate) trait Node {
    fn run(&mut self) -> Result<(), Error>;
}

#[derive(Debug, Default, Clone, wasmer::WasmerEnv)]
pub(crate) struct Runtime {
    shared_state: Arc<Mutex<State>>,
}

#[derive(Debug, Default)]
pub(crate) struct State {
    pub(crate) tensors: Vec<Option<TensorResult>>,
    pub(crate) tensor_constraints: Vec<Option<TensorConstraint>>,
    pub(crate) graph_contexts: IndexMap<String, GraphContext>,
}

pub struct ZuneEngine {
    input_nodes: Vec<String>,
    output_nodes: Vec<String>,
    nodes: IndexMap<String, Box<dyn Node>>,
    processing_order: Vec<String>,
    shared_state: Arc<Mutex<State>>, // resources
}

impl ZuneEngine {
    #[tracing::instrument(skip_all)]
    pub fn load(binary: &[u8]) -> Result<Self, LoadError>
    where
        Self: Sized,
    {
        let mut archive = zip::ZipArchive::new(Cursor::new(binary))
            .context("Unable to load Zune")?;

        let mut read_zip_resource_by_path =
            |path: &str| -> Result<Vec<u8>, Error> {
                let mut requested_file =
                    archive.by_name(path).with_context(|| {
                        anyhow!("Unable to find {} in zune", path)
                    })?;
                let mut buffer = Vec::new();
                requested_file.read_to_end(&mut buffer).with_context(|| {
                    format!("Unable to read {} from zune", path)
                })?;
                Ok(buffer)
            };

        let runefile =
            String::from_utf8(read_zip_resource_by_path("Runefile.yml")?)
                .context("Unable to read Runefile")?;
        tracing::debug!(length = runefile.len(), "Read the Rune");

        let parsed_runefile =
            Document::parse(&runefile).context("Unable to parse Runefile")?;
        let pipeline = &parsed_runefile.to_v1().pipeline;

        let inputs: Vec<_> = pipeline
            .iter()
            .filter_map(|(k, v)| match v {
                Stage::Capability(_) => Some(k.clone()),
                _ => None,
            })
            .collect();

        let outputs: Vec<_> = pipeline
            .iter()
            .filter_map(|(k, v)| match v {
                Stage::Out(_) => Some(k.clone()),
                _ => None,
            })
            .collect();

        let (tensors, input_tensors, output_tensors, processing_order) =
            get_tensors(&inputs, &outputs, &pipeline)
                .context("Unable to map out input/output tensors")?;

        let graph_contexts = pipeline
            .iter()
            .map(|(k, v)| {
                let arguments = v
                    .args()
                    .iter()
                    .map(|(name, argument)| {
                        (name.clone(), argument.to_string())
                    })
                    .collect();
                (
                    k.clone(),
                    GraphContext {
                        arguments,
                        input_tensors: IndexMap::new(),
                        output_tensors: IndexMap::new(),
                    },
                )
            })
            .collect();

        let tensor_constraints = tensors.iter().map(|_| None).collect();
        let shared_state = Arc::new(Mutex::new(State {
            tensors,
            tensor_constraints,
            graph_contexts,
        }));

        tracing::trace!(?input_tensors, ?output_tensors, "Loaded tensors");

        let nodes = instantiate_nodes(
            pipeline,
            read_zip_resource_by_path,
            &shared_state,
            input_tensors,
            output_tensors,
        )
        .map_err(LoadError::Other)?;

        tracing::debug!(order=?processing_order, "Determined the execution order");

        // TODO: Validate and allocate input/output tensors

        Ok(ZuneEngine {
            input_nodes: inputs,
            output_nodes: outputs,
            nodes,
            processing_order,
            shared_state,
        })
    }

    #[tracing::instrument(skip_all)]
    pub fn predict(&mut self) -> Result<(), Error> {
        for stage_name in &self.processing_order {
            let _span =
                tracing::debug_span!("Running Stage", %stage_name).entered();

            if let Some(node) = self.nodes.get_mut(stage_name) {
                node.run()?;
            }
        }
        Ok(())
    }

    pub fn input_nodes(&self) -> &[String] {
        return &self.input_nodes;
    }

    pub fn output_nodes(&self) -> &[String] {
        return &self.output_nodes;
    }

    pub fn get_input_tensor_names(
        &self,
        node_name: &str,
    ) -> Result<Vec<String>, Error> {
        let state = self.shared_state.lock().unwrap();
        state
            .graph_contexts
            .get(node_name)
            .and_then(|c| {
                let tensor_list: Vec<String> = c
                    .input_tensors
                    .iter()
                    .map(|(k, _)| k.to_string())
                    .collect();
                Some(tensor_list)
            })
            .context("Unable to get input tensors")
    }

    pub fn get_input_tensor(
        &self,
        node_name: &str,
        tensor_name: &str,
    ) -> Option<TensorResult> {
        let state = self.shared_state.lock().unwrap();
        let tensor_constraint = state
            .graph_contexts
            .get(node_name)
            .and_then(|c| c.input_tensors.get(tensor_name));

        match tensor_constraint {
            Some(c) if c.tensor_id.is_some() => {
                state.tensors[c.tensor_id.unwrap()].clone()
            },
            _ => None,
        }
    }

    pub fn set_input_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
        tensor: &TensorResult,
    ) {
        let mut state = self.shared_state.lock().unwrap();
        let tensor_id = state.graph_contexts.get(node_name).and_then(|c| {
            c.input_tensors
                .get(tensor_name)
                .and_then(|c| c.tensor_id.clone())
        });

        match tensor_id {
            Some(i) => state.tensors[i] = Some(tensor.clone()),
            _ => {},
        }
    }

    pub fn get_output_tensor_names(
        &self,
        node_name: &str,
    ) -> Result<Vec<String>, Error> {
        let state = self.shared_state.lock().unwrap();
        state
            .graph_contexts
            .get(node_name)
            .and_then(|c| {
                let tensor_list: Vec<String> = c
                    .output_tensors
                    .iter()
                    .map(|(k, _)| k.to_string())
                    .collect();
                Some(tensor_list)
            })
            .context("Unable to get input tensors")
    }

    pub fn get_output_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
    ) -> Option<TensorResult> {
        let state = self.shared_state.lock().unwrap();
        let tensor_constraint = state
            .graph_contexts
            .get(node_name)
            .and_then(|c| c.output_tensors.get(tensor_name));

        match tensor_constraint {
            Some(c) if c.tensor_id.is_some() => {
                state.tensors[c.tensor_id.unwrap()].clone()
            },
            _ => None,
        }
    }

    // pub fn get_tensor(&self, tensor_id: usize) -> Option<&TensorResult> {
    //     self.shared_state
    //         .lock()
    //         .unwrap()
    //         .tensors
    //         .get(tensor_id)
    //         .unwrap_or(&None)
    //         .as_ref()
    // }

    // pub fn set_tensor(&mut self, tensor_id: usize, tensor: &TensorResult) -> Result<(), Error> {
    //     self.shared_state
    //         .lock()
    //         .unwrap()
    //         .tensors
    //         .get_mut(tensor_id)
    //         .and_then(|t| { t = Some(tensor.clone()); Ok() })
    //         .ok()
    // }

    pub fn set_output_tensor(
        &mut self,
        node_name: &str,
        tensor_name: &str,
        tensor: &TensorResult,
    ) {
        let mut state = self.shared_state.lock().unwrap();
        let tensor_id = state.graph_contexts.get(node_name).and_then(|c| {
            c.output_tensors
                .get(tensor_name)
                .and_then(|c| c.tensor_id.clone())
        });

        match tensor_id {
            Some(i) => state.tensors[i] = Some(tensor.clone()),
            _ => {},
        }
    }
}

fn get_buffer_size(element_type: ElementType, dimensions: &Vec<u32>) -> usize {
    (dimensions.iter().fold(1, |a, &b| a * b)
        * get_bytes_per_element(element_type)) as usize
}

fn get_bytes_per_element(element_type: ElementType) -> u32 {
    match element_type {
        ElementType::I16 => 2,
        ElementType::I32 | ElementType::F32 => 4,
        ElementType::I64 | ElementType::F64 => 8,
        _ => 1,
    }
}

fn instantiate_nodes(
    pipeline: &IndexMap<String, Stage>,
    mut read_zip_resource_by_path: impl FnMut(&str) -> Result<Vec<u8>, Error>,
    shared_state: &Arc<Mutex<State>>,
    input_tensors: IndexMap<String, usize>,
    output_tensors: IndexMap<String, usize>,
) -> Result<IndexMap<String, Box<dyn Node>>, Error> {
    let mut nodes: IndexMap<String, Box<dyn Node>> = IndexMap::new();

    let runtime = Runtime {
        shared_state: shared_state.clone(),
    };

    for (stage_name, stage) in pipeline {
        // Collect each output tensor into tensors
        match stage {
            // Models are handled on the host side, so we treat them separately
            Stage::Capability(stage) => {
                let wasm =
                    read_zip_resource_by_path(&stage.capability.to_string())
                        .context("Unable to load the capability")?;

                let pb = ProcBlockNode::load(
                    &stage_name,
                    &wasm,
                    &runtime,
                    &input_tensors,
                    &output_tensors,
                )?;
                nodes.insert(stage_name.to_string(), Box::new(pb));
            },
            Stage::Model(stage) => {
                // Instantiating the model's inference context here because that
                // way model_data gets deallocated once we are done with it
                // This way memory usage is under control
                let model_data =
                    read_zip_resource_by_path(&stage.model.to_string())
                        .with_context(|| {
                            format!(
                                "Unable to read model from zune {}",
                                stage.model
                            )
                        })?;

                let model_format =
                    stage.args.get("model-format").map(|f| f.to_string());
                let node = load_model(
                    &model_data,
                    model_format.as_deref().unwrap_or("tflite"),
                    stage_name,
                    stage,
                    shared_state,
                    &input_tensors,
                    &output_tensors,
                )?;
                nodes.insert(stage_name.to_string(), node);
            },
            Stage::ProcBlock(stage) => {
                let wasm =
                    read_zip_resource_by_path(&stage.proc_block.to_string())
                        .context("Unable to load the proc_block")?;

                let pb = ProcBlockNode::load(
                    &stage_name,
                    &wasm,
                    &runtime,
                    &input_tensors,
                    &output_tensors,
                )?;
                nodes.insert(stage_name.to_string(), Box::new(pb));
            },
            Stage::Out(stage) => {
                shared_state
                .lock()
                .unwrap()
                .graph_contexts
                .get_mut(stage_name)
                .and_then(|c| {
                    for input in stage.inputs.iter() {
                        let tensor_key = key(&input.name, input.index);
                        let tensor_id = output_tensors.get(&tensor_key).copied();
                        c.input_tensors.insert(tensor_key,
                            TensorConstraint {
                                tensor_id,
                                element_type: ElementType::U8,
                                dimensions: Dimensions::Dynamic
                            }
                        );
                    }
                    Some(())
                });
            }, // Do nothing for capabilities/outputs
        }
    }

    Ok(nodes)
}

fn load_model(
    model_data: &[u8],
    model_format: &str,
    stage_name: &str,
    stage: &ModelStage,
    shared_state: &Arc<Mutex<State>>,
    input_tensors: &IndexMap<String, usize>,
    output_tensors: &IndexMap<String, usize>,
) -> Result<Box<dyn Node>, Error> {
    match model_format {
        #[cfg(feature = "tflite")]
        "tflite" | hotg_rune_core::TFLITE_MIMETYPE => {
            let model = tflite::ModelNode::load(
                stage_name,
                stage,
                model_data,
                shared_state,
                input_tensors,
                output_tensors,
            )?;

            Ok(Box::new(model))
        },
        other => anyhow::bail!("Unsupported model format, \"{}\"", other),
    }
}

fn get_tensors(
    inputs: &Vec<String>,
    outputs: &Vec<String>,
    pipeline: &IndexMap<String, Stage>,
) -> Result<
    (
        Vec<Option<TensorResult>>,
        IndexMap<String, usize>,
        IndexMap<String, usize>,
        Vec<String>,
    ),
    Error,
> {
    let mut nodes_to_visit = outputs.clone();
    let mut nodes_visited = Vec::new();
    let mut tensors: Vec<Option<TensorResult>> = Vec::new();
    let mut output_tensors: IndexMap<String, usize> = IndexMap::new();
    let mut input_tensors: IndexMap<String, usize> = IndexMap::new();

    // For Inputs/Capabilities - We create an input so as to be able to inject inputs
    for item in inputs {
        tensors.push(None);
        input_tensors.insert(key(item, Some(0)), tensors.len() - 1);
        output_tensors.insert(key(item, Some(0)), tensors.len() - 1);
    }

    // // For Outputs - we allocate all the outputs
    // for item in outputs {
    //     for _ in pipeline.get(item).unwrap().output_types() {
    //         tensors.push(None);
    //         output_tensors.insert(key(item, Some(0)), tensors.len() - 1);
    //     }
    // }

    // Do a depth first traversal of the tree structure to determine the order
    // of processing/calling predict() Also allocate the output tensors of
    // each node along the way
    while !nodes_to_visit.is_empty() {
        let node = nodes_to_visit.pop().unwrap();
        nodes_visited.push(node.clone());

        let stage = pipeline.get(&node).unwrap();
        for output_index in 0..stage.output_types().len() {
            tensors.push(None);
            output_tensors
                .insert(key(&node, Some(output_index)), tensors.len() - 1);
        }

        for input in stage.inputs() {
            if !nodes_to_visit.contains(&input.name)
                && !nodes_visited.contains(&input.name)
            {
                nodes_to_visit.push(input.name.clone());
            }
        }
    }

    // For each stage in the pipeline, since the inputs have to come from the
    // outputs of other stages, simply map to the same tensor
    for item in pipeline {
        // Collect each output tensor into tensors
        let stage_name = item.0;
        for i in 0..item.1.inputs().len() {
            let input = &item.1.inputs()[i];
            let input_key = key(&input.name, input.index);
            let &input_tensor_index =
                output_tensors.get(&input_key).with_context(|| {
                    format!("Invalid input key specified: {}", &input_key)
                })?;
            input_tensors.insert(key(stage_name, Some(i)), input_tensor_index);
        }
    }

    nodes_visited.reverse();

    Ok((tensors, input_tensors, output_tensors, nodes_visited))
}

fn key(node_name: &str, tensor_index: Option<usize>) -> String {
    format!("{}.{}", node_name, tensor_index.or(Some(0)).unwrap())
}

impl std::error::Error for proc_block_v1::GraphError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GraphError::InvalidArgument(InvalidArgument { reason, .. }) => {
                Some(reason)
            },
            _ => None,
        }
    }
}

impl Display for proc_block_v1::GraphError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GraphError::Other(msg) => Display::fmt(msg, f),
            GraphError::InvalidArgument(InvalidArgument { name, .. }) => {
                write!(f, "The \"{}\" argument is invalid", name)
            },
            GraphError::MissingContext => {
                write!(f, "Unable to retrieve the graph context")
            },
        }
    }
}

impl std::error::Error for proc_block_v1::KernelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            KernelError::InvalidArgument(InvalidArgument {
                reason, ..
            }) => Some(reason),
            KernelError::InvalidInput(InvalidInput { reason, .. }) => {
                Some(reason)
            },
            _ => None,
        }
    }
}

impl Display for proc_block_v1::KernelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::Other(s) => Display::fmt(s, f),
            KernelError::InvalidArgument(InvalidArgument { name, .. }) => {
                write!(f, "The \"{}\" argument is invalid", name)
            },
            KernelError::InvalidInput(InvalidInput { name, .. }) => {
                write!(f, "The \"{}\" input is invalid", name)
            },
            KernelError::MissingContext => {
                write!(f, "Unable to retrieve the kernel context")
            },
        }
    }
}

impl std::error::Error for BadArgumentReason {}

impl Display for BadArgumentReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BadArgumentReason::Other(msg) => Display::fmt(msg, f),
            BadArgumentReason::NotFound => write!(f, "Argument not found"),
            BadArgumentReason::InvalidValue(msg) => {
                write!(f, "Invalid argument value: {}", msg)
            },
        }
    }
}

impl std::error::Error for BadInputReason {}

impl Display for BadInputReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BadInputReason::Other(msg) => Display::fmt(msg, f),
            BadInputReason::NotFound => write!(f, "Input not found"),
            BadInputReason::UnsupportedShape => {
                write!(f, "Unsupported tensor shape")
            },
            BadInputReason::InvalidValue(msg) => {
                write!(f, "Invalid argument value: {}", msg)
            },
        }
    }
}
