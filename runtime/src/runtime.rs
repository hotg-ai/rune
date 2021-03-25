use crate::{Environment, capability::Capability, outputs::Output};
use anyhow::{Context as _, Error};
use log::{Record, Level};
use runic_types::{SerializableRecord, Value, Type, outputs};
use tflite::{
    FlatBufferModel, Interpreter, InterpreterBuilder,
    ops::builtin::BuiltinOpResolver,
};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt::{self, Debug, Display, Formatter},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
};
use wasmer::{
    Array, BaseTunables, Cranelift, CraneliftOptLevel, Engine, Function,
    ImportObject, Instance, JIT, LazyInit, Memory, Module, NativeFunc, Pages,
    RuntimeError, Store, WASM_MIN_PAGES, WasmPtr,
};

type Models = Arc<Mutex<HashMap<u32, Interpreter<'static, BuiltinOpResolver>>>>;
type Capabilities = Arc<Mutex<HashMap<u32, Box<dyn Capability>>>>;
type Outputs = Arc<Mutex<HashMap<u32, Box<dyn Output>>>>;

pub struct Runtime {
    instance: Instance,
    env: Arc<dyn Environment>,
}

impl Runtime {
    pub fn load<E>(rune: &[u8], env: E) -> Result<Self, Error>
    where
        E: Environment + Send + Sync + 'static,
    {
        log::debug!("Compiling the WebAssembly to native code");

        // we want control over memory usage so we'll need to instantiate the
        // compiler and store manually.
        let mut config = Cranelift::default();
        config.enable_simd(true).opt_level(CraneliftOptLevel::Speed);
        let engine = JIT::new(config).engine();
        let tunables = BaseTunables {
            static_memory_bound: Pages(WASM_MIN_PAGES as u32),
            ..BaseTunables::for_target(engine.target())
        };
        let store = Store::new_with_tunables(&engine, tunables);

        let module = Module::new(&store, rune)
            .context("WebAssembly compilation failed")?;

        Runtime::load_from_module(&module, &store, env)
    }

    pub fn load_from_module<E>(
        module: &Module,
        store: &Store,
        env: E,
    ) -> Result<Self, Error>
    where
        E: Environment + Send + Sync + 'static,
    {
        let env: Arc<dyn Environment> = Arc::new(env);
        let imports = import_object(&store, Arc::clone(&env));
        log::debug!("Instantiating the WebAssembly module");

        let instance =
            Instance::new(&module, &imports).context("Instantiation failed")?;

        // TODO: Rename the _manifest() method to _start() so it gets
        // automatically invoked while instantiating.
        let manifest: NativeFunc<(), i32> = instance
            .exports
            .get_native_function("_manifest")
            .context("Unable to load the _manifest function")?;
        manifest
            .call()
            .context("Unable to call the _manifest function")?;

        log::debug!("Loaded the Rune");

        Ok(Runtime { instance, env })
    }

    pub fn call(&mut self) -> Result<(), Error> {
        log::debug!("Running the rune");

        self.env.before_call();

        let call_func: NativeFunc<(i32, i32, i32), i32> = self
            .instance
            .exports
            .get_native_function("_call")
            .context("Unable to load the _call function")?;

        // For some reason we pass in the RAND capability ID when it's meant
        // to be the Rune's responsibility to remember it. Similarly we are
        // passing in the sine model's output type as the "input_type" parameter
        // even though the model should know that.
        //
        // We should be able to change the _call function's signature once
        // hotg-ai/rune#28 lands.
        call_func
            .call(0, 0, 0)
            .context("Unable to call the _call function")?;

        self.env.after_call();

        Ok(())
    }
}

fn import_object(store: &Store, env: Arc<dyn Environment>) -> ImportObject {
    let ids = Arc::new(Identifiers::new());
    let models = Models::default();
    let capabilities = Capabilities::default();
    let outputs = Outputs::default();

    wasmer::imports! {
        "env" => {
            "_debug" => log(Arc::clone(&env), store),
            "tfm_preload_model" => tfm_preload_model(Arc::clone(&ids), Arc::clone(&models), store),
            "tfm_model_invoke" => tfm_model_invoke(Arc::clone(&models), store),
            "request_capability" => request_capability(Arc::clone(&ids), Arc::clone(&env), Arc::clone(&capabilities), store),
            "request_capability_set_param" => request_capability_set_param(Arc::clone(&capabilities), store),
            "request_provider_response" => request_provider_response(Arc::clone(&capabilities), store),
            "request_output" => request_output(Arc::clone(&ids), Arc::clone(&env), Arc::clone(&outputs), store),
            "consume_output" => consume_output(Arc::clone(&outputs), store),
            "log_backtrace" => log_backtrace(store),
        },
    }
}

#[derive(Debug)]
struct Identifiers(AtomicU32);

impl Identifiers {
    pub const fn new() -> Self { Identifiers(AtomicU32::new(0)) }

    pub fn next(&self) -> u32 { self.0.fetch_add(1, Ordering::SeqCst) }
}

fn log(env: Arc<dyn Environment + 'static>, store: &Store) -> Function {
    #[derive(wasmer::WasmerEnv, Clone)]
    struct State {
        env: Arc<dyn Environment>,
        #[wasmer(export)]
        memory: LazyInit<Memory>,
    }

    let state = State {
        env,
        memory: LazyInit::default(),
    };

    Function::new_native_with_env(
        store,
        state,
        |s: &State, buffer: WasmPtr<u8, Array>, len: u32| unsafe {
            let memory = s.memory.get_unchecked();
            let msg = buffer
                .get_utf8_str(memory, len)
                .unwrap_or_trap("Bad message pointer");

            match serde_json::from_str::<SerializableRecord>(msg) {
                Ok(r) => {
                    r.with_record(|record| s.env.log(record));

                    if r.level == Level::Error {
                        let cause = Error::msg(r.message.into_owned());
                        raise_user_trap(
                            cause.context("Aborting due to fatal error"),
                        );
                    }
                },
                Err(_) => s.env.log(
                    &Record::builder().args(format_args!("{}", msg)).build(),
                ),
            }

            0_u32
        },
    )
}

fn tfm_preload_model(
    ids: Arc<Identifiers>,
    models: Models,
    store: &Store,
) -> Function {
    #[derive(Clone, wasmer::WasmerEnv)]
    struct State {
        ids: Arc<Identifiers>,
        models: Models,
        #[wasmer(export)]
        memory: LazyInit<Memory>,
    }

    let state = State {
        ids,
        models,
        memory: LazyInit::default(),
    };

    Function::new_native_with_env(
        store,
        state,
        |s: &State,
         model: WasmPtr<u8, Array>,
         model_len: u32,
         _inputs: u32,
         _outputs: u32| {
            unsafe {
                let memory = s.memory.get_unchecked();
                let raw = model
                    .deref(memory, 0, model_len)
                    .unwrap_or_trap("Bad pointer");
                let raw: &[u8] = std::mem::transmute(raw);

                let mut models = s.models.lock().unwrap();
                preload_model(raw, &s.ids, &mut models)
                    .unwrap_or_trap("Unable to load the model")
            }
        },
    )
}

fn preload_model(
    raw: &[u8],
    ids: &Identifiers,
    models: &mut HashMap<u32, Interpreter<'static, BuiltinOpResolver>>,
) -> Result<u32, Error> {
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

    let id = ids.next();

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
            "Loaded model {} with inputs {:?} and outputs {:?}",
            id,
            inputs,
            outputs
        );
    }

    models.insert(id, interpreter);

    Ok(id)
}

pub fn tfm_model_invoke(models: Models, store: &Store) -> Function {
    #[derive(Clone, wasmer::WasmerEnv)]
    struct State {
        models: Models,
        #[wasmer(export)]
        memory: LazyInit<Memory>,
    }

    let state = State {
        models,
        memory: LazyInit::default(),
    };

    Function::new_native_with_env(
        store,
        state,
        |s: &State,
         model_id: u32,
         input: WasmPtr<u8, Array>,
         input_len: u32,
         output: WasmPtr<u8, Array>,
         output_len: u32| unsafe {
            let memory = s.memory.get_unchecked();

            let input = input
                .deref(memory, 0, input_len)
                .unwrap_or_trap("Bad input pointer");
            let input: &[u8] = std::mem::transmute(input);

            let output = output
                .deref_mut(memory, 0, output_len)
                .unwrap_or_trap("Bad output pointer");
            let output: &mut [u8] = std::mem::transmute(output);

            let mut models = s.models.lock().unwrap();

            let interpreter =
                models.get_mut(&model_id).unwrap_or_trap("Invalid model");

            invoke_model(model_id, interpreter, input, output)
                .unwrap_or_trap("");

            0
        },
    )
}

fn invoke_model(
    model_index: u32,
    model: &mut Interpreter<BuiltinOpResolver>,
    input: &[u8],
    output: &mut [u8],
) -> Result<(), Error> {
    let tensor_inputs = model.inputs();
    anyhow::ensure!(
        tensor_inputs.len() == 1,
        "We can't handle models with less/more than 1 input"
    );
    let input_index = tensor_inputs[0];

    let buffer = model
        .tensor_buffer_mut(input_index)
        .context("Unable to get the input buffer")?;

    if input.len() != buffer.len() {
        log::warn!(
                "The input vector for model {} is {} bytes long but the tensor expects {}",
                model_index,
                input.len(),
                buffer.len(),
            );
    }
    let len = std::cmp::min(input.len(), buffer.len());
    buffer[..len].copy_from_slice(&input[..len]);

    log::debug!("Model {} received {} bytes", model_index, buffer.len());
    log::trace!("Model {} input: {:?}", model_index, &buffer[..len]);

    model.invoke().context("Calling the model failed")?;

    let tensor_outputs = model.outputs();
    anyhow::ensure!(
        tensor_outputs.len() == 1,
        "We can't handle models with less/more than 1 output"
    );
    let output_index = tensor_outputs[0];
    let buffer = model
        .tensor_buffer(output_index)
        .context("Unable to get the output buffer")?;

    log::debug!("Model {} Output: {:?}", model_index, buffer);

    anyhow::ensure!(buffer.len() == output.len());
    output.copy_from_slice(buffer);

    Ok(())
}

fn request_capability(
    ids: Arc<Identifiers>,
    env: Arc<dyn Environment>,
    caps: Capabilities,
    store: &Store,
) -> Function {
    #[derive(Clone, wasmer::WasmerEnv)]
    struct State {
        ids: Arc<Identifiers>,
        caps: Capabilities,
        env: Arc<dyn Environment>,
        #[wasmer(export)]
        memory: LazyInit<Memory>,
    }

    let state = State {
        ids,
        caps,
        env,
        memory: LazyInit::default(),
    };

    Function::new_native_with_env(
        store,
        state,
        |s: &State, capability_type: u32| {
            let cap = unsafe {
                match capability_type {
                    runic_types::capabilities::ACCEL => {
                        s.env.new_accelerometer().unwrap_or_trap(
                            "Unable to create a accelerometer capability",
                        )
                    },
                    runic_types::capabilities::RAND => s
                        .env
                        .new_random()
                        .unwrap_or_trap("Unable to create a random capability"),
                    runic_types::capabilities::IMAGE => s
                        .env
                        .new_image()
                        .unwrap_or_trap("Unable to create an image capability"),
                    runic_types::capabilities::SOUND => s
                        .env
                        .new_sound()
                        .unwrap_or_trap("Unable to create a sound capability"),
                    _ => raise_user_trap(anyhow::anyhow!(
                        "Unknown capability type, {}",
                        capability_type
                    )),
                }
            };

            let id = s.ids.next();
            log::debug!("Capability {} = {:?}", id, cap);
            s.caps.lock().unwrap().insert(id, cap);

            id
        },
    )
}

fn request_capability_set_param(caps: Capabilities, store: &Store) -> Function {
    #[derive(Clone, wasmer::WasmerEnv)]
    struct State {
        caps: Capabilities,
        #[wasmer(export)]
        memory: LazyInit<Memory>,
    }

    let state = State {
        caps,
        memory: LazyInit::default(),
    };

    Function::new_native_with_env(
        store,
        state,
        |s: &State,
         capability_id: u32,
         key: WasmPtr<u8, Array>,
         key_len: u32,
         value: WasmPtr<u8, Array>,
         value_len: u32,
         value_type: u32| unsafe {
            let memory = s.memory.get_unchecked();
            let key = key
                .get_utf8_str(memory, key_len)
                .unwrap_or_trap("Invalid key");

            let raw = value
                .deref(memory, 0, value_len)
                .unwrap_or_trap("Invalid value");
            let raw: &[u8] = std::mem::transmute(raw);

            let ty = Type::try_from(value_type)
                .map_err(|_| Error::msg("Unknown type"))
                .unwrap_or_trap(
                    "Unable to determine the capability parameter value",
                );

            let value = Value::from_le_bytes(ty, raw)
                .unwrap_or_trap("Unable to unmarshal the parameter value");

            let mut capabilities = s.caps.lock().unwrap();

            log::debug!(
                "Setting \"{}\" to {} on capability {}",
                key,
                value,
                capability_id
            );

            capabilities
                .get_mut(&capability_id)
                .unwrap_or_trap("Invalid capability ID")
                .set_parameter(key, value)
                .unwrap_or_trap_with(|| {
                    format!(
                        "Unable to set capability {}'s \"{}\"",
                        capability_id, key,
                    )
                });

            0_u32
        },
    )
}

fn request_provider_response(caps: Capabilities, store: &Store) -> Function {
    #[derive(Clone, wasmer::WasmerEnv)]
    struct State {
        caps: Capabilities,
        #[wasmer(export)]
        memory: LazyInit<Memory>,
    }

    let state = State {
        caps,
        memory: LazyInit::default(),
    };

    Function::new_native_with_env(
        store,
        state,
        |s: &State,
         buffer: WasmPtr<u8, Array>,
         buffer_len: u32,
         capability_id: u32| unsafe {
            let memory = s.memory.get_unchecked();
            let buffer = buffer
                .deref_mut(memory, 0, buffer_len)
                .unwrap_or_trap("Bad buffer pointer");
            let buffer: &mut [u8] = std::mem::transmute(buffer);

            let mut capabilities = s.caps.lock().unwrap();

            let bytes_written =
                invoke_capability(&mut capabilities, capability_id, buffer)
                    .unwrap_or_trap("Unable to invoke the capability");

            bytes_written as i32
        },
    )
}

fn invoke_capability(
    capabilities: &mut HashMap<u32, Box<dyn Capability>>,
    id: u32,
    dest: &mut [u8],
) -> Result<usize, Error> {
    let cap = unsafe {
        capabilities
            .get_mut(&id)
            .unwrap_or_trap("Invalid capability")
    };

    log::debug!(
        "Invoking capability {} ({:?}) on a {}-byte buffer",
        id,
        cap,
        dest.len()
    );

    cap.generate(dest)
}

fn request_output(
    ids: Arc<Identifiers>,
    env: Arc<dyn Environment>,
    outputs: Outputs,
    store: &Store,
) -> Function {
    #[derive(Clone, wasmer::WasmerEnv)]
    struct State {
        ids: Arc<Identifiers>,
        env: Arc<dyn Environment>,
        outputs: Outputs,
    }

    let state = State { outputs, ids, env };

    Function::new_native_with_env(
        store,
        state,
        |s: &State, output_type: u32| unsafe {
            let output = match output_type {
                outputs::SERIAL => s
                    .env
                    .new_serial()
                    .unwrap_or_trap("Unable to create a new SERIAL output"),
                _ => raise_user_trap(anyhow::anyhow!(
                    "Unknown output type: {}",
                    output_type
                )),
            };

            let id = s.ids.next();
            log::debug!("Output {} = {:?}", id, output);
            s.outputs.lock().unwrap().insert(id, output);

            id
        },
    )
}

fn consume_output(outputs: Outputs, store: &Store) -> Function {
    #[derive(Clone, wasmer::WasmerEnv)]
    struct State {
        outputs: Outputs,
        #[wasmer(export)]
        memory: LazyInit<Memory>,
    }

    let state = State {
        outputs,
        memory: LazyInit::default(),
    };

    Function::new_native_with_env(
        store,
        state,
        |s: &State, id: u32, input: WasmPtr<u8, Array>, len: u32| unsafe {
            let memory = s.memory.get_unchecked();
            let input = input
                .deref(memory, 0, len)
                .unwrap_or_trap("Bad buffer pointer");
            let input: &[u8] = std::mem::transmute(input);

            let mut outputs = s.outputs.lock().unwrap();

            invoke_output(&mut outputs, id, input)
                .unwrap_or_trap("Unable to invoke the output");

            0_u32
        },
    )
}

fn invoke_output(
    outputs: &mut HashMap<u32, Box<dyn Output>>,
    id: u32,
    input: &[u8],
) -> Result<(), Error> {
    let out = unsafe { outputs.get_mut(&id).unwrap_or_trap("Invalid output") };

    log::debug!(
        "Invoking output {} ({:?}) on a {}-byte buffer",
        id,
        out,
        input.len()
    );
    log::trace!("Buffer: {:?}", input);

    out.consume(input)
}

fn log_backtrace(store: &Store) -> Function {
    #[derive(Clone, wasmer::WasmerEnv)]
    struct State {
        #[wasmer(export)]
        memory: LazyInit<Memory>,
    }

    let state = State {
        memory: LazyInit::default(),
    };

    Function::new_native_with_env(
        store,
        state,
        |s: &State, msg: WasmPtr<u8, Array>, len: u32| unsafe {
            let msg = msg
                .get_utf8_str(s.memory.get_unchecked(), len)
                .unwrap_or_trap("Bad message pointer");

            let bt = WebAssemblyBacktrace::capture();
            log::debug!("{} Backtrace\n{}", msg, bt);
        },
    )
}

#[derive(Debug)]
struct WebAssemblyBacktrace {
    err: RuntimeError,
}

impl WebAssemblyBacktrace {
    pub fn capture() -> Self {
        let err = RuntimeError::new("");
        assert!(!err.trace().is_empty());
        WebAssemblyBacktrace { err }
    }
}

impl Display for WebAssemblyBacktrace {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, frame) in self.err.trace().iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
                write!(f, "    at ")?;
            }

            let name = frame.module_name();
            let func_index = frame.func_index();
            match frame.function_name() {
                Some(name) => match rustc_demangle::try_demangle(name) {
                    Ok(name) => write!(f, "{}", name)?,
                    Err(_) => write!(f, "{}", name)?,
                },
                None => write!(f, "<unnamed>")?,
            }
            write!(
                f,
                " ({}[{}]:0x{:x})",
                name,
                func_index,
                frame.module_offset()
            )?;
        }
        Ok(())
    }
}

trait TrapExt<T> {
    unsafe fn unwrap_or_trap_with<F, D>(self, func: F) -> T
    where
        F: FnOnce() -> D,
        D: Display + Debug + Send + Sync + 'static;

    unsafe fn unwrap_or_trap(self, msg: &'static str) -> T
    where
        Self: Sized,
    {
        self.unwrap_or_trap_with(|| msg)
    }
}

impl<T> TrapExt<T> for Option<T> {
    unsafe fn unwrap_or_trap_with<F, D>(self, func: F) -> T
    where
        F: FnOnce() -> D,
        D: Display + Debug + Send + Sync + 'static,
    {
        match self {
            Some(value) => value,
            None => {
                let err = Error::msg(func());
                raise_user_trap(err.into());
            },
        }
    }
}

impl<T, E> TrapExt<T> for Result<T, E>
where
    Result<T, E>: anyhow::Context<T, E>,
{
    unsafe fn unwrap_or_trap_with<F, D>(self, func: F) -> T
    where
        F: FnOnce() -> D,
        D: Display + Debug + Send + Sync + 'static,
    {
        match self.with_context(func) {
            Ok(value) => value,
            Err(err) => {
                raise_user_trap(err.into());
            },
        }
    }
}

unsafe fn raise_user_trap(error: Error) -> ! {
    wasmer::raise_user_trap(error.into());
}

fn _set_max_log_level(instance: &Instance) -> Result<(), Error> {
    let global = instance
        .exports
        .get_global("MAX_LOG_LEVEL")
        .context("Unable to find the MAX_LOG_LEVEL global")?;

    let index: u32 = global
        .get()
        .try_into()
        .map_err(Error::msg)
        .context("The MAX_LOG_LEVEL variable wasn't an integer")?;
    let ptr: WasmPtr<u32> = WasmPtr::new(index);

    let memory = instance
        .exports
        .get_memory("memory")
        .context("Unable to find the main memory")?;

    let cell = ptr.deref(memory).context("Incorrect MAX_LOG_LEVEL index")?;

    let level = log::max_level();
    log::debug!("Setting the MAX_LOG_LEVEL inside the Rune to {:?}", level);
    cell.set(level as u32);
    Ok(())
}
