use indexmap::IndexMap;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Error};
use wasmer::{ImportObject, Module, Store};

use crate::zune::{
    key, runtime_v1, ArgumentType, DimensionsParam, ElementType, LogLevel,
    LogMetadata, LogValue, ModelInferError, ModelLoadError, Node, ProcBlockV1,
    Runtime, TensorParam, TensorResult,
};

pub(crate) struct ProcBlockNode {
    node_id: String,
    context: ProcBlockV1,
}

impl ProcBlockNode {
    #[tracing::instrument(skip_all, level = "debug", fields(%node_id))]
    pub(crate) fn load(
        node_id: &str,
        wasm: &[u8],
        runtime: &Runtime,
        input_tensors: &IndexMap<String, usize>,
        output_tensors: &IndexMap<String, usize>,
    ) -> Result<ProcBlockNode, Error> {
        let shared_state = runtime.shared_state.clone();
        let store = Store::default();
        let mut imports = ImportObject::default();
        super::add_to_imports(&store, &mut imports, runtime.clone());

        let module =
            Module::new(&store, wasm).context("Unable to load the module")?;
        let (pb, _) =
            ProcBlockV1::instantiate(&store, &module, &mut imports)
                .context("Unable to instantiate the WebAssembly module")?;

        let _result = pb.graph(node_id)??;

        // Assign tensors
        // TODO: See if this can be more smart.
        // Not bothering with that for now because tensor names are lost in current Runefile format
        shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get_mut(node_id)
            .and_then(|c| {
                c.input_tensors.iter_mut().enumerate().for_each(
                    |(i, (_, t))| {
                        input_tensors.get(&key(node_id, Some(i))).and_then(
                            |&tensor_index| {
                                Some(t.tensor_id = Some(tensor_index))
                            },
                        );
                    },
                );

                c.output_tensors.iter_mut().enumerate().for_each(
                    |(i, (_, t))| {
                        output_tensors.get(&key(node_id, Some(i))).and_then(
                            |&tensor_index| {
                                Some(t.tensor_id = Some(tensor_index))
                            },
                        );
                    },
                );
                Some(())
            });

        Ok(ProcBlockNode {
            node_id: node_id.to_string(),
            context: pb,
        })
    }
}

impl Node for ProcBlockNode {
    #[tracing::instrument(skip_all, level = "debug")]
    fn run(&mut self) -> Result<(), Error> {
        self.context.kernel(&self.node_id)??;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Never {}

#[derive(Debug, Clone)]
pub(crate) struct Metadata {
    description: String,
    repository: String,
    homepage: String,
    tags: Vec<String>,
    arguments: Vec<ArgumentMetadata>,
    inputs: Vec<TensorMetadata>,
    outputs: Vec<TensorMetadata>,
}

#[derive(Debug, Clone)]
struct ArgumentMetadata {
    description: String,
    default_value: String,
    // hint: Vec<ArgumentHints>
}

#[derive(Debug, Clone)]
pub(crate) struct TensorMetadata {}

#[derive(Debug, Clone)]
pub(crate) enum Dimensions {
    Dynamic,
    Fixed(Vec<usize>),
}

#[derive(Debug, Clone)]
pub(crate) struct TensorConstraint {
    pub tensor_id: Option<usize>,
    pub element_type: ElementType,
    pub dimensions: Dimensions,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct GraphContext {
    pub arguments: IndexMap<String, String>,
    pub input_tensors: IndexMap<String, TensorConstraint>,
    pub output_tensors: IndexMap<String, TensorConstraint>,
}

impl runtime_v1::RuntimeV1 for Runtime {
    type ArgumentHint = Never;
    type ArgumentMetadata = Never;
    type KernelContext = String;
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
        _ctx: &Self::Metadata,
        _description: &str,
    ) {
        todo!()
    }

    fn metadata_set_repository(&mut self, _ctx: &Self::Metadata, _url: &str) {
        todo!()
    }

    fn metadata_set_homepage(&mut self, _ctx: &Self::Metadata, _url: &str) {
        todo!()
    }

    fn metadata_add_tag(&mut self, _ctx: &Self::Metadata, _tag: &str) {
        todo!()
    }

    fn metadata_add_argument(
        &mut self,
        _ctx: &Self::Metadata,
        _arg: &Self::ArgumentMetadata,
    ) {
        todo!()
    }

    fn metadata_add_input(
        &mut self,
        _ctx: &Self::Metadata,
        _metadata: &Self::TensorMetadata,
    ) {
        todo!()
    }

    fn metadata_add_output(
        &mut self,
        _ctx: &Self::Metadata,
        _metadata: &Self::TensorMetadata,
    ) {
        todo!()
    }

    fn argument_metadata_new(&mut self, _name: &str) -> Self::ArgumentMetadata {
        todo!()
    }

    fn argument_metadata_set_description(
        &mut self,
        _ctx: &Self::ArgumentMetadata,
        _description: &str,
    ) {
        todo!()
    }

    fn argument_metadata_set_default_value(
        &mut self,
        _ctx: &Self::ArgumentMetadata,
        _default_value: &str,
    ) {
        todo!()
    }

    fn argument_metadata_add_hint(
        &mut self,
        _ctx: &Self::ArgumentMetadata,
        _hint: &Self::ArgumentHint,
    ) {
        todo!()
    }

    fn tensor_metadata_new(&mut self, _name: &str) -> Self::TensorMetadata {
        todo!()
    }

    fn tensor_metadata_set_description(
        &mut self,
        _ctx: &Self::TensorMetadata,
        _description: &str,
    ) {
        todo!()
    }

    fn tensor_metadata_add_hint(
        &mut self,
        _ctx: &Self::TensorMetadata,
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

    fn non_negative_number(&mut self) -> Self::ArgumentHint {
        todo!()
    }

    fn supported_argument_type(
        &mut self,
        _hint: ArgumentType,
    ) -> Self::ArgumentHint {
        todo!()
    }

    fn register_node(&mut self, _metadata: &Self::Metadata) {
        todo!()
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn graph_context_for_node(
        &mut self,
        node_id: &str,
    ) -> Option<Self::GraphContext> {
        self.shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get(node_id)?;

        Some(node_id.to_string())
    }

    #[tracing::instrument(skip(self, ctx), level = "debug")]
    fn graph_context_get_argument(
        &mut self,
        ctx: &Self::GraphContext,
        name: &str,
    ) -> Option<String> {
        self.shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get(ctx)
            .and_then(|c| c.arguments.get(name).and_then(|v| Some(v.clone())))
    }

    #[tracing::instrument(skip(self, ctx), level = "debug")]
    fn graph_context_add_input_tensor(
        &mut self,
        ctx: &Self::GraphContext,
        name: &str,
        element_type: ElementType,
        dimensions: DimensionsParam<'_>,
    ) {
        self.shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get_mut(ctx)
            .and_then(|c| {
                c.input_tensors.insert(
                    name.to_string(),
                    TensorConstraint {
                        tensor_id: None,
                        element_type,
                        dimensions: match dimensions {
                            DimensionsParam::Dynamic => Dimensions::Dynamic,
                            DimensionsParam::Fixed(shape) => Dimensions::Fixed(
                                shape
                                    .iter()
                                    .map(|&i| i.get() as usize)
                                    .collect(),
                            ),
                        },
                    },
                )
            });
    }

    #[tracing::instrument(skip(self, ctx), level = "debug")]
    fn graph_context_add_output_tensor(
        &mut self,
        ctx: &Self::GraphContext,
        name: &str,
        element_type: ElementType,
        dimensions: DimensionsParam<'_>,
    ) {
        self.shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get_mut(ctx)
            .and_then(|c| {
                c.output_tensors.insert(
                    name.to_string(),
                    TensorConstraint {
                        tensor_id: None,
                        element_type,
                        dimensions: match dimensions {
                            DimensionsParam::Dynamic => Dimensions::Dynamic,
                            DimensionsParam::Fixed(shape) => Dimensions::Fixed(
                                shape
                                    .iter()
                                    .map(|&i| i.get() as usize)
                                    .collect(),
                            ),
                        },
                    },
                )
            });
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn kernel_context_for_node(
        &mut self,
        node_id: &str,
    ) -> Option<Self::KernelContext> {
        self.shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get(node_id)?;
        Some(node_id.to_string())
    }

    #[tracing::instrument(skip(self, ctx), level = "debug")]
    fn kernel_context_get_argument(
        &mut self,
        ctx: &Self::KernelContext,
        name: &str,
    ) -> Option<String> {
        self.shared_state
            .lock()
            .unwrap()
            .graph_contexts
            .get(ctx)
            .and_then(|c| c.arguments.get(name).and_then(|v| Some(v.clone())))
    }

    #[tracing::instrument(skip(self, ctx), level = "debug")]
    fn kernel_context_get_input_tensor(
        &mut self,
        ctx: &Self::KernelContext,
        name: &str,
    ) -> Option<TensorResult> {
        let state = self.shared_state.lock().unwrap();

        let tensor_id = state
            .graph_contexts
            .get(ctx)
            .and_then(|c| c.input_tensors.get(name).and_then(|v| v.tensor_id));

        match tensor_id {
            Some(i) => {
                let tensor = state.tensors[i].clone();
                tracing::debug!(
                    ?tensor.element_type,
                    ?tensor.dimensions,
                    tensor.buffer_length = tensor.buffer.len(),
                    id=i,
                    "Returning a tensor",
                );
                tensor
            },
            _ => None,
        }
    }

    #[tracing::instrument(skip(self, ctx, buffer), level = "debug")]
    fn kernel_context_set_output_tensor(
        &mut self,
        ctx: &Self::KernelContext,
        name: &str,
        TensorParam {
            element_type,
            buffer,
            dimensions,
        }: TensorParam<'_>,
    ) {
        let mut state = self.shared_state.lock().unwrap();

        let tensor_id = state
            .graph_contexts
            .get(ctx)
            .and_then(|c| c.output_tensors.get(name).and_then(|v| v.tensor_id));

        let dimensions = dimensions.iter().map(|&i| i.get() as u32).collect();

        // Todo check tensor constraint

        if tensor_id.is_some() {
            state.tensors[tensor_id.unwrap()] = Some(TensorResult {
                element_type,
                buffer: buffer.to_vec(),
                dimensions,
            });
        }
    }

    fn is_enabled(&mut self, _metadata: LogMetadata) -> bool {
        true
    }

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
        _ctx: &Self::KernelContext,
        _name: &str,
    ) -> Option<TensorResult> {
        todo!()
    }

    fn kernel_context_set_global_output(
        &mut self,
        _ctx: &Self::KernelContext,
        _name: &str,
        _tensor: TensorParam<'_>,
    ) {
        todo!()
    }

    fn model_load(
        &mut self,
        _: &str,
        _: &[u8],
        _: Vec<(&str, &str)>,
    ) -> Result<Self::Model, ModelLoadError> {
        todo!()
    }

    fn model_inputs(&mut self, _: &Self::Model) -> Vec<runtime_v1::Shape> {
        todo!()
    }

    fn model_outputs(&mut self, _: &Self::Model) -> Vec<runtime_v1::Shape> {
        todo!()
    }

    fn model_infer(
        &mut self,
        _: &Self::Model,
        _: Vec<TensorParam<'_>>,
    ) -> Result<Vec<TensorResult>, ModelInferError> {
        todo!()
    }
}
