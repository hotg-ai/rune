use crate::{
    Environment,
    capability::{CapabilityParam, CapabilityRequest},
};
use anyhow::{Context as _, Error};
use log::Level;
use runic_types::{CAPABILITY, PARAM_TYPE};
use tflite::{
    FlatBufferModel, Interpreter, InterpreterBuilder,
    ops::builtin::BuiltinOpResolver,
};
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
};
use wasmer::{
    Array, Function, ImportObject, Instance, LazyInit, Memory, Module,
    NativeFunc, Store, WasmPtr,
};

type Models = Arc<Mutex<HashMap<u32, Interpreter<'static, BuiltinOpResolver>>>>;
type Capabilities = Arc<Mutex<HashMap<u32, CapabilityRequest>>>;

pub struct Runtime {
    instance: Instance,
}

impl Runtime {
    pub fn load<E>(rune: &[u8], env: E) -> Result<Self, Error>
    where
        E: Environment + Send + Sync + 'static,
    {
        log::debug!("Compiling the WebAssembly to native code");
        let store = Store::default();
        let module = Module::new(&store, rune)
            .context("WebAssembly compilation failed")?;

        let imports = import_object(&store, env);
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

        Ok(Runtime { instance })
    }

    pub fn call(&mut self) -> Result<(), Error> {
        log::debug!("Running the rune");

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
            .call(CAPABILITY::RAND as i32, PARAM_TYPE::FLOAT as i32, 2)
            .context("Unable to call the _call function")?;

        Ok(())
    }
}

fn import_object(store: &Store, env: impl Environment) -> ImportObject {
    let env: Arc<dyn Environment> = Arc::new(env);
    let ids = Arc::new(Identifiers::new());
    let models = Arc::new(Mutex::new(HashMap::new()));
    let capabilities = Arc::new(Mutex::new(HashMap::new()));

    wasmer::imports! {
        "env" => {
            "_debug" => log(Arc::clone(&env), store),
            "tfm_preload_model" => tfm_preload_model(Arc::clone(&ids), Arc::clone(&models), store),
            "tfm_model_invoke" => tfm_model_invoke(Arc::clone(&models), store),
            "request_capability" => request_capability(Arc::clone(&ids), Arc::clone(&capabilities), store),
            "request_capability_set_param" => request_capability_set_param(Arc::clone(&capabilities), store),
            "request_provider_response" => request_provider_response(Arc::clone(&env), Arc::clone(&capabilities), store),
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

            s.env.log(msg);

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

            let _output = invoke_model(model_id, interpreter, input, output)
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

    log::debug!("Model {} input: {:?}", model_index, &buffer[..len]);

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
    caps: Capabilities,
    store: &Store,
) -> Function {
    #[derive(Clone, wasmer::WasmerEnv)]
    struct State {
        ids: Arc<Identifiers>,
        caps: Capabilities,
        #[wasmer(export)]
        memory: LazyInit<Memory>,
    }

    let state = State {
        ids,
        caps,
        memory: LazyInit::default(),
    };

    Function::new_native_with_env(
        store,
        state,
        |s: &State, capability_type: u32| {
            let cap = runic_types::CAPABILITY::from_u32(capability_type);
            let request = CapabilityRequest::new(cap);

            let id = s.ids.next();
            s.caps.lock().unwrap().insert(id, request);

            log::debug!("Requested capability {:?} with ID {}", cap, id);

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

            let value = value
                .deref(memory, 0, value_len)
                .unwrap_or_trap("Invalid value");
            let value: &[u8] = std::mem::transmute(value);

            let mut capabilities = s.caps.lock().unwrap();
            set_capability_parameter(
                capability_id,
                &mut capabilities,
                key,
                value,
                PARAM_TYPE::from_u32(value_type),
            )
            .unwrap_or_trap("Unable to set the capability parameter");

            0_u32
        },
    )
}

fn set_capability_parameter(
    id: u32,
    capabilities: &mut HashMap<u32, CapabilityRequest>,
    key: &str,
    value: &[u8],
    ty: PARAM_TYPE,
) -> Result<(), Error> {
    let request = capabilities.get_mut(&id).context("Invalid capability")?;
    let value = CapabilityParam::from_raw(value.to_vec(), ty)
        .context("Invalid capability parameter")?;

    log::debug!("Setting {}={:?} on capability {}", key, value, id);
    request.params.insert(key.to_string(), value);

    Ok(())
}

fn request_provider_response(
    env: Arc<dyn Environment>,
    caps: Capabilities,
    store: &Store,
) -> Function {
    #[derive(Clone, wasmer::WasmerEnv)]
    struct State {
        env: Arc<dyn Environment>,
        caps: Capabilities,
        #[wasmer(export)]
        memory: LazyInit<Memory>,
    }

    let state = State {
        caps,
        env,
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

            invoke_capability(
                &mut capabilities,
                capability_id,
                &*s.env,
                buffer,
            )
            .unwrap_or_trap("Unable to invoke the capability");

            buffer.len() as u32
        },
    )
}

fn invoke_capability(
    capabilities: &mut HashMap<u32, CapabilityRequest>,
    id: u32,
    env: &dyn Environment,
    dest: &mut [u8],
) -> Result<(), Error> {
    log::debug!("Getting capability {}", id);
    let cap =
        unsafe { capabilities.get(&id).unwrap_or_trap("Invalid capability") };

    log::debug!(
        "Invoking capability {} ({:?}) on a {}-byte buffer",
        id,
        cap.c_type,
        dest.len()
    );

    match cap.c_type {
        runic_types::CAPABILITY::RAND => {
            env.fill_random(dest)
                .context("Unable to get random bytes")?;

            log::debug!("Rand: {:?}", dest);

            Ok(())
        },
        runic_types::CAPABILITY::ACCEL => {
            let buffer = unsafe {
                // HACK: We've been given a byte array but accelerometer data
                // comes as XYZ floats. Float arrays are POD types so it's okay
                // to transmute them.
                //
                // This wouldn't be necessary if each capability had its own
                // host function with a strongly typed signature.
                let len = dest.len() / std::mem::size_of::<[f32; 3]>();
                std::slice::from_raw_parts_mut(
                    dest.as_mut_ptr() as *mut [f32; 3],
                    len,
                )
            };
            env.fill_accelerometer(buffer)
                .context("Unable to fill the buffer with accelerometer data")?;

            Ok(())
        },
        runic_types::CAPABILITY::IMAGE => {
            env.fill_image(dest, 0, 0)
                .context("Unable to fill the buffer with image data")?;

            Ok(())
        },
        runic_types::CAPABILITY::SOUND => {
            let buffer = unsafe {
                // HACK: We've been given a byte array but audio data comes as
                // PCM-encoded i16. Integer arrays are POD types so it's okay to
                // transmute them.
                //
                // This wouldn't be necessary if each capability had its own
                // host function with a strongly typed signature.
                let len = dest.len() / std::mem::size_of::<i16>();
                std::slice::from_raw_parts_mut(
                    dest.as_mut_ptr() as *mut i16,
                    len,
                )
            };
            env.fill_audio(buffer)
                .context("Unable to fill the buffer with audio data")?;

            Ok(())
        },
        other => Err(anyhow::anyhow!(
            "The {:?} capability isn't implemented",
            other
        )),
    }
}

trait TrapExt<T> {
    unsafe fn unwrap_or_trap(self, msg: &'static str) -> T;
}

impl<T> TrapExt<T> for Option<T> {
    unsafe fn unwrap_or_trap(self, msg: &'static str) -> T {
        match self {
            Some(value) => value,
            None => {
                let err = Error::msg(msg);
                wasmer::raise_user_trap(err.into());
            },
        }
    }
}

impl<T, E> TrapExt<T> for Result<T, E>
where
    Result<T, E>: anyhow::Context<T, E>,
{
    unsafe fn unwrap_or_trap(self, msg: &'static str) -> T {
        match self.context(msg) {
            Ok(value) => value,
            Err(e) => {
                let err = e.context(msg);
                wasmer::raise_user_trap(err.into())
            },
        }
    }
}
