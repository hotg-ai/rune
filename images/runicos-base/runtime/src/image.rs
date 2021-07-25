use std::{
    cell::Cell,
    collections::HashMap,
    convert::TryInto,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
};
use log::Record;
use anyhow::{Context, Error};
use rune_core::{SerializableRecord, Shape, capabilities, outputs};
use rune_runtime::{
    Capability, Image, Output, common_capabilities::Random,
    common_outputs::Serial,
};
use wasmer::{Array, Function, LazyInit, Memory, RuntimeError, ValueType, WasmPtr};
use rune_wasmer_runtime::Registrar;

const TFLITE_MIMETYPE: &str = "application/tflite-model";

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
}

impl<'vm> Image<rune_wasmer_runtime::Registrar<'vm>> for BaseImage {
    fn initialize_imports(self, registrar: &mut Registrar<'vm>) {
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
            identifiers: identifiers.clone(),
            memory: LazyInit::new(),
        };
        let model_env = ModelEnv {
            factories: Arc::new(models),
            instances: Arc::new(Mutex::new(HashMap::new())),
            identifiers: identifiers.clone(),
            memory: LazyInit::new(),
        };
        let output_env = OutputEnv {
            factories: Arc::new(outputs),
            instances: Arc::new(Mutex::new(HashMap::new())),
            identifiers,
            memory: LazyInit::new(),
        };

        let store = registrar.store();

        registrar
            .register_function(
                "env",
                "_debug",
                Function::new_native_with_env(store, log_env, debug),
            )
            .register_function(
                "env",
                "request_capability",
                Function::new_native_with_env(
                    store,
                    cap_env.clone(),
                    request_capability,
                ),
            )
            .register_function(
                "env",
                "request_capability_set_param",
                Function::new_native_with_env(
                    store,
                    cap_env.clone(),
                    request_capability_set_param,
                ),
            )
            .register_function(
                "env",
                "request_provider_response",
                Function::new_native_with_env(
                    store,
                    cap_env.clone(),
                    request_provider_response,
                ),
            )
            .register_function(
                "env",
                "tfm_model_invoke",
                Function::new_native_with_env(
                    store,
                    model_env.clone(),
                    tfm_model_invoke,
                ),
            )
            .register_function(
                "env",
                "tfm_preload_model",
                Function::new_native_with_env(
                    store,
                    model_env.clone(),
                    tfm_preload_model,
                ),
            )
            .register_function(
                "env",
                "rune_model_load",
                Function::new_native_with_env(
                    store,
                    model_env.clone(),
                    rune_model_load,
                ),
            )
            .register_function(
                "env",
                "rune_model_infer",
                Function::new_native_with_env(
                    store,
                    model_env,
                    rune_model_infer,
                ),
            )
            .register_function(
                "env",
                "request_output",
                Function::new_native_with_env(
                    store,
                    output_env.clone(),
                    request_output,
                ),
            )
            .register_function(
                "env",
                "consume_output",
                Function::new_native_with_env(
                    store,
                    output_env,
                    consume_output,
                ),
            );
    }
}

#[derive(Debug, Default, Clone)]
struct Identifiers(Arc<AtomicU32>);

impl Identifiers {
    fn next(&self) -> u32 { self.0.fetch_add(1, Ordering::SeqCst) }
}

#[derive(Clone, wasmer::WasmerEnv)]
struct OutputEnv {
    factories: Arc<HashMap<u32, Box<dyn OutputFactory>>>,
    instances: Arc<Mutex<HashMap<u32, Box<dyn Output>>>>,
    identifiers: Identifiers,
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

fn request_output(
    env: &OutputEnv,
    output_type: u32,
) -> Result<u32, RuntimeError> {
    let factory = env
        .factories
        .get(&output_type)
        .with_context(|| match rune_core::outputs::name(output_type) {
            Some(name) => {
                format!("No handler registered for output \"{}\"", name)
            },
            None => format!("No handler registered for output {}", output_type),
        })
        .map_err(runtime_error)?;

    let output = factory
        .new_output(None)
        .context("Unable to instantiate the output")
        .map_err(runtime_error)?;

    let id = env.identifiers.next();
    env.instances.lock().unwrap().insert(id, output);

    Ok(id)
}

fn consume_output(
    env: &OutputEnv,
    output_id: u32,
    buffer: WasmPtr<u8, Array>,
    len: u32,
) -> Result<u32, RuntimeError> {
    let mut outputs = env.instances.lock().unwrap();
    let output = outputs
        .get_mut(&output_id)
        .with_context(|| format!("There is no output with ID {}", output_id))
        .map_err(runtime_error)?;

    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    let buffer = buffer
        .deref(memory, 0, len)
        .context("Invalid input")
        .map_err(runtime_error)?;

    // Safety: This function isn't reentrant so there are no concurrent
    // modifications. That also means it's safe to transmute [Cell<T>] to [T].
    let buffer = unsafe {
        std::slice::from_raw_parts(buffer.as_ptr() as *const u8, buffer.len())
    };

    output
        .consume(buffer)
        .context("Unable to consume the data")
        .map_err(runtime_error)?;

    Ok(len)
}

#[derive(Clone, wasmer::WasmerEnv)]
struct ModelEnv {
    factories: Arc<HashMap<String, Box<dyn ModelFactory>>>,
    instances: Arc<Mutex<HashMap<u32, Box<dyn Model>>>>,
    identifiers: Identifiers,
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

fn tfm_model_invoke(
    env: &ModelEnv,
    model_id: u32,
    input: WasmPtr<u8, Array>,
    input_len: u32,
    output: WasmPtr<u8, Array>,
    output_len: u32,
) -> Result<u32, RuntimeError> {
    let mut models = env.instances.lock().unwrap();
    let model = models
        .get_mut(&model_id)
        .with_context(|| format!("There is no model with ID {}", model_id))
        .map_err(runtime_error)?;

    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    let input = input
        .deref(memory, 0, input_len)
        .context("Invalid input")
        .map_err(runtime_error)?;

    let output = output
        .deref(memory, 0, output_len)
        .context("Invalid output")
        .map_err(runtime_error)?;

    model.infer(&[input], &[output]).map_err(runtime_error)?;

    Ok(0)
}

fn rune_model_infer(
    env: &ModelEnv,
    model_id: u32,
    inputs: WasmPtr<WasmPtr<u8, Array>, Array>,
    outputs: WasmPtr<WasmPtr<u8, Array>, Array>,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    let mut models = env.instances.lock().unwrap();
    let model = models
        .get_mut(&model_id)
        .with_context(|| format!("There is no model with ID {}", model_id))
        .map_err(runtime_error)?;

    let inputs = vector_of_tensors(memory, model.input_shapes(), inputs)
        .context("Invalid inputs")
        .map_err(runtime_error)?;
    let outputs = vector_of_tensors(memory, model.output_shapes(), outputs)
        .context("Invalid outputs")
        .map_err(runtime_error)?;

    model
        .infer(&inputs, &outputs)
        .context("Inference failed")
        .map_err(runtime_error)?;

    Ok(0)
}

fn vector_of_tensors<'vm>(
    memory: &'vm Memory,
    shapes: &[Shape<'_>],
    ptr: WasmPtr<WasmPtr<u8, Array>, Array>,
) -> Result<Vec<&'vm [Cell<u8>]>, Error> {
    let pointers = ptr
        .deref(memory, 0, shapes.len() as u32)
        .context("Invalid tensor array pointer")?;

    let mut tensors = Vec::new();

    for (i, ptr) in pointers.iter().enumerate() {
        let ptr = ptr.get();
        let shape = &shapes[i];
        let data = ptr
            .deref(memory, 0, shape.size() as u32)
            .with_context(|| format!("Bad pointer for tensor {}", i))?;
        tensors.push(data);
    }

    Ok(tensors)
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct StringRef {
    data: WasmPtr<u8, Array>,
    len: u32,
}

// Safety: All bit patterns are valid and the wasmer memory will do any
// necessary bounds checks.
unsafe impl ValueType for StringRef {}

fn rune_model_load(
    env: &ModelEnv,
    mimetype: WasmPtr<u8, Array>,
    mimetype_len: u32,
    model: WasmPtr<u8, Array>,
    model_len: u32,
    input_descriptors: WasmPtr<StringRef, Array>,
    input_len: u32,
    output_descriptors: WasmPtr<StringRef, Array>,
    output_len: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: This function isn't reentrant so there are no concurrent
    // modifications. That also means it's safe to transmute [Cell<T>] to [T].
    let (mimetype, model) = unsafe {
        let mimetype = mimetype
            .get_utf8_str(memory, mimetype_len)
            .context("Invalid mimtype string")
            .map_err(runtime_error)?;

        let model = model
            .deref(memory, 0, model_len)
            .context("Invalid model")
            .map_err(runtime_error)?;
        let model = std::slice::from_raw_parts(
            model.as_ptr() as *const u8,
            model.len(),
        );

        (mimetype, model)
    };

    let factory = env
        .factories
        .get(mimetype)
        .with_context(|| {
            format!(
                "No handlers registered for the \"{}\" model type",
                mimetype
            )
        })
        .map_err(runtime_error)?;

    let (inputs, outputs) = unsafe {
        let inputs =
            shape_from_descriptors(memory, input_descriptors, input_len)
                .map_err(runtime_error)?;
        let outputs =
            shape_from_descriptors(memory, output_descriptors, output_len)
                .map_err(runtime_error)?;

        (inputs, outputs)
    };

    let model = factory
        .new_model(model, Some(inputs.as_slice()), Some(outputs.as_slice()))
        .context("Unable to load the model")
        .map_err(runtime_error)?;

    let id = env.identifiers.next();
    env.instances.lock().unwrap().insert(id, model);

    Ok(id)
}

fn tfm_preload_model(
    env: &ModelEnv,
    model: WasmPtr<u8, Array>,
    model_len: u32,
    _: u32,
    _: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: This function isn't reentrant so there are no concurrent
    // modifications. That also means it's safe to transmute [Cell<T>] to [T].
    let model = unsafe {
        let model = model
            .deref(memory, 0, model_len)
            .context("Invalid model")
            .map_err(runtime_error)?;
        std::slice::from_raw_parts(model.as_ptr() as *const u8, model.len())
    };

    let factory = env
        .factories
        .get(TFLITE_MIMETYPE)
        .with_context(|| {
            format!(
                "No handlers registered for the \"{}\" model type",
                TFLITE_MIMETYPE
            )
        })
        .map_err(runtime_error)?;

    let model = factory
        .new_model(model, None, None)
        .context("Unable to instantiate the model")
        .map_err(runtime_error)?;

    let id = env.identifiers.next();
    env.instances.lock().unwrap().insert(id, model);

    Ok(id)
}

/// # Safety
unsafe fn shape_from_descriptors(
    memory: &Memory,
    descriptors: WasmPtr<StringRef, Array>,
    len: u32,
) -> Result<Vec<Shape<'static>>, Error> {
    let descriptors = descriptors
        .deref(memory, 0, len)
        .context("Invalid descriptor pointer")?;

    let mut shapes = Vec::new();

    for (i, descriptor) in descriptors.iter().enumerate() {
        let StringRef { data, len } = descriptor.get();
        let descriptor = data.get_utf8_str(memory, len).with_context(|| {
            format!("The {}'th descriptor pointer is invalid", i)
        })?;
        let shape = descriptor.parse().with_context(|| {
            format!("Unable to parse the {}'th descriptor", i)
        })?;
        shapes.push(shape);
    }

    Ok(shapes)
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
            let cap = f
                .new_capability()
                .with_context(|| {
                    match rune_core::capabilities::name(capability_type) {
                        Some(n) => {
                            format!("Unable to create the \"{}\" capability", n)
                        },
                        None => format!(
                            "Unable to create capability type {}",
                            capability_type
                        ),
                    }
                })
                .map_err(runtime_error)?;
            let id = env.identifiers.next();
            env.instances.lock().unwrap().insert(id, cap);
            Ok(id)
        },
        None => match rune_core::capabilities::name(capability_type) {
            Some(name) => {
                return Err(runtime_error(anyhow::anyhow!(
                    "No \"{}\" capability registered",
                    name
                )));
            },
            None => Err(runtime_error(anyhow::anyhow!(
                "No capability registered for capability type {}",
                capability_type
            ))),
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
) -> Result<u32, RuntimeError> {
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

    Ok(0)
}

fn request_provider_response(
    env: &CapabilityEnv,
    buffer: WasmPtr<u8, Array>,
    len: u32,
    capability_id: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: this function isn't reentrant, so we don't need to worry about
    // concurrent mutations.
    let buffer = unsafe {
        let buffer = buffer
            .deref_mut(memory, 0, len)
            .context("Invalid buffer pointer")
            .map_err(runtime_error)?;

        std::slice::from_raw_parts_mut(
            buffer.as_mut_ptr() as *mut u8,
            buffer.len(),
        )
    };

    env.instances
        .lock()
        .unwrap()
        .get_mut(&capability_id)
        .context("No such capability")
        .and_then(|c| c.generate(buffer))
        .map_err(runtime_error)?;

    Ok(buffer.len() as u32)
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
) -> Result<u32, RuntimeError> {
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

        log::debug!("Received message: {}", message);

        if let Ok(record) = serde_json::from_str::<SerializableRecord>(message)
        {
            record
                .with_record(|r| (env.log)(r))
                .context("Logging failed")
                .map_err(runtime_error)?;
        } else {
            log::warn!("{}", message);
        }
    }

    Ok(0)
}

impl Default for BaseImage {
    fn default() -> Self { BaseImage::new() }
}

pub trait Model: Send + Sync + 'static {
    fn infer(
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
        fn infer(
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

                for (src, dest) in input.iter().zip(buffer) {
                    *dest = src.get();
                }
            }

            interpreter.invoke().context("Calling the model failed")?;

            for (&ix, output) in interpreter.outputs().iter().zip(outputs) {
                let buffer = interpreter
                    .tensor_buffer(ix)
                    .context("Unable to get the output buffer")?;

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

#[cfg(test)]
mod tests {
    use syn::{ForeignItem, ForeignItemFn, Item};
    use wasmer::{Export, Store};
    use super::*;

    fn extern_functions(src: &str) -> impl Iterator<Item = ForeignItemFn> {
        let module: syn::File = syn::parse_str(src).unwrap();

        module
            .items
            .into_iter()
            .filter_map(|it| match it {
                Item::ForeignMod(e) => Some(e.items.into_iter()),
                _ => None,
            })
            .flatten()
            .filter_map(|item| match item {
                ForeignItem::Fn(f) => Some(f),
                _ => None,
            })
    }

    #[test]
    fn all_intrinsics_are_registered() {
        let store = Store::default();
        let intrinsics_rs = include_str!("../../wasm/src/intrinsics.rs");
        let intrinsics = extern_functions(intrinsics_rs).map(|f| f.sig);
        let mut registrar = Registrar::new(&store);

        BaseImage::default().initialize_imports(&mut registrar);

        let imports = registrar.into_import_object();

        for intrinsic in intrinsics {
            let name = intrinsic.ident.to_string();
            let got = imports.get_export("env", &name).expect(&name);

            let got = match got {
                Export::Function(f) => f,
                other => panic!("\"{}\" was a {:?}", name, other),
            };
            let host_function_signature = &got.vm_function.signature;
            assert_eq!(
                intrinsic.inputs.len(),
                host_function_signature.params().len(),
                "parameters for \"{}\" are mismatched",
                name,
            );
        }
    }
}
