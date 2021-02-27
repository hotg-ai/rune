use crate::{context::Context, Environment};
use anyhow::{Context as _, Error};
use runic_types::{CAPABILITY, OUTPUT, PARAM_TYPE};
use std::{
    ffi::c_void,
    fmt::{self, Display, Formatter},
    sync::Mutex,
};
use wasmer_runtime::{
    error::{
        CallError, Error as WasmerError, LinkError, RuntimeError, RuntimeResult,
    },
    func, Array, Ctx, Func, ImportObject, Instance, WasmPtr,
};

pub struct Runtime {
    instance: Instance,
}

impl Runtime {
    pub fn load<E>(rune: &[u8], env: E) -> Result<Self, Error>
    where
        E: Environment + Send + Sync + 'static,
    {
        let module = wasmer_runtime::compile(rune)
            .context("WebAssembly compilation failed")?;
        let imports = onetime_import_object(env);
        let instance = module
            .instantiate(&imports)
            .map_err(|e| match e {
                WasmerError::CompileError(c) => Error::from(c),
                WasmerError::LinkError(l) => Error::from(LinkErrors(l)),
                WasmerError::RuntimeError(r) => runtime_error(r),
                WasmerError::ResolveError(r) => Error::from(r),
                WasmerError::CallError(CallError::Resolve(r)) => Error::from(r),
                WasmerError::CallError(CallError::Runtime(r)) => {
                    runtime_error(r)
                },
                WasmerError::CreationError(c) => Error::from(c),
            })
            .context("Instantiation failed")?;

        // TODO: Rename the _manifest() method to _start() so it gets
        // automatically invoked while instantiating.
        let manifest: Func<(), i32> = instance
            .exports
            .get("_manifest")
            .context("Unable to load the _manifest function")?;
        manifest
            .call()
            .map_err(runtime_error)
            .context("Unable to call the _manifest function")?;

        Ok(Runtime { instance })
    }

    pub fn call(&mut self) -> Result<(), Error> {
        log::debug!("Running the rune");

        let call_func: Func<(i32, i32, i32), i32> = self
            .instance
            .exports
            .get("_call")
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
            .map_err(runtime_error)
            .context("Unable to call the _call function")?;

        Ok(())
    }
}

fn onetime_import_object<E>(env: E) -> ImportObject
where
    E: Environment + Send + Sync + 'static,
{
    let env = Mutex::new(Some(env));

    import_object(move || {
        env.lock()
            .unwrap()
            .take()
            .expect("Initializer should only ever be called once")
    })
}

/// Create a new [`ImportObject`] using a closure that will instantiate a new
/// [`Environment`] for every new WebAssembly [`Instance`].
fn import_object<F, E>(constructor: F) -> ImportObject
where
    F: Fn() -> E + Send + Sync + 'static,
    E: Environment,
{
    wasmer_runtime::imports! {
        move || {
            let env = constructor();

            let ctx = Box::new(Context::new(env));
            let free = |data: *mut c_void| unsafe {
                let _ = Box::from_raw(data as *mut Context<E>);
            };

            (Box::into_raw(ctx) as *mut c_void, free)
        },

        "env" => {
            "_debug" => func!(log::<E>),
            "_abort" => func!(abort::<E>),
            "tfm_model_invoke" => func!(tfm_model_invoke::<E>),
            "tfm_preload_model" => func!(tfm_preload_model::<E>),
            "request_capability" => func!(request_capability::<E>),
            "request_capability_set_param" => func!(request_capability_set_param::<E>),
            "request_manifest_output" => func!(request_manifest_output::<E>),
            "request_provider_response" => func!(request_provider_response::<E>)
        },
    }
}

fn log<E>(
    ctx: &mut Ctx,
    buffer: WasmPtr<u8, Array>,
    len: u32,
) -> RuntimeResult<u32>
where
    E: Environment,
{
    let (mem, context) = unsafe { ctx.memory_and_data_mut::<Context<E>>(0) };

    match buffer.get_utf8_string(mem, len) {
        Some(msg) => {
            context.log(msg);
            // FIXME: We should just return () here because logging isn't
            // fallible.
            Ok(0)
        },
        None => Err(RuntimeError::User(Box::new(Error::msg("")))),
    }
}

fn abort<E>(
    ctx: &mut Ctx,
    msg: WasmPtr<u8, Array>,
    msg_len: u32,
    file: WasmPtr<u8, Array>,
    file_len: u32,
    line: u32,
) -> RuntimeResult<()>
where
    E: Environment,
{
    let mem = ctx.memory(0);

    let msg = msg
        .get_utf8_string(mem, msg_len)
        .ok_or_else(|| runtime_err("Unable to retrieve the abort message"))?;
    let file = file.get_utf8_string(mem, file_len).ok_or_else(|| {
        runtime_err("Unable to retrieve the abort line number")
    })?;

    Err(RuntimeError::User(Box::new(Error::new(Abort {
        message: msg.to_string(),
        file: file.to_string(),
        line,
    }))))
}

pub fn tfm_preload_model<E>(
    ctx: &mut Ctx,
    model: WasmPtr<u8, Array>,
    model_len: u32,
    _inputs: u32,
    _outputs: u32,
) -> Result<u32, RuntimeError>
where
    E: Environment,
{
    let (mem, context) = unsafe { ctx.memory_and_data_mut::<Context<E>>(0) };

    let model = model
        .deref(mem, 0, model_len)
        .ok_or_else(|| runtime_err("Unable to retrieve the model buffer"))?;

    let model = model.iter().map(|b| b.get()).collect();

    context
        .register_model(model)
        .map_err(|e| RuntimeError::User(Box::new(e)))
}

pub fn tfm_model_invoke<E>(
    _ctx: &mut Ctx,
    _feature_idx: WasmPtr<u8, Array>,
    _feature_len: u32,
) -> Result<u32, RuntimeError>
where
    E: Environment,
{
    todo!()
}

pub fn request_capability<E>(ctx: &mut Ctx, capability_type: u32) -> u32
where
    E: Environment,
{
    let context = unsafe { &mut *(ctx.data as *mut Context<E>) };
    let cap = runic_types::CAPABILITY::from_u32(capability_type);

    context.request_capability(cap)
}

pub fn request_capability_set_param<E>(
    ctx: &mut Ctx,
    capability_id: u32,
    key: WasmPtr<u8, Array>,
    key_len: u32,
    value: WasmPtr<u8, Array>,
    value_len: u32,
    value_type: u32,
) -> Result<u32, RuntimeError>
where
    E: Environment,
{
    let (mem, context) = unsafe { ctx.memory_and_data_mut::<Context<E>>(0) };

    let key = key
        .get_utf8_string(mem, key_len)
        .context("Unable to load the key")
        .map_err(|e| RuntimeError::User(Box::new(e)))?;

    let value = value
        .deref(mem, 0, value_len)
        .ok_or_else(|| runtime_err("Bad value pointer"))?
        .iter()
        .map(|v| v.get())
        .collect();

    context
        .set_capability_request_parameter(
            capability_id,
            key,
            value,
            PARAM_TYPE::from_u32(value_type),
        )
        .map_err(|e| RuntimeError::User(Box::new(e)))?;

    Ok(0)
}

pub fn request_manifest_output<E>(ctx: &mut Ctx, output: u32) -> u32
where
    E: Environment,
{
    let context = unsafe { &mut *(ctx.data as *mut Context<E>) };
    let output = OUTPUT::from_u32(output);
    context.register_output(output)
}

pub fn request_provider_response<E>(
    ctx: &mut Ctx,
    buffer: WasmPtr<u8, Array>,
    buffer_len: u32,
    capability_id: u32,
) -> Result<u32, RuntimeError>
where
    E: Environment,
{
    unsafe {
        let (mem, context) = ctx.memory_and_data_mut::<Context<E>>(0);

        let buffer = buffer
            .deref_mut(mem, 0, buffer_len)
            .ok_or_else(|| runtime_err("Bad buffer pointer"))?;

        // SAFETY: We are guaranteed to have unique access to this piece of
        // WebAssembly memory because
        // - The runtime's call() method takes &mut self
        // - Context methods never call back into WebAssembly, so it's not
        //   possible to accidentally end up back here while buffer is still
        //   mutably borrowed
        let buffer: &mut [u8] = std::mem::transmute(buffer);

        context
            .invoke_capability(capability_id, buffer)
            .map_err(|e| RuntimeError::User(Box::new(e)))?;

        Ok(0)
    }
}

/// An error indicating there was an abnormal abort.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("Abort on line {} of {}: {}", line, file, message)]
struct Abort {
    message: String,
    file: String,
    line: u32,
}

fn runtime_err(msg: &'static str) -> RuntimeError {
    RuntimeError::User(Box::new(Error::msg(msg)))
}

/// Tries to extract the [`anyhow::Error`] from a runtime error.
fn runtime_error(e: RuntimeError) -> Error {
    match e {
        RuntimeError::User(user_error) => {
            match user_error.downcast::<Error>() {
                // it was an anyhow::Error thrown by one of our host bindings,
                // keep bubbling it up
                Ok(error) => *error,
                // Sometimes wasmer will randomly re-box a RuntimeError
                Err(other) => match other.downcast::<RuntimeError>() {
                    Ok(error) => runtime_error(*error),
                    // Our code only ever returns an anyhow::Error, so wasmer
                    // must have caught a panic.
                    Err(panic_payload) => {
                        std::panic::resume_unwind(panic_payload)
                    },
                },
            }
        },
        // just fall back to the default error message (RuntimeError: !Sync so
        // we can't just wrap it)
        other => Error::msg(other.to_string()),
    }
}

#[derive(Debug, Clone)]
struct LinkErrors(Vec<LinkError>);

impl std::error::Error for LinkErrors {}

impl Display for LinkErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let errs = &self.0;

        if errs.len() == 1 {
            write!(f, "link error: {}", errs[0])
        } else {
            write!(f, "{} link errors:", errs.len())?;
            for (i, err) in errs.iter().enumerate() {
                write!(f, " ({} of {}) {}", i + 1, errs.len(), err)?;
            }

            Ok(())
        }
    }
}
