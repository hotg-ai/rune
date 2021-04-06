use std::{
    collections::HashMap,
    convert::TryFrom,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
};
use log::{Level, Record};
use rune_runtime::{CallContext, Capability, Function, Image, Output, Registrar};
use anyhow::{Context, Error};
use runic_types::{SerializableRecord, Type, Value};

type OutputFactory =
    dyn Fn() -> Result<Box<dyn Output>, Error> + Send + Sync + 'static;
type CapabilityFactory =
    dyn Fn() -> Result<Box<dyn Capability>, Error> + Send + Sync + 'static;
type LogFunc = dyn Fn(&Record) -> Result<(), Error> + Sync + Send + 'static;
type ModelFactory =
    dyn Fn(&[u8]) -> Result<Box<dyn Model>, Error> + Send + Sync + 'static;

#[derive(Clone)]
pub struct BaseImage {
    accelerometer: Arc<CapabilityFactory>,
    image: Arc<CapabilityFactory>,
    log: Arc<LogFunc>,
    model: Arc<ModelFactory>,
    rand: Arc<CapabilityFactory>,
    serial: Arc<OutputFactory>,
    sound: Arc<CapabilityFactory>,
}

impl BaseImage {
    pub fn new() -> Self { BaseImage::default() }

    pub fn with_log<F>(&mut self, log: F) -> &mut Self
    where
        F: Fn(&Record) -> Result<(), Error> + Sync + Send + 'static,
    {
        self.log = Arc::new(log);
        self
    }

    pub fn with_rand<F>(&mut self, rand: F) -> &mut Self
    where
        F: Fn() -> Result<Box<dyn Capability>, Error> + Send + Sync + 'static,
    {
        self.rand = Arc::new(rand);
        self
    }

    pub fn with_accelerometer<F>(&mut self, accelerometer: F) -> &mut Self
    where
        F: Fn() -> Result<Box<dyn Capability>, Error> + Send + Sync + 'static,
    {
        self.accelerometer = Arc::new(accelerometer);
        self
    }

    pub fn with_sound<F>(&mut self, sound: F) -> &mut Self
    where
        F: Fn() -> Result<Box<dyn Capability>, Error> + Send + Sync + 'static,
    {
        self.sound = Arc::new(sound);
        self
    }

    pub fn with_image<F>(&mut self, image: F) -> &mut Self
    where
        F: Fn() -> Result<Box<dyn Capability>, Error> + Send + Sync + 'static,
    {
        self.image = Arc::new(image);
        self
    }

    pub fn with_serial<F>(&mut self, serial: F) -> &mut Self
    where
        F: Fn() -> Result<Box<dyn Output>, Error> + Send + Sync + 'static,
    {
        self.serial = Arc::new(serial);
        self
    }

    pub fn with_model<F>(&mut self, model: F) -> &mut Self
    where
        F: Fn(&[u8]) -> Result<Box<dyn Model>, Error> + Send + Sync + 'static,
    {
        self.model = Arc::new(model);
        self
    }
}

impl Default for BaseImage {
    fn default() -> Self {
        BaseImage {
            accelerometer: Arc::new(|| anyhow::bail!("Unsupported")),
            image: Arc::new(|| anyhow::bail!("Unsupported")),
            rand: Arc::new(|| anyhow::bail!("Unsupported")),
            serial: Arc::new(initialize_serial_output),
            sound: Arc::new(|| anyhow::bail!("Unsupported")),
            model: Arc::new(initialize_model),
            log: Arc::new(|record| {
                log::logger().log(record);
                Ok(())
            }),
        }
    }
}

impl Image for BaseImage {
    fn initialize_imports(self, registrar: &mut dyn Registrar) {
        let ids = Identifiers::default();
        let outputs = Arc::new(Mutex::new(HashMap::new()));
        let capabilities = Arc::new(Mutex::new(HashMap::new()));
        let models = Arc::new(Mutex::new(HashMap::new()));

        registrar.register_function("env", "_debug", log(&self.log));

        let output_factories = Outputs {
            serial: Arc::clone(&self.serial),
        };
        registrar.register_function(
            "env",
            "request_output",
            request_output(&ids, &outputs, output_factories),
        );
        registrar.register_function(
            "env",
            "consume_output",
            consume_output(&outputs),
        );

        let capability_factories = Capabilities {
            rand: Arc::clone(&self.rand),
            accel: Arc::clone(&self.accelerometer),
            image: Arc::clone(&self.image),
            sound: Arc::clone(&self.sound),
        };
        registrar.register_function(
            "env",
            "request_capability",
            request_capability(&ids, &capabilities, capability_factories),
        );
        registrar.register_function(
            "env",
            "request_capability_set_param",
            request_capability_set_param(&capabilities),
        );
        registrar.register_function(
            "env",
            "request_provider_response",
            request_provider_response(&capabilities),
        );

        registrar.register_function(
            "env",
            "tfm_preload_model",
            tfm_preload_model(&ids, &models, &self.model),
        );
        registrar.register_function(
            "env",
            "tfm_model_invoke",
            tfm_model_invoke(&models),
        );
    }
}

fn consume_output(
    outputs: &Arc<Mutex<HashMap<u32, Box<dyn Output>>>>,
) -> Function {
    let outputs = Arc::clone(outputs);

    Function::new(
        move |ctx: &dyn CallContext, (id, address, len): (u32, u32, u32)| {
            let data = ctx.memory(address, len).context("Bad input pointer")?;
            let mut outputs = outputs.lock().unwrap();
            let output = outputs.get_mut(&id).context("Invalid output")?;
            output.consume(data)?;

            Ok(0)
        },
    )
}

struct Outputs {
    serial: Arc<OutputFactory>,
}

fn log(log: &Arc<LogFunc>) -> Function {
    let log = Arc::clone(log);

    Function::new(move |ctx: &dyn CallContext, (msg, len): (u32, u32)| {
        let msg = ctx.utf8_str(msg, len)?;

        // this is a little more verbose than normal because we want to try
        // *really* hard to log messages and abort on errors.
        match serde_json::from_str::<SerializableRecord>(msg) {
            Ok(r) => {
                r.with_record(|record| log(record))?;

                if r.level == Level::Error {
                    // Make sure we abort on error
                    return Err(Error::msg(r.message.into_owned()));
                }
            },
            Err(_) => {
                log(&Record::builder().args(format_args!("{}", msg)).build())?;
            },
        };

        Ok(0)
    })
}

#[derive(Default, Debug, Clone)]
struct Identifiers(Arc<AtomicU32>);

impl Identifiers {
    fn next(&self) -> u32 { self.0.fetch_add(1, Ordering::SeqCst) }
}

fn request_output(
    ids: &Identifiers,
    outputs: &Arc<Mutex<HashMap<u32, Box<dyn Output>>>>,
    constructors: Outputs,
) -> Function {
    let ids = ids.clone();
    let outputs = Arc::clone(outputs);

    Function::new(move |_: &dyn CallContext, (output_type,): (u32,)| {
        let output = match output_type {
            runic_types::outputs::SERIAL => (constructors.serial)()
                .context("Unable to create a new SERIAL output")?,
            _ => anyhow::bail!("Unknown output type: {}", output_type),
        };

        let id = ids.next();
        log::debug!("Output {} = {:?}", id, output);
        outputs.lock().unwrap().insert(id, output);

        Ok(id)
    })
}

struct Capabilities {
    accel: Arc<CapabilityFactory>,
    image: Arc<CapabilityFactory>,
    rand: Arc<CapabilityFactory>,
    sound: Arc<CapabilityFactory>,
}

fn request_capability(
    ids: &Identifiers,
    capabilities: &Arc<Mutex<HashMap<u32, Box<dyn Capability>>>>,
    factories: Capabilities,
) -> Function {
    let ids = ids.clone();
    let capabilities = Arc::clone(capabilities);

    Function::new(move |_, capability_type: u32| {
        let cap = match capability_type {
            runic_types::capabilities::ACCEL => (factories.accel)()?,
            runic_types::capabilities::IMAGE => (factories.image)()?,
            runic_types::capabilities::RAND => (factories.rand)()?,
            runic_types::capabilities::SOUND => (factories.sound)()?,
            _ => anyhow::bail!("Unknown capability type: {}", capability_type),
        };

        let id = ids.next();
        capabilities.lock().unwrap().insert(id, cap);
        Ok(id)
    })
}

fn request_capability_set_param(
    capabilities: &Arc<Mutex<HashMap<u32, Box<dyn Capability>>>>,
) -> Function {
    let capabilities = Arc::clone(capabilities);

    Function::new(
        move |ctx,
              (
            capability_id,
            key_ptr,
            key_len,
            value_ptr,
            value_len,
            value_type,
        ): (u32, u32, u32, u32, u32, u32)| {
            let mut capabilities = capabilities.lock().unwrap();
            let capability = capabilities
                .get_mut(&capability_id)
                .context("Unknown capability")?;

            let key = ctx
                .utf8_str(key_ptr, key_len)
                .context("Unable to read the key")?;
            let value = ctx
                .memory(value_ptr, value_len)
                .context("Unable to read the value")?;

            let value_type = Type::try_from(value_type)
                .map_err(|_| Error::msg("Invalid value type"))?;
            let value = Value::from_le_bytes(value_type, value)
                .context("Unable to unmarshal the value")?;

            capability
                .set_parameter(key, value)
                .context("Unable to set the parameter")?;

            Ok(0)
        },
    )
}

fn request_provider_response(
    capabilities: &Arc<Mutex<HashMap<u32, Box<dyn Capability>>>>,
) -> Function {
    let capabilities = Arc::clone(capabilities);

    Function::new(
        move |ctx, (buffer, buffer_len, capability_id): (u32, u32, u32)| {
            let mut capabilities = capabilities.lock().unwrap();
            let capability = capabilities
                .get_mut(&capability_id)
                .context("Unknown capability")?;

            unsafe {
                let buffer = ctx
                    .memory_mut(buffer, buffer_len)
                    .context("Unable to read the buffer")?;

                let bytes_written = capability.generate(buffer)?;
                Ok(bytes_written as u32)
            }
        },
    )
}

pub trait Model: Send + Sync + 'static {
    fn infer(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error>;
}

fn tfm_preload_model(
    ids: &Identifiers,
    models: &Arc<Mutex<HashMap<u32, Box<dyn Model>>>>,
    constructor: &Arc<ModelFactory>,
) -> Function {
    let ids = ids.clone();
    let models = Arc::clone(models);
    let constructor = Arc::clone(constructor);

    Function::new(
        move |ctx,
         (
        model,
        model_len,
        _inputs,
        _outputs,
): (
            u32,
            u32,
            u32,
            u32,
        )| {
            let model = ctx.memory(model, model_len)
            .context("Invalid model buffer")?;
            let model = constructor(model).context("Unable to create the model")?;

            let mut models = models.lock().unwrap();
            let id = ids.next();
            models.insert(id, model);

            Ok(id)
        },
    )
}

fn tfm_model_invoke(
    models: &Arc<Mutex<HashMap<u32, Box<dyn Model>>>>,
) -> Function {
    let models = Arc::clone(models);

    Function::new(
        move |ctx,
              (model_id, input, input_len, output, output_len): (
            u32,
            u32,
            u32,
            u32,
            u32,
        )| {
            let mut models = models.lock().unwrap();
            let model = models.get_mut(&model_id).context("Invalid model")?;

            let input = ctx
                .memory(input, input_len)
                .context("Invalid input buffer")?;

            let output = unsafe {
                ctx.memory_mut(output, output_len)
                    .context("Invalid output buffer")?
            };

            model
                .infer(input, output)
                .context("Unable to execute the model")?;

            Ok(0)
        },
    )
}

#[cfg(feature = "tflite")]
fn initialize_model(raw: &[u8]) -> Result<Box<dyn Model>, Error> {
    use tflite::{
        FlatBufferModel, InterpreterBuilder, ops::builtin::BuiltinOpResolver,
    };

    let model = FlatBufferModel::build_from_buffer(raw.to_vec())
        .context("Unable to build the model")?;

    let resolver = BuiltinOpResolver::default();

    let builder = InterpreterBuilder::new(model, resolver)
        .context("Unable to create a model interpreter builder")?;
    let mut interpreter = builder
        .build()
        .context("Unable to initialize the model interpreter")?;
    interpreter
        .allocate_tensors()
        .context("Unable to allocate tensors")?;

    if log::log_enabled!(Level::Debug) {
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

    Ok(Box::new(interpreter))
}

#[cfg(feature = "tflite")]
impl Model
    for tflite::Interpreter<'static, tflite::ops::builtin::BuiltinOpResolver>
{
    fn infer(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error> {
        let tensor_inputs = self.inputs();
        anyhow::ensure!(
            tensor_inputs.len() == 1,
            "We can't handle models with less/more than 1 input"
        );
        let input_index = tensor_inputs[0];

        let buffer = self
            .tensor_buffer_mut(input_index)
            .context("Unable to get the input buffer")?;

        if input.len() != buffer.len() {
            log::warn!(
                "The input vector for the model is {} bytes long but the tensor expects {}",
                input.len(),
                buffer.len(),
            );
        }
        let len = std::cmp::min(input.len(), buffer.len());
        buffer[..len].copy_from_slice(&input[..len]);

        log::debug!("Model received {} bytes", buffer.len());
        log::trace!("Model input: {:?}", &buffer[..len]);

        self.invoke().context("Calling the model failed")?;

        let tensor_outputs = self.outputs();
        anyhow::ensure!(
            tensor_outputs.len() == 1,
            "We can't handle models with less/more than 1 output"
        );
        let output_index = tensor_outputs[0];
        let buffer = self
            .tensor_buffer(output_index)
            .context("Unable to get the output buffer")?;

        log::debug!("Model Output: {:?}", buffer);

        anyhow::ensure!(buffer.len() == output.len());
        output.copy_from_slice(buffer);

        Ok(())
    }
}

#[cfg(not(feature = "tflite"))]
fn initialize_model(raw: &[u8]) -> Result<Box<dyn Model>, Error> {
    anyhow::bail!("Unsupported")
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct SerialOutput;

impl Output for SerialOutput {
    fn consume(&mut self, buffer: &[u8]) -> Result<(), Error> {
        let json = std::str::from_utf8(buffer)
            .context("Unable to parse the input as UTF-8")?;

        log::info!("Serial: {}", json);

        Ok(())
    }
}

fn initialize_serial_output() -> Result<Box<dyn Output>, Error> {
    Ok(Box::new(SerialOutput::default()))
}

#[cfg(test)]
mod tests {
    use std::{cell::UnsafeCell, collections::HashSet};
    use rune_runtime::WasmValue;
    use super::*;

    #[derive(Default, Debug)]
    struct DummyOutput(Arc<AtomicU32>);

    impl rune_runtime::Output for DummyOutput {
        fn consume(&mut self, _: &[u8]) -> Result<(), Error> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    struct DummyCallContext {
        memory: UnsafeCell<Vec<u8>>,
    }

    impl DummyCallContext {
        pub fn with_data(address: usize, data: &[u8]) -> Self {
            let mut memory = vec![0; address + data.len()];
            memory[address..address + data.len()].copy_from_slice(data);

            DummyCallContext {
                memory: UnsafeCell::new(memory),
            }
        }
    }

    impl CallContext for DummyCallContext {
        fn memory(&self, address: u32, len: u32) -> Result<&[u8], Error> {
            let start = address as usize;
            let end = start + len as usize;
            let memory = unsafe { &*self.memory.get() };
            memory.get(start..end).context("Out of bounds")
        }

        unsafe fn memory_mut(
            &self,
            address: u32,
            len: u32,
        ) -> Result<&mut [u8], Error> {
            let start = address as usize;
            let end = start + len as usize;
            let memory = &mut *self.memory.get();
            memory.get_mut(start..end).context("Out of bounds")
        }
    }

    #[test]
    fn invoke_output() {
        let calls = Arc::new(AtomicU32::new(0));
        let ctx = DummyCallContext::default();
        let d = DummyOutput(Arc::clone(&calls));
        let mut outputs = HashMap::new();
        outputs.insert(3_u32, Box::new(d) as Box<dyn Output>);
        let outputs = Arc::new(Mutex::new(outputs));
        let func = consume_output(&outputs);

        let ret = func
            .call(
                &ctx,
                &[WasmValue::I32(3), WasmValue::I32(0), WasmValue::I32(0)],
            )
            .unwrap();

        assert!(ret.is_empty());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    fn mock_log() -> (Arc<Mutex<Vec<SerializableRecord<'static>>>>, Arc<LogFunc>)
    {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let calls2 = Arc::clone(&calls);

        let log_func: Arc<LogFunc> = Arc::new(move |r: &Record| {
            let record = SerializableRecord::from(r).into_owned();
            calls2.lock().unwrap().push(record);
            Ok(())
        });

        (calls, log_func)
    }

    #[test]
    fn call_log_function() {
        let record = SerializableRecord {
            message: "Hello, world".into(),
            ..Default::default()
        };
        let serialized = serde_json::to_vec(&record).unwrap();
        let ctx = DummyCallContext::with_data(42, &serialized);
        let (calls, log_func) = mock_log();
        let func = log(&log_func);
        let args =
            &[WasmValue::I32(42), WasmValue::I32(serialized.len() as i32)];

        let ret = func.call(&ctx, args).unwrap();

        assert!(ret.is_empty());
        let got = calls.lock().unwrap();
        assert_eq!(got.len(), 1);
        assert_eq!(&got[0], &record);
    }

    #[derive(Debug, Default)]
    struct Registrar {
        functions: HashMap<(String, String), Function>,
    }

    impl Registrar {
        fn keys(&self) -> HashSet<(&str, &str)> {
            let mut keys = HashSet::new();

            for (ns, name) in self.functions.keys() {
                keys.insert((ns.as_str(), name.as_str()));
            }

            keys
        }
    }

    impl rune_runtime::Registrar for Registrar {
        fn register_function(
            &mut self,
            namespace: &str,
            name: &str,
            function: Function,
        ) {
            self.functions
                .insert((namespace.to_string(), name.to_string()), function);
        }
    }

    #[test]
    fn functions_are_registered() {
        let mut registrar = Registrar::default();
        let image = BaseImage::default();

        image.initialize_imports(&mut registrar);

        let got = registrar.keys();
        let should_be: HashSet<_> = vec![
            ("env", "_debug"),
            ("env", "consume_output"),
            ("env", "request_capability_set_param"),
            ("env", "request_capability"),
            ("env", "request_output"),
            ("env", "request_provider_response"),
            ("env", "tfm_model_invoke"),
            ("env", "tfm_preload_model"),
        ]
        .into_iter()
        .collect();
        assert_eq!(got, should_be);
    }
}
