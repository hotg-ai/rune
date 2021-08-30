#[cfg(feature = "wasm3-runtime")]
mod wasm3_impl;

#[cfg(feature = "wasmer-runtime")]
mod wasmer_impl;

use std::{
    cell::Cell,
    collections::HashMap,
    io::Read,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};
use log::{Level, Record};
use anyhow::Error;
use hotg_rune_core::{Shape, capabilities, outputs, TFLITE_MIMETYPE};
use hotg_rune_runtime::{Capability, Output, common_outputs::Serial};

use crate::random::Random;

type LogFunc = dyn Fn(&Record<'_>) -> Result<(), Error> + Send + Sync + 'static;

pub struct BaseImage {
    capabilities: HashMap<u32, Box<dyn CapabilityFactory>>,
    models: HashMap<String, Box<dyn ModelFactory>>,
    outputs: HashMap<u32, Box<dyn OutputFactory>>,
    resources: HashMap<String, Box<dyn ResourceFactory>>,
    log: Arc<LogFunc>,
}

impl BaseImage {
    pub fn new() -> Self {
        BaseImage {
            capabilities: HashMap::new(),
            models: HashMap::new(),
            outputs: HashMap::new(),
            resources: HashMap::new(),
            log: Arc::new(|_| Ok(())),
        }
    }

    pub fn with_defaults() -> Self {
        let mut image = BaseImage::new();

        image
            .with_logger(default_log_function)
            .register_output(outputs::SERIAL, serial_factory)
            .register_capability(capabilities::RAND, || {
                Ok(Box::new(Random::from_os()) as Box<dyn Capability>)
            });

        #[cfg(feature = "tflite")]
        image.register_model(TFLITE_MIMETYPE, tf::initialize_model);

        image
    }

    pub fn with_logger<F>(&mut self, log_func: F) -> &mut Self
    where
        F: Fn(&Record<'_>) -> Result<(), Error> + Send + Sync + 'static,
    {
        self.log = Arc::new(log_func);
        self
    }

    pub fn register_capability(
        &mut self,
        id: u32,
        factory: impl CapabilityFactory,
    ) -> &mut Self {
        self.capabilities.insert(id, Box::new(factory));
        self
    }

    pub fn register_output(
        &mut self,
        id: u32,
        factory: impl OutputFactory,
    ) -> &mut Self {
        self.outputs.insert(id, Box::new(factory));
        self
    }

    pub fn register_model(
        &mut self,
        mimetype: &str,
        factory: impl ModelFactory,
    ) -> &mut Self {
        self.models.insert(mimetype.to_string(), Box::new(factory));
        self
    }

    pub fn register_resource(
        &mut self,
        name: &str,
        factory: impl ResourceFactory,
    ) -> &mut Self {
        self.resources.insert(name.to_string(), Box::new(factory));
        self
    }
}

fn default_log_function(record: &Record<'_>) -> Result<(), Error> {
    log::logger().log(record);

    if record.level() > Level::Error {
        Ok(())
    } else {
        Err(anyhow::anyhow!("{}", record.args())
            .context("The Rune encountered a fatal error"))
    }
}

#[derive(Debug, Default, Clone)]
struct Identifiers(Arc<AtomicU32>);

impl Identifiers {
    fn next(&self) -> u32 { self.0.fetch_add(1, Ordering::SeqCst) }
}

impl Default for BaseImage {
    fn default() -> Self { BaseImage::new() }
}

pub trait Model: Send + Sync + 'static {
    /// Run inference on the input tensors, writing the results to `outputs`.
    ///
    /// # Safety
    ///
    /// Implementations can assume that they have unique access to `outputs`
    /// (i.e. converting the `&[Cell<u8>]` to `&mut [u8]` is valid).
    ///
    /// The `inputs` parameter may be aliased.
    unsafe fn infer(
        &mut self,
        inputs: &[&[Cell<u8>]],
        outputs: &[&[Cell<u8>]],
    ) -> Result<(), Error>;

    fn input_shapes(&self) -> &[Shape<'_>];
    fn output_shapes(&self) -> &[Shape<'_>];
}

pub trait ModelFactory: Send + Sync + 'static {
    fn new_model(
        &self,
        model_bytes: &[u8],
        inputs: Option<&[Shape<'_>]>,
        outputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn Model>, Error>;
}

impl<F> ModelFactory for F
where
    F: Fn(
            &[u8],
            Option<&[Shape<'_>]>,
            Option<&[Shape<'_>]>,
        ) -> Result<Box<dyn Model>, Error>
        + Send
        + Sync
        + 'static,
{
    fn new_model(
        &self,
        model_bytes: &[u8],
        inputs: Option<&[Shape<'_>]>,
        outputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn Model>, Error> {
        (*self)(model_bytes, inputs, outputs)
    }
}

pub trait CapabilityFactory: Send + Sync + 'static {
    fn new_capability(&self) -> Result<Box<dyn Capability>, Error>;
}

impl<F> CapabilityFactory for F
where
    F: Fn() -> Result<Box<dyn Capability>, Error> + Send + Sync + 'static,
{
    fn new_capability(&self) -> Result<Box<dyn Capability>, Error> { (*self)() }
}

pub trait OutputFactory: Send + Sync + 'static {
    fn new_output(
        &self,
        inputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn Output>, Error>;
}

impl<F> OutputFactory for F
where
    F: Fn(Option<&[Shape<'_>]>) -> Result<Box<dyn Output>, Error>
        + Send
        + Sync
        + 'static,
{
    fn new_output(
        &self,
        inputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn Output>, Error> {
        (*self)(inputs)
    }
}

fn serial_factory(_: Option<&[Shape<'_>]>) -> Result<Box<dyn Output>, Error> {
    Ok(Box::new(Serial::default()))
}

pub trait ResourceFactory: Send + Sync + 'static {
    fn open_resource(
        &self,
        name: &str,
    ) -> Result<Box<dyn Read + Send + Sync + 'static>, Error>;
}

impl<F> ResourceFactory for F
where
    F: Fn(&str) -> Result<Box<dyn Read + Send + Sync + 'static>, Error>
        + Send
        + Sync
        + 'static,
{
    fn open_resource(
        &self,
        name: &str,
    ) -> Result<Box<dyn Read + Send + Sync + 'static>, Error> {
        (self)(name)
    }
}

#[cfg(feature = "tflite")]
mod tf {
    use super::*;
    use anyhow::Context;
    use log::Level;
    use hotg_rune_core::reflect::Type;
    use tflite::{
        FlatBufferModel, Interpreter, InterpreterBuilder,
        context::{ElementKind, TensorInfo},
        ops::builtin::BuiltinOpResolver,
    };

    pub(crate) fn initialize_model(
        raw: &[u8],
        inputs: Option<&[Shape<'_>]>,
        outputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn super::Model>, Error> {
        let model = TensorFlowLiteModel::load(raw)?;

        if let Some(shapes) = inputs {
            ensure_shapes_equal(shapes, model.input_shapes())
                .context("Invalid inputs")?;
        }
        if let Some(shapes) = outputs {
            ensure_shapes_equal(shapes, model.output_shapes())
                .context("Invalid outputs")?;
        }

        Ok(Box::new(model))
    }

    fn ensure_shapes_equal(
        from_rune: &[Shape],
        from_model: &[Shape],
    ) -> Result<(), Error> {
        if from_rune == from_model {
            return Ok(());
        }

        fn pretty_shapes(shapes: &[Shape<'_>]) -> String {
            format!(
                "[{}]",
                shapes
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }

        anyhow::bail!(
            "The Rune said tensors would be {}, but the model said they would be {}",
            pretty_shapes(from_rune),
            pretty_shapes(from_model),
        );
    }

    struct TensorFlowLiteModel {
        interpreter: Interpreter<'static, BuiltinOpResolver>,
        inputs: Vec<Shape<'static>>,
        outputs: Vec<Shape<'static>>,
    }

    impl TensorFlowLiteModel {
        fn load(model_bytes: &[u8]) -> Result<Self, Error> {
            let model =
                FlatBufferModel::build_from_buffer(model_bytes.to_vec())
                    .context("Unable to build the model")?;

            let resolver = BuiltinOpResolver::default();

            let builder = InterpreterBuilder::new(model, resolver)
                .context("Unable to create a model interpreter builder")?;
            let mut interpreter = builder
                .build()
                .context("Unable to initialize the model interpreter")?;

            log_interpreter(&interpreter);

            interpreter
                .allocate_tensors()
                .context("Unable to allocate tensors")?;

            let inputs = tensor_shapes(&interpreter, interpreter.inputs())
                .context("Invalid input types")?;
            let outputs = tensor_shapes(&interpreter, interpreter.outputs())
                .context("Invalid output types")?;

            Ok(TensorFlowLiteModel {
                interpreter,
                inputs,
                outputs,
            })
        }
    }

    fn tensor_shapes(
        interpreter: &Interpreter<BuiltinOpResolver>,
        tensor_indices: &[i32],
    ) -> Result<Vec<Shape<'static>>, Error> {
        let mut tensors = Vec::new();

        for (i, tensor_index) in tensor_indices.iter().copied().enumerate() {
            let tensor_info =
                interpreter.tensor_info(tensor_index).with_context(|| {
                    format!("Unable to find tensor #{}", tensor_index)
                })?;
            let shape = tensor_shape(&tensor_info)
                .with_context(|| format!("Tensor {} is invalid", i))?
                .to_owned();

            tensors.push(shape);
        }

        Ok(tensors)
    }

    fn tensor_shape(tensor: &TensorInfo) -> Result<Shape<'_>, Error> {
        let element_type = match tensor.element_kind {
            ElementKind::kTfLiteFloat32 => Type::f32,
            ElementKind::kTfLiteInt32 => Type::i32,
            ElementKind::kTfLiteUInt8 => Type::u8,
            ElementKind::kTfLiteInt64 => Type::i64,
            ElementKind::kTfLiteString => Type::String,
            ElementKind::kTfLiteInt16 => Type::i16,
            ElementKind::kTfLiteInt8 => Type::i8,
            other => {
                anyhow::bail!("Unsupported element type: {:?}", other);
            },
        };

        Ok(Shape::new(element_type, tensor.dims.as_slice()))
    }

    fn log_interpreter(interpreter: &Interpreter<BuiltinOpResolver>) {
        if !log::log_enabled!(Level::Debug) {
            return;
        }
        let inputs: Vec<_> = interpreter
            .inputs()
            .iter()
            .filter_map(|ix| interpreter.tensor_info(*ix))
            .collect();

        let outputs: Vec<_> = interpreter
            .outputs()
            .iter()
            .filter_map(|ix| interpreter.tensor_info(*ix))
            .collect();

        log::debug!(
            "Loaded model with inputs {:?} and outputs {:?}",
            inputs,
            outputs
        );
    }

    impl super::Model for TensorFlowLiteModel {
        unsafe fn infer(
            &mut self,
            inputs: &[&[Cell<u8>]],
            outputs: &[&[Cell<u8>]],
        ) -> Result<(), Error> {
            let interpreter = &mut self.interpreter;

            anyhow::ensure!(
                interpreter.inputs().len() == inputs.len(),
                "The model supports {} inputs but {} were provided",
                interpreter.inputs().len(),
                inputs.len(),
            );
            anyhow::ensure!(
                interpreter.outputs().len() == outputs.len(),
                "The model supports {} inputs but {} were provided",
                interpreter.outputs().len(),
                outputs.len(),
            );

            let input_indices: Vec<_> = interpreter.inputs().to_vec();

            for (&ix, &input) in input_indices.iter().zip(inputs) {
                let buffer = interpreter
                    .tensor_buffer_mut(ix)
                    .context("Unable to get the input buffer")?;

                // FIXME: Figure out how to tell TensorFlow Lite to use the
                // input buffers directly instead of copying.
                for (src, dest) in input.iter().zip(buffer) {
                    *dest = src.get();
                }
            }

            interpreter.invoke().context("Calling the model failed")?;

            for (&ix, output) in interpreter.outputs().iter().zip(outputs) {
                let buffer = interpreter
                    .tensor_buffer(ix)
                    .context("Unable to get the output buffer")?;

                // FIXME: Figure out how to tell TensorFlow Lite to use the
                // output buffers directly instead of copying.
                for (src, dest) in buffer.iter().zip(*output) {
                    dest.set(*src);
                }
            }

            Ok(())
        }

        fn input_shapes(&self) -> &[Shape<'_>] { &self.inputs }

        fn output_shapes(&self) -> &[Shape<'_>] { &self.outputs }
    }
}
