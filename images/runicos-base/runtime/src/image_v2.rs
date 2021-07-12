use std::{
    collections::HashMap,
    convert::TryInto,
    hash::Hash,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
};
use log::Record;
use anyhow::{Context, Error};
use rune_core::{Shape, capabilities};
use rune_runtime::{Capability, Output, common_capabilities::Random};
use wasmer::{
    Array, Function, ImportObject, LazyInit, Memory, RuntimeError, Store,
    WasmPtr,
};

type LogFunc = dyn Fn(&Record<'_>) -> Result<(), Error> + Send + Sync + 'static;

pub struct BaseImage {
    capabilities: HashMap<u32, Box<dyn CapabilityFactory>>,
    models: HashMap<String, Box<dyn ModelFactory>>,
    outputs: HashMap<u32, Box<dyn OutputFactory>>,
    log: Arc<LogFunc>,
}

impl BaseImage {
    pub fn new() -> Self {
        BaseImage {
            capabilities: HashMap::new(),
            models: HashMap::new(),
            outputs: HashMap::new(),
            log: Arc::new(|_| Ok(())),
        }
    }

    pub fn with_defaults() -> Self {
        let mut image = BaseImage::new();

        image
            .with_logger(|r| {
                log::logger().log(r);
                Ok(())
            })
            .register_capability(capabilities::RAND, || {
                Ok(Box::new(Random::from_os()) as Box<dyn Capability>)
            });

        #[cfg(feature = "tflite")]
        image.register_model("application/tflite-model", tf::initialize_model);

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

    pub fn to_imports(self, store: &Store) -> ImportObject {
        let BaseImage {
            capabilities,
            models,
            outputs,
            log,
        } = self;
        let identifiers = Identifiers::default();

        let log_env = LogEnv {
            log,
            memory: LazyInit::new(),
        };
        let cap_env = CapabilityEnv {
            factories: Arc::new(capabilities),
            instances: Arc::new(Mutex::new(HashMap::new())),
            identifiers,
            memory: LazyInit::new(),
        };

        wasmer::imports! {
            "env" => {
                "_debug" =>  Function::new_native_with_env(store, log_env, debug),
                "request_capability" => Function::new_native_with_env(store, cap_env.clone(), request_capability),
                "request_capability_set_param" => Function::new_native_with_env(store, cap_env.clone(), request_capability_set_param),
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Identifiers(Arc<AtomicU32>);

impl Identifiers {
    fn next(&self) -> u32 { self.0.fetch_add(1, Ordering::SeqCst) }
}

#[derive(Clone, wasmer::WasmerEnv)]
struct CapabilityEnv {
    factories: Arc<HashMap<u32, Box<dyn CapabilityFactory>>>,
    instances: Arc<Mutex<HashMap<u32, Box<dyn Capability>>>>,
    identifiers: Identifiers,
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

fn request_capability(
    env: &CapabilityEnv,
    capability_type: u32,
) -> Result<u32, RuntimeError> {
    match env.factories.get(&capability_type) {
        Some(f) => {
            let cap = f.new_capability().map_err(runtime_error)?;
            let id = env.identifiers.next();
            env.instances.lock().unwrap().insert(id, cap);
            Ok(id)
        },
        None => {
            if let Some(name) = rune_core::capabilities::name(capability_type) {
                return Err(runtime_error(anyhow::anyhow!(
                    "No \"{}\" capability registered",
                    name
                )));
            }

            Err(runtime_error(anyhow::anyhow!(
                "No capability registered for capability type {}",
                capability_type
            )))
        },
    }
}

fn request_capability_set_param(
    env: &CapabilityEnv,
    capability_id: u32,
    key_ptr: WasmPtr<u8, Array>,
    key_len: u32,
    value_ptr: WasmPtr<u8, Array>,
    value_len: u32,
    value_type: u32,
) -> Result<(), RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: this function isn't reentrant, so we don't need to worry about
    // concurrent mutations.
    unsafe {
        let key = key_ptr
            .get_utf8_str(memory, key_len)
            .context("Unable to read the key")
            .map_err(runtime_error)?;

        let ty = value_type
            .try_into()
            .map_err(|()| Error::msg("Invalid key type"))
            .map_err(runtime_error)?;

        let value = value_ptr
            .deref(memory, 0, value_len)
            .context("Unable to read the value")
            .map_err(runtime_error)?;

        // Safety: this is sound when there are no concurrent modifications
        let value: &[u8] =
            std::slice::from_raw_parts(value.as_ptr().cast(), value.len());
        let value = rune_core::Value::from_le_bytes(ty, value)
            .context("Invalid value")
            .map_err(runtime_error)?;

        env.instances
            .lock()
            .unwrap()
            .get_mut(&capability_id)
            .context("No such capability")
            .map_err(runtime_error)?
            .set_parameter(key, value)
            .with_context(|| {
                format!(
                    "Unable to set the \"{}\" parameter to \"{}\"",
                    key, value
                )
            })
            .map_err(runtime_error)?;
    }

    Ok(())
}

#[derive(Clone, wasmer::WasmerEnv)]
struct LogEnv {
    log: Arc<LogFunc>,
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

fn debug(
    env: &LogEnv,
    msg: WasmPtr<u8, Array>,
    len: u32,
) -> Result<(), RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: this function isn't reentrant, so we don't need to worry about
    // concurrent mutations.
    unsafe {
        let message = msg
            .get_utf8_str(memory, len)
            .context("Unable to read the message")
            .map_err(runtime_error)?;

        let record: rune_core::SerializableRecord =
            serde_json::from_str(message)
                .context("Unable to parse the log message")
                .map_err(runtime_error)?;

        record
            .with_record(|r| (env.log)(r))
            .context("Logging failed")
            .map_err(runtime_error)?;
    }

    Ok(())
}

impl Default for BaseImage {
    fn default() -> Self { BaseImage::new() }
}

pub trait Model {
    fn infer(
        &mut self,
        input: &[&[u8]],
        output: &mut [&mut [u8]],
    ) -> Result<(), Error>;
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

fn runtime_error(e: Error) -> RuntimeError {
    RuntimeError::from_trap(wasmer_vm::Trap::User(e.into()))
}

#[cfg(feature = "tflite")]
mod tf {
    use super::*;
    use anyhow::Context;
    use log::Level;
    use rune_core::reflect::Type;
    use tflite::{
        FlatBufferModel, Interpreter, InterpreterBuilder,
        context::{ElementKind, TensorInfo},
        ops::builtin::BuiltinOpResolver,
    };

    pub(crate) fn initialize_model(
        raw: &[u8],
        inputs: Option<&[Shape<'_>]>,
        outputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn Model>, Error> {
        let model = FlatBufferModel::build_from_buffer(raw.to_vec())
            .context("Unable to build the model")?;

        let resolver = BuiltinOpResolver::default();

        let builder = InterpreterBuilder::new(model, resolver)
            .context("Unable to create a model interpreter builder")?;
        let mut interpreter = builder
            .build()
            .context("Unable to initialize the model interpreter")?;

        validate(&interpreter, inputs, outputs)?;
        log_interpreter(&interpreter);

        interpreter
            .allocate_tensors()
            .context("Unable to allocate tensors")?;

        Ok(Box::new(interpreter))
    }

    fn validate(
        interpreter: &Interpreter<BuiltinOpResolver>,
        inputs: Option<&[Shape<'_>]>,
        outputs: Option<&[Shape<'_>]>,
    ) -> Result<(), Error> {
        if let Some(shape) = inputs {
            validate_tensor_shapes(interpreter, interpreter.inputs(), shape)
                .context("Invalid inputs")?;
        }
        if let Some(shape) = outputs {
            validate_tensor_shapes(interpreter, interpreter.outputs(), shape)
                .context("Invalid outputs")?;
        }

        Ok(())
    }

    fn validate_tensor_shapes(
        interpreter: &Interpreter<BuiltinOpResolver>,
        tensors: &[i32],
        shapes: &[Shape<'_>],
    ) -> Result<(), Error> {
        anyhow::ensure!(
            tensors.len() == shapes.len(),
            "The model expects {} tensors but the Runefile specified {}",
            tensors.len(),
            shapes.len()
        );

        for (i, (&tensor_index, shape_from_rune)) in
            tensors.iter().zip(shapes).enumerate()
        {
            let tensor_info =
                interpreter.tensor_info(tensor_index).with_context(|| {
                    format!("Unable to find tensor #{} while checking the {}'th tensor", tensor_index, i)
                })?;
            let shape_from_model = tensor_shape(&tensor_info)
                .with_context(|| format!("Tensor {} is invalid", i))?;

            if *shape_from_rune != shape_from_model {
                anyhow::bail!(
                    "The Rune said tensor {} would be a {}, but the model says it is a {}",
                    i,
                    shape_from_rune,
                    shape_from_model,
                );
            }
        }
        todo!()
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

    impl Model for Interpreter<'static, BuiltinOpResolver> {
        fn infer(
            &mut self,
            inputs: &[&[u8]],
            outputs: &mut [&mut [u8]],
        ) -> Result<(), Error> {
            anyhow::ensure!(
                self.inputs().len() == inputs.len(),
                "The model supports {} inputs but {} were provided",
                self.inputs().len(),
                inputs.len(),
            );
            anyhow::ensure!(
                self.outputs().len() == outputs.len(),
                "The model supports {} inputs but {} were provided",
                self.outputs().len(),
                outputs.len(),
            );

            let input_indices: Vec<_> = self.inputs().to_vec();

            for (&ix, &input) in input_indices.iter().zip(inputs) {
                self.tensor_buffer_mut(ix)
                    .context("Unable to get the input buffer")?
                    .copy_from_slice(input);
            }

            self.invoke().context("Calling the model failed")?;

            for (&ix, output) in self.outputs().iter().zip(outputs) {
                let buffer = self
                    .tensor_buffer(ix)
                    .context("Unable to get the output buffer")?;
                output.copy_from_slice(buffer);
            }

            Ok(())
        }
    }
}
