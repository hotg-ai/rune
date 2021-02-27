use crate::provider::Provider;
use anyhow::{Context, Error};
use log;
use std::{ffi::c_void, string::String};
use wasmer_runtime::{
    error::RuntimeError, func, imports, instantiate, Array, Ctx, Func,
    Instance, WasmPtr,
};

/// Rune Executor
///  Executes the Rune and provides the appropriate interfaces
pub struct VM {
    instance: Instance,
}

///
impl VM {
    pub fn init(filename: &str) -> Result<Self, Error> {
        log::info!("Initializing");

        let rune_bytes = std::fs::read(filename)
            .with_context(|| format!("Unable to read \"{}\"", filename))?;

        log::debug!(
            "Loaded {} bytes from {} container",
            rune_bytes.len(),
            filename
        );

        let imports = VM::get_imports();
        let mut instance = instantiate(&rune_bytes[..], &imports)
            .map_err(|e| Error::msg(e.to_string()))
            .context("failed to instantiate Rune")?;

        // Pass ownership of our Provider to the instance and tell it how to
        // destroy the Provider afterwards.
        let data = Box::new(Provider::init());
        let ctx = instance.context_mut();
        ctx.data = Box::into_raw(data) as *mut c_void;
        ctx.data_finalizer = Some(|data| {
            // SAFETY: We are the only ones to set instance data so we know
            // what type it is.
            unsafe {
                let _ = Box::from_raw(data as *mut Provider);
            }
        });

        let manifest: Func<(), u32> = instance
            .exports
            .get("_manifest")
            .context("Unable to get the _manifest() function")?;
        let _manifest_size: u32 = manifest
            .call()
            .map_err(|e| Error::msg(e.to_string()))
            .context("failed to call manifest")?;

        Ok(VM { instance })
    }

    fn get_imports() -> wasmer_runtime::ImportObject {
        imports! {
            "env" => {
                "tfm_model_invoke" => func!(tfm_model_invoke),
                "tfm_preload_model" => func!(tfm_preload_model),
                "_debug" => func!(_debug),
                "request_capability" => func!(request_capability),
                "request_capability_set_param" => func!(request_capability_set_param),
                "request_manifest_output" => func!(request_manifest_output),
                "request_provider_response" => func!(request_provider_response)
            },
        }
    }

    pub fn call(&self, _input: Vec<u8>) -> Result<Vec<u8>, Error> {
        let instance = &self.instance;

        log::info!("CALLING ");
        let call_fn: Func<(i32, i32, i32), i32> = instance
            .exports
            .get("_call")
            .context("Unable to load the _call() function")?;

        let feature_buff_size = call_fn
            .call(
                runic_types::CAPABILITY::RAND as i32,
                runic_types::PARAM_TYPE::FLOAT as i32,
                0,
            )
            .map_err(inner_error)
            .context("failed to _call")?;
        log::debug!("Guest::_call() returned {}", feature_buff_size);

        Ok(vec![0, 2, 1, 2])
    }
}

/// Tries to extract the [`anyhow::Error`] from a runtime error.
fn inner_error(e: RuntimeError) -> Error {
    match e {
        RuntimeError::User(user_error) => {
            match user_error.downcast::<Error>() {
                // it was an anyhow::Error thrown by one of our host bindings,
                // keep bubbling it up
                Ok(error) => *error,
                // some other sort of error
                Err(other) => Error::msg(RuntimeError::User(other).to_string()),
            }
        },
        // just fall back to the default error message (RuntimeError: !Sync so
        // we can't just wrap it)
        other => Error::msg(other.to_string()),
    }
}

fn get_mem_str(
    ctx: &Ctx,
    ptr: WasmPtr<u8, Array>,
    data_len: u32,
) -> Result<String, Error> {
    let bytes = get_mem_array(ctx, ptr, data_len)?;
    String::from_utf8(bytes).map_err(Error::from)
}

fn get_mem_array(
    ctx: &Ctx,
    ptr: WasmPtr<u8, Array>,
    data_len: u32,
) -> Result<Vec<u8>, Error> {
    let memory = ctx.memory(0);

    let bytes = ptr.deref(memory, 0, data_len).context("Invalid pointer")?;
    let mut buffer = vec![0; bytes.len()];

    for (src, dest) in bytes.iter().zip(&mut buffer) {
        *dest = src.get();
    }

    Ok(buffer)
}

pub fn tfm_preload_model(
    ctx: &mut Ctx,
    model_idx: WasmPtr<u8, Array>,
    model_len: u32,
    inputs: u32,
    outputs: u32,
) -> Result<u32, RuntimeError> {
    let provider: &mut Provider = unsafe { &mut *(ctx.data as *mut Provider) };

    let model = get_mem_array(ctx, model_idx, model_len)
        .map_err(|e| RuntimeError::User(Box::new(e)))?;

    Ok(provider.add_model(model, inputs, outputs))
}

pub fn tfm_model_invoke(
    ctx: &mut Ctx,
    feature_idx: WasmPtr<u8, Array>,
    feature_len: u32,
) -> Result<u32, RuntimeError> {
    log::info!("Calling tfm_model_invoke");

    let feature_bytes = get_mem_array(ctx, feature_idx, feature_len)
        .map_err(|e| RuntimeError::User(Box::new(e)))?;
    log::info!("{:?}", feature_bytes);

    let provider: &mut Provider = unsafe { &mut *(ctx.data as *mut Provider) };

    provider
        .predict_model::<f32>(0, feature_bytes, runic_types::PARAM_TYPE::FLOAT)
        .map_err(|e| RuntimeError::User(Box::new(e)))?;

    Ok(0)
}

pub fn _debug(ctx: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32) -> u32 {
    if let Ok(msg) = get_mem_str(ctx, ptr, len) {
        log::info!("[Rune::Debug] {}", msg);
    }

    0
}

pub fn request_capability(ctx: &mut Ctx, ct: u32) -> u32 {
    let provider: &mut Provider = unsafe { &mut *(ctx.data as *mut Provider) };

    log::info!("Requesting Capability");
    provider.request_capability(ct)
}

pub fn request_capability_set_param(
    ctx: &mut Ctx,
    idx: u32,
    key_str_ptr: WasmPtr<u8, Array>,
    key_str_len: u32,
    value_ptr: WasmPtr<u8, Array>,
    value_len: u32,
    value_type: u32,
) -> Result<u32, RuntimeError> {
    let provider: &mut Provider = unsafe { &mut *(ctx.data as *mut Provider) };

    log::info!("Setting param");
    let key_str = get_mem_str(ctx, key_str_ptr, key_str_len)
        .map_err(|e| RuntimeError::User(Box::new(e)))?;

    let value = get_mem_array(ctx, value_ptr, value_len)
        .map_err(|e| RuntimeError::User(Box::new(e)))?;

    provider.set_capability_request_param(
        idx,
        key_str.clone(),
        value.clone(),
        runic_types::PARAM_TYPE::from_u32(value_type),
    );

    Ok(0)
}

pub fn request_manifest_output(_ctx: &mut Ctx, _t: u32) -> u32 {
    log::info!("Setting output");
    return 0;
}

pub fn request_provider_response(
    ctx: &mut Ctx,
    provider_response_idx: WasmPtr<u8, Array>,
    max_allowed_provider_response: u32,
    _capability_idx: u32,
) -> Result<u32, RuntimeError> {
    log::info!("Requesting provider response");

    // Get Capaability and get input
    let input = f32::to_be_bytes(0.2);

    let wasm_instance_memory = ctx.memory(0);
    log::debug!("Trying to write provider response");

    let memory_writer = provider_response_idx
        .deref(wasm_instance_memory, 0, max_allowed_provider_response)
        .context("Unable to get a reference to the input memory")
        .map_err(|e| RuntimeError::User(Box::new(e)))?;

    if memory_writer.len() < input.len() {
        // the caller hasn't given us enough space
        return Err(RuntimeError::User(Box::new(Error::msg(
            "Insufficient space",
        ))));
    }

    for (src, dest) in input.iter().copied().zip(memory_writer) {
        dest.set(src);
    }

    Ok(input.len() as u32)
}
