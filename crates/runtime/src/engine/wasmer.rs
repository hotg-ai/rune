use std::{
    convert::TryInto,
    fmt::{self, Display, Formatter},
    sync::{Arc, Mutex},
};

use anyhow::{Context, Error};
use hotg_rune_core::Shape;
use wasmer::{
    Array, Function, Instance, LazyInit, Memory, Module, NativeFunc,
    RuntimeError, Store, ValueType, WasmPtr, WasmerEnv,
};

use crate::{
    callbacks::Callbacks,
    engine::{host_functions::HostFunctions, WebAssemblyEngine},
};

pub struct WasmerEngine {
    instance: Instance,
    host_functions: Arc<Mutex<HostFunctions>>,
    callbacks: Arc<dyn Callbacks>,
}

impl WebAssemblyEngine for WasmerEngine {
    fn load(wasm: &[u8], callbacks: Arc<dyn Callbacks>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let store = Store::default();
        let module = Module::from_binary(&store, wasm)
            .context("Unable to load the WebAssembly binary")?;

        let host_functions =
            Arc::new(Mutex::new(HostFunctions::new(callbacks.clone())));
        let env = Env {
            memory: LazyInit::new(),
            host_functions: Arc::clone(&host_functions),
        };

        let imports = wasmer::imports! {
            "env" => {
                "_debug" => Function::new_native_with_env(&store, env.clone(), debug),
                "request_capability" => Function::new_native_with_env(&store, env.clone(), request_capability),
                "request_capability_set_param" => Function::new_native_with_env(&store, env.clone(), request_capability_set_param),
                "request_provider_response" => Function::new_native_with_env(&store, env.clone(), request_provider_response),
                "tfm_model_invoke" => Function::new_native_with_env(&store, env.clone(), tfm_model_invoke),
                "tfm_preload_model" => Function::new_native_with_env(&store, env.clone(), tfm_preload_model),
                "rune_model_load" => Function::new_native_with_env(&store, env.clone(), rune_model_load),
                "rune_model_infer" => Function::new_native_with_env(&store, env.clone(), rune_model_infer),
                "request_output" => Function::new_native_with_env(&store, env.clone(), request_output),
                "consume_output" => Function::new_native_with_env(&store, env.clone(), consume_output),
                "rune_resource_open" => Function::new_native_with_env(&store, env.clone(), rune_resource_open),
                "rune_resource_read" => Function::new_native_with_env(&store, env.clone(), rune_resource_read),
                "rune_resource_close" => Function::new_native_with_env(&store, env.clone(), rune_resource_close),
            }
        };

        let instance = Instance::new(&module, &imports)
            .context("Unable to instantiate the WebAssembly module")?;

        Ok(WasmerEngine {
            instance,
            host_functions,
            callbacks,
        })
    }

    fn init(&mut self) -> Result<(), Error> {
        let manifest: NativeFunc<(), i32> = self
            .instance
            .exports
            .get_native_function("_manifest")
            .context("Unable to get the \"_manifest\" function")?;

        manifest
            .call()
            .map_err(unwrap_anyhow_error)
            .context("Call failed")?;

        let host_functions = self.host_functions.lock().unwrap();
        let graph = host_functions.graph();
        self.callbacks.loaded(&graph)
    }

    fn predict(&mut self) -> Result<(), Error> {
        let call: NativeFunc<(i32, i32, i32), i32> = self
            .instance
            .exports
            .get_native_function("_call")
            .context("Unable to get the \"_call\" function")?;

        call.call(0, 0, 0)
            .map_err(unwrap_anyhow_error)
            .context("Call failed")?;

        Ok(())
    }
}

#[derive(Debug)]
struct Shim(Error);

impl std::error::Error for Shim {}

impl Display for Shim {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

fn runtime_error(e: Error) -> RuntimeError {
    RuntimeError::user(Box::new(Shim(e)))
}

fn unwrap_anyhow_error(e: RuntimeError) -> Error {
    match e.downcast::<Shim>() {
        Ok(e) => e.0,
        Err(other) => Error::from(other),
    }
}

#[derive(Clone, WasmerEnv)]
struct Env {
    #[wasmer(export)]
    memory: LazyInit<Memory>,
    host_functions: Arc<Mutex<HostFunctions>>,
}

fn debug(
    env: &Env,
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

        env.host_functions
            .lock()
            .unwrap()
            .debug(message)
            .map_err(runtime_error)?;

        Ok(0)
    }
}

fn rune_resource_open(
    env: &Env,
    name: WasmPtr<u8, Array>,
    len: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: this function isn't reentrant, so we don't need to worry about
    // concurrent mutations.
    let name = unsafe {
        name.get_utf8_str(memory, len)
            .context("Invalid buffer pointer")
            .map_err(runtime_error)?
    };

    env.host_functions
        .lock()
        .unwrap()
        .rune_resource_open(name)
        .map_err(runtime_error)
}

fn rune_resource_read(
    env: &Env,
    id: u32,
    dest: WasmPtr<u8, Array>,
    len: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    let mut buffer = vec![0_u8; len as usize];

    let bytes_written = env
        .host_functions
        .lock()
        .unwrap()
        .rune_resource_read(id, &mut buffer)
        .map_err(runtime_error)?;

    let len = std::cmp::min(len, bytes_written);

    let view = memory.view::<u8>();
    // Safety: Function isn't re-entrant so we don't need to worry about
    // concurrent mutations.
    unsafe {
        view.subarray(dest.offset(), dest.offset() + len)
            .copy_from(&buffer[..len as usize]);
    }

    Ok(bytes_written)
}

fn rune_resource_close(env: &Env, id: u32) -> Result<(), RuntimeError> {
    env.host_functions
        .lock()
        .unwrap()
        .rune_resource_close(id)
        .map_err(runtime_error)
}

fn request_capability(
    env: &Env,
    capability_type: u32,
) -> Result<u32, RuntimeError> {
    env.host_functions
        .lock()
        .unwrap()
        .request_capability(capability_type)
        .map_err(runtime_error)
}

fn request_capability_set_param(
    env: &Env,
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

        let value: Vec<u8> = value.into_iter().map(|c| c.get()).collect();
        let value = hotg_rune_core::Value::from_le_bytes(ty, &value)
            .context("Unable to deserialize the value")
            .map_err(runtime_error)?;

        env.host_functions
            .lock()
            .unwrap()
            .request_capability_set_param(
                capability_id,
                key,
                stringified(value),
            )
            .map_err(runtime_error)?;

        Ok(0)
    }
}

fn stringified(value: hotg_rune_core::Value) -> String {
    match value {
        hotg_rune_core::Value::Byte(b) => b.to_string(),
        hotg_rune_core::Value::Short(s) => s.to_string(),
        hotg_rune_core::Value::Integer(i) => i.to_string(),
        hotg_rune_core::Value::Float(f) => f.to_string(),
        hotg_rune_core::Value::SignedByte(s) => s.to_string(),
        _ => unreachable!(),
    }
}

fn request_provider_response(
    env: &Env,
    dest: WasmPtr<u8, Array>,
    len: u32,
    capability_id: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    let mut buffer = vec![0_u8; len as usize];

    let bytes_written = env
        .host_functions
        .lock()
        .unwrap()
        .request_provider_response(capability_id, &mut buffer)
        .map_err(runtime_error)?;

    let dest = dest
        .deref(memory, 0, len)
        .context("Invalid buffer pointer")
        .map_err(runtime_error)?;

    for (i, cell) in dest.iter().enumerate() {
        cell.set(buffer[i]);
    }

    Ok(bytes_written)
}

fn tfm_model_invoke(
    env: &Env,
    _model_id: u32,
    _input: WasmPtr<u8, Array>,
    _input_len: u32,
    _output: WasmPtr<u8, Array>,
    _output_len: u32,
) -> Result<u32, RuntimeError> {
    env.host_functions
        .lock()
        .unwrap()
        .tfm_model_invoke()
        .map_err(runtime_error)?;
    Ok(0)
}

fn rune_model_infer(
    env: &Env,
    model_id: u32,
    inputs: WasmPtr<WasmPtr<u8, Array>, Array>,
    outputs: WasmPtr<WasmPtr<u8, Array>, Array>,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    let mut host = env.host_functions.lock().unwrap();

    let model = host
        .model_by_id(model_id)
        .with_context(|| format!("No model with ID {}", model_id))
        .map_err(runtime_error)?;

    unsafe {
        let (inputs, mut outputs) = inputs_and_outputs(
            memory,
            model.input_shapes(),
            inputs,
            model.output_shapes(),
            outputs,
        )
        .map_err(runtime_error)?;

        host.rune_model_infer(model_id, &inputs, &mut outputs)
            .map_err(runtime_error)?;
    }

    Ok(0)
}

/// Given WebAssembly pointers to tensors in linear memory, get access to their
/// backing arrays.
///
/// # Safety
///
/// This assumes linear memory won't be touched again (e.g. by executing a
/// WebAssembly function) until the returned references are dropped.
///
/// We also assume that none of the output tensor buffers are aliased. This
/// should be fine because the Rune is written in Rust and the borrow checker
/// will enforce mutable XOR shared.
unsafe fn inputs_and_outputs<'vm>(
    memory: &'vm Memory,
    input_shapes: &[Shape<'_>],
    input_ptr: WasmPtr<WasmPtr<u8, Array>, Array>,
    output_shapes: &[Shape<'_>],
    output_ptr: WasmPtr<WasmPtr<u8, Array>, Array>,
) -> Result<(Vec<&'vm [u8]>, Vec<&'vm mut [u8]>), Error> {
    let linear_memory = memory.data_unchecked_mut();

    let inputs = offsets(memory, input_ptr, input_shapes)?;
    let inputs: Vec<_> = inputs
        .into_iter()
        .map(|(start, end)| {
            std::slice::from_raw_parts(
                linear_memory.as_ptr().add(start),
                end - start,
            )
        })
        .collect();

    let outputs = offsets(memory, output_ptr, output_shapes)?;
    let outputs: Vec<_> = outputs
        .into_iter()
        .map(|(start, end)| {
            std::slice::from_raw_parts_mut(
                linear_memory.as_mut_ptr().add(start),
                end - start,
            )
        })
        .collect();

    Ok((inputs, outputs))
}

fn offsets(
    mem: &Memory,
    pointers: WasmPtr<WasmPtr<u8, Array>, Array>,
    shapes: &[Shape<'_>],
) -> Result<Vec<(usize, usize)>, Error> {
    let pointers = pointers
        .deref(mem, 0, shapes.len() as u32)
        .context("Pointer out of bounds")?;

    let mut offsets = Vec::new();

    for (i, shape) in shapes.iter().enumerate() {
        let ptr = pointers[i].get();
        let start = ptr.offset() as usize;
        let size = shape
            .size()
            .context("The element type is dynamically sized")?;
        let end = start + size;

        offsets.push((start, end));
    }

    Ok(offsets)
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
    env: &Env,
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
    // modifications.
    let (mimetype, model) = unsafe {
        let mimetype = mimetype
            .get_utf8_str(memory, mimetype_len)
            .context("Invalid mimtype string")
            .map_err(runtime_error)?;

        let model = model
            .deref(memory, 0, model_len)
            .context("Invalid model")
            .map_err(runtime_error)?;

        let model: Vec<u8> = model.into_iter().map(|v| v.get()).collect();

        (mimetype, model)
    };

    let (inputs, outputs) = unsafe {
        let inputs =
            shape_from_descriptors(memory, input_descriptors, input_len)
                .map_err(runtime_error)?;
        let outputs =
            shape_from_descriptors(memory, output_descriptors, output_len)
                .map_err(runtime_error)?;

        (inputs, outputs)
    };

    env.host_functions
        .lock()
        .unwrap()
        .rune_model_load(mimetype, &model, &inputs, &outputs)
        .map_err(runtime_error)
}

fn tfm_preload_model(
    env: &Env,
    _model: WasmPtr<u8, Array>,
    _model_len: u32,
    _: u32,
    _: u32,
) -> Result<u32, RuntimeError> {
    env.host_functions
        .lock()
        .unwrap()
        .tfm_preload_model()
        .map_err(runtime_error)?;

    Ok(0)
}

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

fn request_output(env: &Env, output_type: u32) -> Result<u32, RuntimeError> {
    env.host_functions
        .lock()
        .unwrap()
        .request_output(output_type)
        .map_err(runtime_error)
}

fn consume_output(
    env: &Env,
    output_id: u32,
    buffer: WasmPtr<u8, Array>,
    len: u32,
) -> Result<u32, RuntimeError> {
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
    let buffer: Vec<u8> = buffer.into_iter().map(|c| c.get()).collect();

    env.host_functions
        .lock()
        .unwrap()
        .consume_output(output_id, &buffer)
        .map_err(runtime_error)?;

    Ok(len)
}
