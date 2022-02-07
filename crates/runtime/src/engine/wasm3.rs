use std::{
    sync::{Arc, Mutex},
    convert::TryInto,
    alloc::Layout,
};

use anyhow::{Error, Context};
use hotg_rune_core::{Value, Shape};
use wasm3::{Environment, Module, WasmArgs, WasmType, CallContext, error::Trap};

use crate::{engine::WebAssemblyEngine, HostFunctions};

const STACK_SIZE: u32 = 1024 * 16;

pub struct Wasm3Engine(wasm3::Runtime);

impl WebAssemblyEngine for Wasm3Engine {
    fn load(
        wasm: &[u8],
        host_functions: Arc<Mutex<HostFunctions>>,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let env = Environment::new().to_anyhow()?;

        let runtime = env
            .create_runtime(STACK_SIZE)
            .to_anyhow()
            .context("Unable to create the runtime")?;

        log::debug!("Instantiating the WebAssembly module");
        let instance = runtime.parse_and_load_module(wasm).to_anyhow()?;

        Linker::new(instance, &host_functions)
            .link("_debug", debug)?
            .link("request_capability", request_capability)?
            .link("request_capability_set_param", request_capability_set_param)?
            .link("request_provider_response", request_provider_response)?
            .link("tfm_model_invoke", tfm_model_invoke)?
            .link("tfm_preload_model", tfm_preload_model)?
            .link("rune_model_load", rune_model_load)?
            .link("rune_model_infer", rune_model_infer)?
            .link("request_output", request_output)?
            .link("consume_output", consume_output)?
            .link("rune_resource_open", rune_resource_open)?
            .link("rune_resource_read", rune_resource_read)?
            .link("rune_resource_close", rune_resource_close)?;

        Ok(Wasm3Engine(runtime))
    }

    fn init(&mut self) -> Result<(), Error> { todo!() }

    fn call(&mut self) -> Result<(), Error> { todo!() }
}

struct Linker<'rt> {
    instance: Module<'rt>,
    host_functions: Arc<Mutex<HostFunctions>>,
}

impl<'rt> Linker<'rt> {
    fn new(
        instance: Module<'rt>,
        host_functions: &Arc<Mutex<HostFunctions>>,
    ) -> Self {
        Self {
            instance,
            host_functions: Arc::clone(host_functions),
        }
    }

    fn link<F, Ret, Args>(
        &mut self,
        name: &str,
        mut func: F,
    ) -> Result<&mut Self, Error>
    where
        Args: WasmArgs,
        Ret: WasmType,
        F: for<'cc> FnMut(
                CallContext<'cc>,
                &mut HostFunctions,
                Args,
            ) -> Result<Ret, Error>
            + 'static,
    {
        let host_functions = Arc::clone(&self.host_functions);
        let ret = self.instance.link_closure(
            "env",
            name,
            move |cc: CallContext<'_>, args: Args| {
                let mut host_functions = host_functions
                    .lock()
                    .map_err(|_| wasm3::error::Trap::Abort)?;

                match func(cc, &mut *host_functions, args) {
                    Ok(ret) => Ok(ret),
                    Err(e) => {
                        log::error!("{:?}", e);
                        Err(Trap::Abort)
                    },
                }
            },
        );

        match ret {
            Ok(_) | Err(wasm3::error::Error::FunctionNotFound) => Ok(self),
            Err(e) => Err(Error::msg(e.to_string())),
        }
    }
}

fn debug(
    cc: CallContext<'_>,
    host: &mut HostFunctions,
    (msg, len): (u32, u32),
) -> Result<u32, Error> {
    let message = cc
        .read_string(msg, len)
        .context("Unable to read the log message")?;
    host.debug(message)?;

    Ok(0)
}

fn request_capability(
    _cc: CallContext<'_>,
    host: &mut HostFunctions,
    capability_type: u32,
) -> Result<u32, Error> {
    host.request_capability(capability_type)
}

fn request_capability_set_param(
    cc: CallContext<'_>,
    host: &mut HostFunctions,
    (capability_id, key_ptr, key_len, value_ptr, value_len, value_type): (
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    ),
) -> Result<u32, Error> {
    let key = cc
        .read_string(key_ptr, key_len)
        .context("Unable to read the key")?;
    let value = unsafe {
        cc.array(value_ptr, value_len)
            .context("Unable to read the value")?
    };
    let value_type = value_type
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid type for \"{}\"", key))?;
    let value = Value::from_le_bytes(value_type, value).with_context(|| {
        format!("Invalid {:?} value for \"{}\"", value_type, key)
    })?;

    let value = match value {
        Value::Byte(b) => b.to_string(),
        Value::Short(s) => s.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::SignedByte(s) => s.to_string(),
        _ => anyhow::bail!("Unknown value type: {}", value),
    };

    host.request_capability_set_param(capability_id, key, value)?;

    Ok(0)
}

fn request_provider_response(
    cc: CallContext<'_>,
    host: &mut HostFunctions,
    (buffer, len, capability_id): (u32, u32, u32),
) -> Result<u32, Error> {
    let buffer = unsafe { cc.array_mut(buffer, len)? };
    host.request_provider_response(capability_id, buffer)?;

    Ok(buffer.len() as u32)
}

fn tfm_model_invoke(
    _cc: CallContext<'_>,
    host: &mut HostFunctions,
    (_model_id, _inputs, _input_len, _outputs, _output_len): (
        u32,
        u32,
        u32,
        u32,
        u32,
    ),
) -> Result<u32, Error> {
    host.tfm_model_invoke()?;
    Ok(0)
}

fn rune_model_infer(
    cc: CallContext<'_>,
    host: &mut HostFunctions,
    (model_id, inputs, outputs): (u32, u32, u32),
) -> Result<u32, Error> {
    let model = host
        .model_by_id(model_id)
        .with_context(|| format!("No model with ID {}", model_id))?;

    let input_shapes = model.input_shapes();
    let inputs: Vec<&[u8]> = unsafe {
        cc.array(inputs, input_shapes.len() as u32)?
            .iter()
            .copied()
            .enumerate()
            .map(|(i, ptr): (usize, u32)| {
                cc.array(ptr, input_shapes[i].size().unwrap() as u32)
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    let output_shapes = model.output_shapes();
    let mut outputs: Vec<&mut [u8]> = unsafe {
        cc.array_mut(outputs, output_shapes.len() as u32)?
            .iter()
            .copied()
            .enumerate()
            .map(|(i, ptr): (usize, u32)| {
                cc.array_mut(ptr, input_shapes[i].size().unwrap() as u32)
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    host.rune_model_infer(model_id, &inputs, &mut outputs)?;

    todo!()
}

fn rune_model_load(
    cc: CallContext<'_>,
    host: &mut HostFunctions,
    (
        mimetype,
        mimetype_len,
        model,
        model_len,
        input_descriptors,
        input_len,
        output_descriptors,
        output_len,
    ): (u32, u32, u32, u32, u32, u32, u32, u32),
) -> Result<u32, Error> {
    let mimetype = cc.read_string(mimetype, mimetype_len)?;
    let model = unsafe { cc.array(model, model_len)? };
    let inputs = read_shapes(&cc, input_descriptors, input_len)?;
    let outputs = read_shapes(&cc, output_descriptors, output_len)?;

    host.rune_model_load(mimetype, model, &inputs, &outputs)
}

fn read_shapes(
    cc: &CallContext<'_>,
    input_descriptors: u32,
    input_len: u32,
) -> Result<Vec<Shape<'static>>, Error> {
    let mut shapes = Vec::new();

    let strings: &[StringRef] =
        unsafe { cc.array(input_descriptors, input_len)? };

    for s in strings {
        let StringRef { data, len } = *s;
        let s = cc.read_string(data, len)?;
        let parsed: Shape<'static> = s.parse()?;
        shapes.push(parsed)
    }

    Ok(shapes)
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct StringRef {
    data: u32,
    len: u32,
}

fn tfm_preload_model(
    _cc: CallContext<'_>,
    host: &mut HostFunctions,
    (_model, _model_len, _, _): (u32, u32, u32, u32),
) -> Result<u32, Error> {
    host.tfm_preload_model()?;
    Ok(0)
}

fn request_output(
    _cc: CallContext<'_>,
    host: &mut HostFunctions,
    output_type: u32,
) -> Result<u32, Error> {
    host.request_output(output_type)
}

fn consume_output(
    cc: CallContext<'_>,
    host: &mut HostFunctions,
    (output_id, buffer, len): (u32, u32, u32),
) -> Result<u32, Error> {
    let data = unsafe { cc.array(buffer, len)? };
    host.consume_output(output_id, data)?;

    Ok(len)
}

fn rune_resource_open(
    cc: CallContext<'_>,
    host: &mut HostFunctions,
    (name, len): (u32, u32),
) -> Result<u32, Error> {
    let name = cc.read_string(name, len)?;
    host.rune_resource_open(name)
}

fn rune_resource_read(
    cc: CallContext<'_>,
    host: &mut HostFunctions,
    (id, buffer, len): (u32, u32, u32),
) -> Result<u32, Error> {
    let buffer = unsafe { cc.array_mut(buffer, len)? };
    host.rune_resource_read(id, buffer)
}

fn rune_resource_close(
    _cc: CallContext<'_>,
    host: &mut HostFunctions,
    id: u32,
) -> Result<u32, Error> {
    host.rune_resource_close(id)?;
    Ok(0)
}

trait Wasm3ResultExt<T> {
    fn to_anyhow(self) -> Result<T, Error>;
}

impl<T> Wasm3ResultExt<T> for Result<T, wasm3::error::Error> {
    fn to_anyhow(self) -> Result<T, Error> {
        self.map_err(|e| Error::msg(e.to_string()))
    }
}

trait LinkResultExt {
    fn ignore_missing_functions(self) -> Result<(), Error>;
}

impl LinkResultExt for Result<(), wasm3::error::Error> {
    fn ignore_missing_functions(self) -> Result<(), Error> {
        match self {
            Ok(_) => Ok(()),
            Err(_) => todo!(),
        }
    }
}

trait CallContextExt<'a> {
    unsafe fn array<T: Copy>(&self, ptr: u32, len: u32) -> Result<&[T], Error>;
    unsafe fn array_mut<T: Copy>(
        &self,
        ptr: u32,
        len: u32,
    ) -> Result<&mut [T], Error>;

    fn read_string(&self, ptr: u32, len: u32) -> Result<&str, Error> {
        let bytes = unsafe { self.array(ptr, len)? };
        std::str::from_utf8(bytes).map_err(Error::from)
    }
}

impl<'a> CallContextExt<'a> for CallContext<'a> {
    unsafe fn array<T: Copy>(&self, ptr: u32, len: u32) -> Result<&[T], Error> {
        let memory = &mut *self.memory_mut();

        let ptr = ptr as usize;
        let len = len as usize;
        let layout = Layout::array::<T>(len as usize)?;
        let end = ptr + layout.size();

        let bytes = memory.get(ptr..end).with_context(|| {
            format!(
                "Range {}..{} lies outside of linear memory ({} bytes)",
                ptr,
                end,
                memory.len()
            )
        })?;

        let (before, items, after) = bytes.align_to();
        anyhow::ensure!(
            before.is_empty() && after.is_empty(),
            "Array was unaligned"
        );

        Ok(items)
    }

    unsafe fn array_mut<T: Copy>(
        &self,
        ptr: u32,
        len: u32,
    ) -> Result<&mut [T], Error> {
        let memory = &mut *self.memory_mut();
        let memory_len = memory.len();

        let ptr = ptr as usize;
        let len = len as usize;
        let layout = Layout::array::<T>(len as usize)?;
        let end = ptr + layout.size();

        let bytes = memory.get_mut(ptr..end).with_context(|| {
            format!(
                "Range {}..{} lies outside of linear memory ({} bytes)",
                ptr, end, memory_len
            )
        })?;

        let (before, items, after) = bytes.align_to_mut();
        anyhow::ensure!(
            before.is_empty() && after.is_empty(),
            "Array was unaligned"
        );

        Ok(items)
    }
}
