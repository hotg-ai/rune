use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    convert::TryInto,
    fmt::{self, Display, Formatter},
    io::{Cursor, Read},
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Context, Error};
use hotg_rune_compiler::{diagnostics::Diagnostics, parse::yaml::*};
use hotg_rune_core::{ElementType as RuneElementType, Shape, TFLITE_MIMETYPE};
use hotg_runecoral::{
    AccelerationBackend, ElementType as RuneCoralElementType, InferenceContext,
    Tensor as RuneCoralTensor, TensorDescriptor as RuneCoralTensorDescriptor,
    TensorMut as RuneCoralTensorMut,
};
use indexmap::IndexMap;
use wasmer::{
    Array, Function, ImportObject, Instance, LazyInit, Memory, Module,
    NativeFunc, RuntimeError, Store, ValueType, WasmPtr, WasmerEnv,
};
use zip;

use self::{proc_block_v1::ProcBlockV1, runtime_v1::*};
use crate::{
    callbacks::Callbacks,
    engine::{host_functions::HostFunctions, LoadError, WebAssemblyEngine},
};

wit_bindgen_wasmer::export!("../../wit-files/rune/runtime-v1.wit");
wit_bindgen_wasmer::import!("../../wit-files/rune/proc-block-v1.wit");

#[derive(Debug, Default, Clone, wasmer::WasmerEnv)]
struct Runtime {
    shared_state: Arc<Mutex<State>>,
}

#[derive(Debug, Default)]
struct State {
    tensors: Vec<Option<TensorResult>>,
    graph_contexts: HashMap<String, GraphContext>,
    tensor_constraints: Vec<Option<TensorResult>>
}

struct ModelNode {
    context: InferenceContext,
    input_tensors: HashSet<usize>,
    output_tensors: HashSet<usize>,
    shared_state: Arc<Mutex<State>>,
}

struct ProcBlockNode {
    input_tensors: HashMap<String, usize>,
    output_tensors: HashMap<String, usize>,
    context: ProcBlockV1,
    shared_state: Arc<Mutex<State>>
}

pub struct ZuneEngine {
    inputs: Vec<String>,
    outputs: Vec<String>,
    models: HashMap<String, ModelNode>,
    procblocks: HashMap<String, ProcBlockNode>,
    pipeline: IndexMap<String, Stage>,
    processing_order: Vec<String>,
    shared_state: Arc<Mutex<State>>, // resources
}

impl WebAssemblyEngine for ZuneEngine {
    fn load(binary: &[u8], _: Arc<dyn Callbacks>) -> Result<Self, LoadError>
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
                    anyhow!("Unable to read {} from zune", path)
                })?;
                Ok(buffer)
            };

        let runefile =
            String::from_utf8(read_zip_resource_by_path("Runefile.yml")?)
                .context("Unable to read Runefile")?;
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
                .context(anyhow!("Unable to map out input/output tensors"))?;

        let graph_contexts = pipeline
            .iter()
            .map(|(k, v)| {

                let arguments =
                    v.args()
                    .iter()
                    .map(|(name, argument)| (name.clone(), argument.to_string()))
                    .collect();
                (k.clone(), GraphContext{ arguments, input_tensors: Vec::new(), output_tensors: Vec::new() })
            })
            .collect();

        let shared_state = Arc::new(Mutex::new(State { tensors, graph_contexts, tensor_constraints: Vec::new() }));

        let (model_contexts, procblock_contexts) = instantiate_nodes(
            pipeline,
            read_zip_resource_by_path,
            &shared_state,
            input_tensors,
            output_tensors,
        )
        .map_err(LoadError::Other)?;

        Ok(ZuneEngine {
            inputs,
            outputs,
            models: model_contexts,
            procblocks: procblock_contexts,
            pipeline: pipeline.to_owned(),
            processing_order,
            shared_state,
        })
    }

    fn init(&mut self) -> Result<(), Error> {
        // TODO: Call each proc block's graph() and each model's inputs/outputs
        // to allocate the tensors with correct dimensions

        Ok(())
    }

    fn predict(&mut self) -> Result<(), Error> {
        for stage_name in &self.processing_order {
            let stage = self.pipeline.get(stage_name).unwrap();
            match stage {
                Stage::Model(stage) => {
                    self.models.get_mut(stage_name).unwrap().run()?;
                },
                Stage::ProcBlock(stage) => {
                    self.procblocks.get_mut(stage_name).unwrap().run()?;
                },
                _ => {},
            }
        }
        Ok(())
    }
}

impl ModelNode {
    fn load(
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

        // Returns the list of tensor indices in the State's tensors
        let allocate_tensors = |tensor_type: &str,
                                model_tensors: &mut dyn Iterator<
            Item = RuneCoralTensorDescriptor,
        >,
                                pipeline_tensors: &HashMap<String, usize>|
         -> Result<HashSet<usize>, Error> {
            let mut result: HashSet<usize> = HashSet::new();
            let mut i = 0;
            let mut s = shared_state.lock().unwrap();

            while let Some(model_tensor) = model_tensors.next() {
                let model_tensor = tensor_from_descriptor(&model_tensor);
                let tensor_key = key(&node_id, Some(i));
                let tensor_id =
                    *pipeline_tensors.get(&tensor_key).ok_or_else(|| {
                        anyhow!(
                            "Unable to find pipeline_tensor for {} tensor \
                             with key {}",
                            &tensor_type,
                            &tensor_key
                        )
                    })?;

                match s.tensors[tensor_id] {
                    Some(ref t)
                        if t.dimensions != model_tensor.dimensions
                            || t.element_type != model_tensor.element_type =>
                    {
                        return Err(anyhow!(
                            "Pipeline tensor for {} with key {} doesn't match \
                             model tensor",
                            &tensor_type,
                            &tensor_key
                        ))
                    },
                    Some(_) => {},
                    ref mut other => {
                        other.insert(model_tensor);
                    },
                }

                result.insert(tensor_id);

                i += 1;
            }

            Ok(result)
        };

        let input_tensors =
            allocate_tensors("input", &mut context.inputs(), &input_tensors)?;
        let output_tensors = allocate_tensors(
            "output",
            &mut context.outputs(),
            &output_tensors,
        )?;

        Ok(ModelNode {
            context,
            input_tensors,
            output_tensors,
            shared_state: shared_state.clone(),
        })
    }

    fn run(&mut self) -> Result<(), Error> {
        // We are recreating the input_tensors and output_tensors every time
        // before predict because wasm linear memory might have changed
        // the locations TODO: There's an optimization that can happen
        // here.. but just not yet
        let mut inputs: Vec<RuneCoralTensor> = Vec::new();
        let mut outputs: Vec<RuneCoralTensorMut> = Vec::new();
        let mut state = self.shared_state.lock().unwrap();

        state.tensors.iter_mut().enumerate().for_each(|(i, t)| {
            if self.input_tensors.contains(&i) {
                let mut pipeline_tensor = t.as_mut().unwrap();
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
                let mut pipeline_tensor = t.as_mut().unwrap();
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
            .map_err(|e| anyhow!(e.to_string()))
    }
}

impl ProcBlockNode {
    fn load(
        node_id: &str,
        node_data: &ProcBlockStage,
        wasm: &[u8],
        store: &Store,
        mut imports: &mut ImportObject,
        shared_state: &Arc<Mutex<State>>,
        input_tensors: &HashMap<String, usize>,
        output_tensors: &HashMap<String, usize>,
    ) -> Result<ProcBlockNode, Error> {
        let module =
            Module::new(&store, wasm).context("Unable to load the module")?;

        let (pb, _) =
            ProcBlockV1::instantiate(&store, &module, &mut imports)
                .context("Unable to instantiate the WebAssembly module")?;

        let result = pb.graph(node_id);

        Ok(ProcBlockNode {
            input_tensors: HashMap::new(),
            output_tensors: HashMap::new(),
            context: pb,
            shared_state: shared_state.clone(),
        })
    }

    fn run(&mut self) -> Result<(), Error> { Ok(()) }
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

fn instantiate_nodes(
    pipeline: &IndexMap<String, Stage>,
    mut read_zip_resource_by_path: impl FnMut(&str) -> Result<Vec<u8>, Error>,
    shared_state: &Arc<Mutex<State>>,
    input_tensors: HashMap<String, usize>,
    output_tensors: HashMap<String, usize>,
) -> Result<(HashMap<String, ModelNode>, HashMap<String, ProcBlockNode>), Error>
{
    let mut models: HashMap<String, ModelNode> = HashMap::new();
    let mut procblocks: HashMap<String, ProcBlockNode> = HashMap::new();

    let store = Store::default();
    let mut imports = ImportObject::default();
    let mut runtime = Runtime{ shared_state: shared_state.clone() };
    add_to_imports(&store, &mut imports, runtime.clone());

    for item in pipeline {
        // Collect each output tensor into tensors
        let stage_name = item.0;
        match item.1 {
            Stage::Capability(_) => {
                // inputs.push(stage_name.to_string());
            },
            Stage::Model(stage) => {
                // Instantiating the model's inference context here because that
                // way model_data gets deallocated once we are done with it
                // This way memory usage is under control
                let model_data =
                    read_zip_resource_by_path(&stage.model.to_string())
                        .with_context(|| {
                            anyhow!(
                                "Unable to read model from zune {}",
                                stage.model
                            )
                        })?;

                models.insert(
                    stage_name.to_string(),
                    ModelNode::load(
                        &stage_name,
                        &stage,
                        &model_data,
                        &shared_state,
                        &input_tensors,
                        &output_tensors,
                    )?,
                );
            },
            Stage::ProcBlock(stage) => {
                println!(
                    "Pipeline stage: {} proc block {:?} {}",
                    stage_name, stage.args, stage.proc_block.base
                );
                let wasm = read_zip_resource_by_path(&stage.proc_block.base)
                    .context("Unable to load the proc_block")?;

                procblocks.insert(
                    stage_name.to_string(),
                    ProcBlockNode::load(
                        &stage_name,
                        &stage,
                        &wasm,
                        &store,
                        &mut imports,
                        &shared_state,
                        &input_tensors,
                        &output_tensors,
                    )?,
                );
            },
            Stage::Out(_) => {
                // outputs.push(stage_name.to_string());
            },
        }
    }

    Ok((models, procblocks))
}

fn get_tensors(
    inputs: &Vec<String>,
    outputs: &Vec<String>,
    pipeline: &IndexMap<String, Stage>,
) -> Result<
    (
        Vec<Option<TensorResult>>,
        HashMap<String, usize>,
        HashMap<String, usize>,
        Vec<String>,
    ),
    Error,
> {
    let mut nodes_to_visit = outputs.clone();
    let mut nodes_visited = Vec::new();
    let mut tensors: Vec<Option<TensorResult>> = Vec::new();
    let mut output_tensors: HashMap<String, usize> = HashMap::new();
    let mut input_tensors: HashMap<String, usize> = HashMap::new();

    // For Inputs/Capabilities - input tensors and output tensors are the same?
    for item in inputs {
        tensors.push(None);
        input_tensors.insert(key(item, Some(0)), tensors.len() - 1);
        output_tensors.insert(key(item, Some(0)), tensors.len() - 1);
    }

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
            if !nodes_visited.contains(&input.name) {
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
            let &input_tensor_index = output_tensors.get(&input_key).context(
                anyhow!("Invalid input key specified: {}", &input_key),
            )?;
            input_tensors.insert(key(stage_name, Some(i)), input_tensor_index);
        }
    }

    nodes_visited.reverse();

    Ok((tensors, input_tensors, output_tensors, nodes_visited))
}

fn key(node_name: &str, tensor_index: Option<usize>) -> String {
    format!("{}.{}", node_name, tensor_index.or(Some(0)).unwrap())
}

#[derive(Debug, Clone)]
pub enum Never {}

#[derive(Debug, Clone)]
struct Metadata {
    description: String,
    repository: String,
    homepage: String,
    tags: Vec<String>,
    arguments: Vec<ArgumentMetadata>,
    inputs: Vec<TensorMetadata>,
    outputs: Vec<TensorMetadata>
}

#[derive(Debug, Clone)]
struct ArgumentMetadata {
    description: String,
    default_value: String,
    // hint: Vec<ArgumentHints>
}

#[derive(Debug, Clone)]
struct TensorMetadata {

}

#[derive(Debug, Clone)]
enum Dimensions {
    Dynamic,
    Fixed(Vec<usize>)
}

#[derive(Debug, Clone)]
struct TensorConstraint {
    name: String,
    element_type: ElementType,
    dimensions: Dimensions
}

#[derive(Debug, Default, Clone)]
struct GraphContext {
    arguments: HashMap<String, String>,
    input_tensors: Vec<TensorConstraint>,
    output_tensors: Vec<TensorConstraint>
}

impl runtime_v1::RuntimeV1 for Runtime {
    type ArgumentHint = Never;
    type ArgumentMetadata = Never;
    type KernelContext = Arc<Mutex<State>>;
    type Metadata = Metadata;
    type Model = Never;
    type TensorHint = Never;
    type TensorMetadata = TensorMetadata;
    type GraphContext = String;

    fn metadata_new(&mut self, _name: &str, _version: &str) -> Self::Metadata {
        todo!()
    }

    fn metadata_set_description(
        &mut self,
        _self_: &Self::Metadata,
        _description: &str,
    ) {
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

    fn metadata_add_argument(
        &mut self,
        _self_: &Self::Metadata,
        _arg: &Self::ArgumentMetadata,
    ) {
        todo!()
    }

    fn metadata_add_input(
        &mut self,
        _self_: &Self::Metadata,
        _metadata: &Self::TensorMetadata,
    ) {
        todo!()
    }

    fn metadata_add_output(
        &mut self,
        _self_: &Self::Metadata,
        _metadata: &Self::TensorMetadata,
    ) {
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

    fn interpret_as_image(&mut self) -> Self::TensorHint { todo!() }

    fn interpret_as_audio(&mut self) -> Self::TensorHint { todo!() }

    fn supported_shapes(
        &mut self,
        _supported_element_types: Vec<ElementType>,
        _dimensions: DimensionsParam<'_>,
    ) -> Self::TensorHint {
        todo!()
    }

    fn interpret_as_number_in_range(
        &mut self,
        _min: &str,
        _max: &str,
    ) -> Self::ArgumentHint {
        todo!()
    }

    fn interpret_as_string_in_enum(
        &mut self,
        _string_enum: Vec<&str>,
    ) -> Self::ArgumentHint {
        todo!()
    }

    fn non_negative_number(&mut self) -> Self::ArgumentHint { todo!() }

    fn supported_argument_type(
        &mut self,
        _hint: ArgumentType,
    ) -> Self::ArgumentHint {
        todo!()
    }

    fn register_node(&mut self, _metadata: &Self::Metadata) { todo!() }

    fn graph_context_for_node(
        &mut self,
        _node_id: &str,
    ) -> Option<Self::GraphContext> {
        self.shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get(_node_id)
            .and_then(|_| Some(_node_id.to_string()))
    }

    fn graph_context_get_argument(
        &mut self,
        _self_: &Self::GraphContext,
        _name: &str,
    ) -> Option<String> {
        self.shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get(_self_)
            .and_then(|c| c.arguments.get(_name).and_then(|v| Some(v.clone()) ))
    }

    fn graph_context_add_input_tensor(
        &mut self,
        _self_: &Self::GraphContext,
        _name: &str,
        _element_type: ElementType,
        _dimensions: DimensionsParam<'_>,
    ) {
        self.shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get_mut(_self_)
            .and_then(|c| {
                Some(c.input_tensors.push(
                    TensorConstraint {
                        name: _name.to_string(),
                        element_type: _element_type,
                        dimensions: match _dimensions {
                            DimensionsParam::Dynamic => Dimensions::Dynamic,
                            DimensionsParam::Fixed(shape) => Dimensions::Fixed(shape.iter().map(|&i| i.get() as usize).collect())
                        }
                   }))
                });
    }

    fn graph_context_add_output_tensor(
        &mut self,
        _self_: &Self::GraphContext,
        _name: &str,
        _element_type: ElementType,
        _dimensions: DimensionsParam<'_>,
    ) {
        self.shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get_mut(_self_)
            .and_then(|c| {
                Some(c.output_tensors.push(
                    TensorConstraint {
                        name: _name.to_string(),
                        element_type: _element_type,
                        dimensions: match _dimensions {
                            DimensionsParam::Dynamic => Dimensions::Dynamic,
                            DimensionsParam::Fixed(shape) => Dimensions::Fixed(shape.iter().map(|&i| i.get() as usize).collect())
                        }
                   }))
                });
    }

    fn kernel_context_for_node(
        &mut self,
        _node_id: &str,
    ) -> Option<Self::KernelContext> {
        Some(self.shared_state.clone())
    }

    fn kernel_context_get_argument(
        &mut self,
        state: &Arc<Mutex<State>>,
        name: &str,
    ) -> Option<String> {
        todo!()
    }

    fn kernel_context_get_input_tensor(
        &mut self,
        state: &Arc<Mutex<State>>,
        name: &str,
    ) -> Option<TensorResult> {
        todo!()
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
        todo!()
    }

    fn is_enabled(&mut self, _metadata: LogMetadata) -> bool { true }

    fn log(
        &mut self,
        metadata: LogMetadata,
        message: &str,
        data: Vec<(&'_ str, LogValue<'_>)>,
    ) {
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

    fn kernel_context_get_global_input(
        &mut self,
        self_: &Self::KernelContext,
        name: &str,
    ) -> Option<TensorResult> {
        todo!()
    }

    fn kernel_context_set_global_output(
        &mut self,
        self_: &Self::KernelContext,
        name: &str,
        tensor: TensorParam<'_>,
    ) {
        todo!()
    }

    fn model_load(
        &mut self,
        model_format: &str,
        model: &[u8],
        arguments: Vec<(&str, &str)>,
    ) -> Result<Self::Model, ModelLoadError> {
        todo!()
    }

    fn model_inputs(&mut self, self_: &Self::Model) -> Vec<runtime_v1::Shape> {
        todo!()
    }

    fn model_outputs(&mut self, self_: &Self::Model) -> Vec<runtime_v1::Shape> {
        todo!()
    }

    fn model_infer(
        &mut self,
        self_: &Self::Model,
        inputs: Vec<TensorParam<'_>>,
    ) -> Result<Vec<TensorResult>, ModelInferError> {
        todo!()
    }
}
