use crate::{context::Context, Environment};
use anyhow::Error;
use std::{ffi::c_void, sync::Mutex};
use wasmer_runtime::{
    error::{CallError, Error as WasmerError, RuntimeError, RuntimeResult},
    func, Array, Ctx, ImportObject, Instance, WasmPtr,
};

pub struct Runtime {
    instance: Instance,
}

impl Runtime {
    pub fn load<E>(rune: &[u8], env: E) -> Result<Self, WasmerError>
    where
        E: Environment + Send + Sync + 'static,
    {
        let module = wasmer_runtime::compile(rune)?;
        let imports = onetime_import_object(env);
        let instance = module.instantiate(&imports)?;

        // TODO: Rename the _manifest() method to _start() so it gets
        // automatically invoked while instantiating.
        instance.call("_manifest", &[])?;

        Ok(Runtime { instance })
    }

    pub fn call(&mut self) -> Result<(), CallError> {
        self.instance.call("_call", &[])?;

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

pub fn request_capability<E>(_ctx: &mut Ctx, _ct: u32) -> u32
where
    E: Environment,
{
    todo!()
}

pub fn request_capability_set_param<E>(
    _ctx: &mut Ctx,
    _idx: u32,
    _key_str_ptr: WasmPtr<u8, Array>,
    _key_str_len: u32,
    _value_ptr: WasmPtr<u8, Array>,
    _value_len: u32,
    _value_type: u32,
) -> Result<u32, RuntimeError>
where
    E: Environment,
{
    todo!()
}

pub fn request_manifest_output<E>(_ctx: &mut Ctx, _t: u32) -> u32
where
    E: Environment,
{
    todo!()
}

pub fn request_provider_response<E>(
    _ctx: &mut Ctx,
    _provider_response_idx: WasmPtr<u8, Array>,
    _max_allowed_provider_response: u32,
    _capability_idx: u32,
) -> Result<u32, RuntimeError>
where
    E: Environment,
{
    todo!()
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
