use std::{
    convert::TryInto,
    fmt::{self, Display, Formatter},
    sync::{Arc, Mutex},
    io::{Cursor, Read},
    collections::HashMap,
};

use zip;
use anyhow::{Context, Error, anyhow};
use hotg_rune_core::{ElementType as RuneElementType, Shape, TFLITE_MIMETYPE};
use indexmap::IndexMap;
use wasmer::{
    Array, Function, Instance, LazyInit, Memory, Module, NativeFunc,
    RuntimeError, Store, ValueType, WasmPtr, WasmerEnv,
};

use hotg_runecoral::{
    AccelerationBackend, ElementType as RuneCoralElementType, InferenceContext, Tensor,
    TensorDescriptor, TensorMut,
};

use hotg_rune_compiler::{
    parse::yaml::*,
    diagnostics::Diagnostics
};

use crate::{
    callbacks::Callbacks,
    engine::{host_functions::HostFunctions, LoadError, WebAssemblyEngine},
};

use self::{proc_block_v1::ProcBlockV1, runtime_v1::*};

wit_bindgen_wasmer::export!("../../wit-files/rune/runtime-v1.wit");
wit_bindgen_wasmer::import!("../../wit-files/rune/proc-block-v1.wit");

#[derive(Debug, Default, Clone, wasmer::WasmerEnv)]
struct Runtime(Arc<Mutex<State>>);

#[derive(Debug, Default)]
pub struct State {
    pub arguments: HashMap<String, String>,
    pub inputs: HashMap<String, TensorResult>,
    pub outputs: HashMap<String, TensorResult>,
}

pub struct ModelContext {
    inference_context: InferenceContext
}

pub struct ProcBlockContext {
    instance: Instance
}

pub struct ZuneEngine {
    tensors: Vec<Option<TensorResult>>,
    inputs: Vec<String>,
    input_tensors: HashMap<String, usize>,
    output_tensors: HashMap<String, usize>,
    outputs: Vec<String>,
    model_contexts: HashMap<String, ModelContext>,
    procblock_contexts: HashMap<String, ProcBlockContext>,
    pipeline: IndexMap<String, Stage>,
    processing_order: Vec<String>
    // resources
}

impl WebAssemblyEngine for ZuneEngine {
    fn load(
        binary: &[u8],
        _: Arc<dyn Callbacks>,
    ) -> Result<Self, LoadError>
    where
        Self: Sized,
    {
        let mut archive = zip::ZipArchive::new(Cursor::new(binary)).context("Unable to load Zune")?;

        let mut read_zip_resource_by_path = |path: &str| -> Result<Vec<u8>, LoadError> {
            let mut requested_file = archive.by_name(path).context(anyhow!("Unable to find {} in zune", path))?;
            let mut buffer = Vec::new();
            requested_file.read_to_end(&mut buffer).context(anyhow!("Unable to read {} from zune", path))?;
            Ok(buffer)
        };

        let runefile = String::from_utf8(read_zip_resource_by_path("Runefile.yml")?).context("Unable to read Runefile")?;
        let parsed_runefile = Document::parse(&runefile).context("Unable to parse Runefile")?;
        let pipeline = &parsed_runefile.to_v1().pipeline;

        let mut model_contexts: HashMap<String, ModelContext> = HashMap::new();
        let mut procblock_contexts: HashMap<String, ProcBlockContext> = HashMap::new();
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        for item in pipeline {
            // Collect each output tensor into tensors
            let stage_name = item.0;
            match item.1 {
                Stage::Capability(_) => {
                    inputs.push(stage_name.to_string());
                },
                Stage::Model(stage) => {
                    // Instantiating the model's inference context here because that way model_data gets deallocated once we are done with it
                    // This way memory usage is under control
                    let model_data = read_zip_resource_by_path(&stage.model.to_string())
                        .context(format!("Unable to read model from zune {}", stage.model))?;
                    let inference_context =
                        InferenceContext::create_context(TFLITE_MIMETYPE, &model_data, AccelerationBackend::NONE)
                            .context(format!("Error Instantiating model from zune {}", stage.model))?;

                    model_contexts.insert(stage_name.to_string(), ModelContext { inference_context });
                },
                Stage::ProcBlock(stage) => {
                    println!("Pipeline stage: {} proc block {:?} {}", stage_name, stage.args, stage.proc_block.base );
                    let wasm = read_zip_resource_by_path(&stage.proc_block.base).context("Unable to load the proc_block")?;
                },
                Stage::Out(_) => {
                    outputs.push(stage_name.to_string());
                }
            }
        }

        let (tensors, input_tensors, output_tensors, processing_order)
            = get_tensors(&inputs, &outputs, &pipeline).context(anyhow!("Unable to map out input/output tensors"))?;

        /*
        println!("input_tensors: {:?}", &input_tensors);
        println!("output_tensors: {:?}", &output_tensors);
        println!("processing_order: {:?}", &processing_order);
        */

        Ok(ZuneEngine {
            tensors,
            inputs,
            input_tensors,
            outputs,
            output_tensors,
            model_contexts,
            procblock_contexts,
            pipeline: pipeline.to_owned(),
            processing_order
        })
    }

    fn init(&mut self) -> Result<(), Error> {
        //TODO: Call each proc block's graph() and each model's inputs/outputs to instantiate the tensors

        Ok(())
    }

    fn predict(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

fn get_tensors(inputs: &Vec<String>, outputs: &Vec<String>, pipeline: &IndexMap<String, Stage>)
    -> Result<(Vec<Option<TensorResult>>, HashMap<String, usize>, HashMap<String, usize>, Vec<String>), Error> {
    let mut nodes_to_visit = outputs.clone();
        let mut nodes_visited = Vec::new();
        let mut tensors: Vec<Option<TensorResult>> = Vec::new();
        let mut output_tensors: HashMap<String, usize> = HashMap::new();
        let mut input_tensors: HashMap<String, usize> = HashMap::new();

        let key = |node_name: &str, tensor_index: Option<usize>| format!("{}.{}", node_name, tensor_index.or(Some(0)).unwrap());

        // For Inputs/Capabilities - input tensors and output tensors are the same?
        for item in inputs {
            tensors.push(None);
            input_tensors.insert(key(item, Some(0)), tensors.len() - 1);
            output_tensors.insert(key(item, Some(0)), tensors.len() - 1);
        }

        // Do a depth first traversal of the tree structure to determine the order of processing/calling predict()
        // Also allocate the output tensors of each node along the way
        while !nodes_to_visit.is_empty() {
            let node = nodes_to_visit.pop().unwrap();
            nodes_visited.push(node.clone());

            let stage = pipeline.get(&node).unwrap();
            for output_index in 0..stage.output_types().len() {
                tensors.push(None);
                output_tensors.insert(key(&node, Some(output_index)), tensors.len() - 1);
            }

            for input in stage.inputs() {
                if !nodes_visited.contains(&input.name) {
                    nodes_to_visit.push(input.name.clone());
                }
            }
        }

        // For each stage in the pipeline, since the inputs have to come from the outputs of other stages, simply map to the same tensor
        for item in pipeline {
            // Collect each output tensor into tensors
            let stage_name = item.0;
            for i in 0..item.1.inputs().len() {
                let input = &item.1.inputs()[i];
                let input_key = key(&input.name, input.index);
                let &input_tensor_index = output_tensors.get(&input_key).context(anyhow!("Invalid input key specified: {}", &input_key))?;
                input_tensors.insert(key(stage_name, Some(i)), input_tensor_index);
            }
        }

        nodes_visited.reverse();

        Ok((tensors, input_tensors, output_tensors, nodes_visited))
}

/*
#[derive(Debug, Clone)]
pub enum Never {}

impl runtime_v1::RuntimeV1 for ZuneEngine {
    type ArgumentHint = Never;
    type ArgumentMetadata = Never;
    type GraphContext = Never;
    type KernelContext = Arc<Mutex<State>>;
    type Metadata = Never;
    type TensorHint = Never;
    type TensorMetadata = Never;
    type LogMetadata = Never;

    fn metadata_new(&mut self, _name: &str, _version: &str) -> Self::Metadata {
        todo!()
    }
    fn metadata_set_description(&mut self, _self_: &Self::Metadata, _description: &str) {
        todo!()
    }
    fn metadata_set_repository(&mut self, _self_: &Self::Metadata, _url: &str) {
        todo!()
    }
    fn metadata_set_homepage(&mut self, _self_: &Self::Metadata, _url: &str) {
        todo!()
    }
    fn metadata_add_tag(&mut self, _self_: &Self::Metadata, _tag: &str) {
        todo!()
    }
    fn metadata_add_argument(&mut self, _self_: &Self::Metadata, _arg: &Self::ArgumentMetadata) {
        todo!()
    }
    fn metadata_add_input(&mut self, _self_: &Self::Metadata, _metadata: &Self::TensorMetadata) {
        todo!()
    }
    fn metadata_add_output(&mut self, _self_: &Self::Metadata, _metadata: &Self::TensorMetadata) {
        todo!()
    }
    fn argument_metadata_new(&mut self, _name: &str) -> Self::ArgumentMetadata {
        todo!()
    }
    fn argument_metadata_set_description(
        &mut self,
        _self_: &Self::ArgumentMetadata,
        _description: &str,
    ) {
        todo!()
    }
    fn argument_metadata_set_default_value(
        &mut self,
        _self_: &Self::ArgumentMetadata,
        _default_value: &str,
    ) {
        todo!()
    }
    fn argument_metadata_add_hint(
        &mut self,
        _self_: &Self::ArgumentMetadata,
        _hint: &Self::ArgumentHint,
    ) {
        todo!()
    }
    fn tensor_metadata_new(&mut self, _name: &str) -> Self::TensorMetadata {
        todo!()
    }
    fn tensor_metadata_set_description(
        &mut self,
        _self_: &Self::TensorMetadata,
        _description: &str,
    ) {
        todo!()
    }
    fn tensor_metadata_add_hint(
        &mut self,
        _self_: &Self::TensorMetadata,
        _hint: &Self::TensorHint,
    ) {
        todo!()
    }
    fn interpret_as_image(&mut self) -> Self::TensorHint {
        todo!()
    }
    fn interpret_as_audio(&mut self) -> Self::TensorHint {
        todo!()
    }
    fn supported_shapes(
        &mut self,
        _supported_element_types: Vec<ElementType>,
        _dimensions: Dimensions<'_>,
    ) -> Self::TensorHint {
        todo!()
    }
    fn interpret_as_number_in_range(&mut self, _min: &str, _max: &str) -> Self::ArgumentHint {
        todo!()
    }
    fn interpret_as_string_in_enum(&mut self, _string_enum: Vec<&str>) -> Self::ArgumentHint {
        todo!()
    }
    fn non_negative_number(&mut self) -> Self::ArgumentHint {
        todo!()
    }
    fn supported_argument_type(&mut self, _hint: ArgumentType) -> Self::ArgumentHint {
        todo!()
    }
    fn register_node(&mut self, _metadata: &Self::Metadata) {
        todo!()
    }
    fn graph_context_for_node(&mut self, _node_id: &str) -> Option<Self::GraphContext> {
        todo!()
    }
    fn graph_context_get_argument(
        &mut self,
        _self_: &Self::GraphContext,
        _name: &str,
    ) -> Option<String> {
        todo!()
    }
    fn graph_context_add_input_tensor(
        &mut self,
        _self_: &Self::GraphContext,
        _name: &str,
        _element_type: ElementType,
        _dimensions: Dimensions<'_>,
    ) {
        todo!()
    }
    fn graph_context_add_output_tensor(
        &mut self,
        _self_: &Self::GraphContext,
        _name: &str,
        _element_type: ElementType,
        _dimensions: Dimensions<'_>,
    ) {
        todo!()
    }
    fn kernel_context_for_node(&mut self, _node_id: &str) -> Option<Self::KernelContext> {
        Some(self.0.clone())
    }
    fn kernel_context_get_argument(
        &mut self,
        state: &Arc<Mutex<State>>,
        name: &str,
    ) -> Option<String> {
        state.lock().unwrap().arguments.get(name).cloned()
    }
    fn kernel_context_get_input_tensor(
        &mut self,
        state: &Arc<Mutex<State>>,
        name: &str,
    ) -> Option<TensorResult> {
        state.lock().unwrap().inputs.get(name).cloned()
    }
    fn kernel_context_set_output_tensor(
        &mut self,
        state: &Arc<Mutex<State>>,
        name: &str,
        TensorParam {
            element_type,
            buffer,
            dimensions,
        }: TensorParam<'_>,
    ) {
        let tensor = TensorResult {
            element_type,
            dimensions: dimensions.iter().map(|d| d.get()).collect(),
            buffer: buffer.to_vec(),
        };
        state
            .lock()
            .unwrap()
            .outputs
            .insert(name.to_string(), tensor);
    }

    fn is_enabled(&mut self, _metadata: LogMetadata) -> bool {
        true
    }

    fn log(&mut self, metadata: LogMetadata, message: &str, data: Vec<(&'_ str, LogValue<'_>)>) {
        let level = match metadata.level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error | LogLevel::Fatal => tracing::Level::ERROR,
        };

        let LogMetadata {
            name,
            target,
            level: _,
            file,
            line,
            module,
        } = metadata;

        tracing::event!(
            tracing::Level::INFO,
            meta.level = %level,
            meta.name = %name,
            meta.target = target,
            meta.file = file,
            meta.line = line,
            meta.module = module,
            ?data,
            message,
        );
    }
}
*/