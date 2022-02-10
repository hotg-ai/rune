use std::{
    sync::{Arc, Mutex},
    convert::TryInto,
    alloc::Layout,
};

use anyhow::{Error, Context};
use hotg_rune_core::{Value, Shape};
use wasm3::{
    Environment, Module, WasmArgs, WasmType, CallContext, error::Trap,
    Function, error::Error as Wasm3Error,
};

use crate::{
    engine::{WebAssemblyEngine, host_functions::HostFunctions},
    callbacks::Callbacks,
};

const STACK_SIZE: u32 = 1024 * 16;

pub struct Wasm3Engine {
    runtime: wasm3::Runtime,
    host_functions: Arc<Mutex<HostFunctions>>,
    last_error: Arc<Mutex<Option<Error>>>,
    callbacks: Arc<dyn Callbacks>,
}

impl Wasm3Engine {
    /// Find a function in the wasm3 module and try to call it.
    ///
    /// Sorry for the generics soup and the whole `apply` thing. The
    /// `wasm3::Function` type doesn't actually have a single generic `call()`
    /// method which accepts some arguments, instead they "overload"
    /// `Function::call()` based on the argument type so you don't need to pass
    /// in tuples.
    fn call<Args, Ret>(
        &mut self,
        name: &str,
        args: Args,
        apply: impl FnOnce(Function<Args, Ret>, Args) -> Result<Ret, Wasm3Error>,
    ) -> Result<Ret, Error>
    where
        Ret: WasmType,
        Args: WasmArgs,
    {
        let function: Function<Args, Ret> =
            self.runtime.find_function(name).to_anyhow().with_context(
                || format!("Unable to find the \"{}()\" function", name),
            )?;

        match apply(function, args) {
            Ok(ret) => Ok(ret),
            // We know that host function errors will emit a trap and set
            // last_error, so we can use that to try and give the user a more
            // useful error message.
            Err(Wasm3Error::Wasm3(e)) if e.is_trap(Trap::Abort) => {
                match self.last_error.lock().unwrap().take() {
                    Some(e) => Err(e),
                    None => Err(Wasm3Error::Wasm3(e)).to_anyhow(),
                }
            },
            Err(e) => Err(e).to_anyhow(),
        }
    }
}

impl WebAssemblyEngine for Wasm3Engine {
    fn load(wasm: &[u8], callbacks: Arc<dyn Callbacks>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let env = Environment::new().to_anyhow()?;
        let host_functions =
            Arc::new(Mutex::new(HostFunctions::new(Arc::clone(&callbacks))));

        let runtime = env
            .create_runtime(STACK_SIZE)
            .to_anyhow()
            .context("Unable to create the runtime")?;

        log::debug!("Instantiating the WebAssembly module");
        let instance = runtime.parse_and_load_module(wasm).to_anyhow()?;

        let last_error = Arc::new(Mutex::new(None));

        Linker::new(instance, &last_error, &host_functions)
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

        Ok(Wasm3Engine {
            runtime,
            last_error,
            host_functions,
            callbacks,
        })
    }

    fn init(&mut self) -> Result<(), Error> {
        let _: i32 = self.call("_manifest", (), |f, _| f.call())?;
        let host_functions = self.host_functions.lock().unwrap();
        let graph = host_functions.graph();

        self.callbacks.loaded(&graph)
    }

    fn predict(&mut self) -> Result<(), Error> {
        // Note: these three parameters used to contain the ID for the RAND
        // capability plus the tensor type sent to the SERIAL output. They are
        // now redundant because the pipeline nodes are compiled directly into
        // the Rune.
        //
        // We should be able to change the _call function's signature once
        // hotg-ai/rune#28 lands.
        let _: i32 =
            self.call("_call", (0_i32, 0_i32, 0_i32), |f, (a, b, c)| {
                f.call(a, b, c)
            })?;

        Ok(())
    }
}

struct Linker<'rt> {
    instance: Module<'rt>,
    last_error: Arc<Mutex<Option<Error>>>,
    host_functions: Arc<Mutex<HostFunctions>>,
}

impl<'rt> Linker<'rt> {
    fn new(
        instance: Module<'rt>,
        last_error: &Arc<Mutex<Option<Error>>>,
        host_functions: &Arc<Mutex<HostFunctions>>,
    ) -> Self {
        Self {
            instance,
            last_error: Arc::clone(last_error),
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
        let error_location = Arc::clone(&self.last_error);

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
                        *error_location.lock().expect("Lock was poisoned") =
                            Some(e);

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

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        ffi::OsStr,
        sync::atomic::{AtomicBool, Ordering},
    };
    use log::Record;
    use crate::{
        callbacks::{ModelMetadata, Model, RuneGraph},
    };

    use super::*;

    #[derive(Debug, Default)]
    struct Spy {
        loaded: AtomicBool,
    }

    impl Callbacks for Spy {
        fn read_capability(
            &self,
            _id: u32,
            _meta: &crate::NodeMetadata,
            _buffer: &mut [u8],
        ) -> Result<(), Error> {
            todo!()
        }

        fn write_output(
            &self,
            _id: u32,
            _meta: &crate::NodeMetadata,
            _data: &[u8],
        ) -> Result<(), Error> {
            todo!()
        }

        fn load_model(
            &self,
            _id: u32,
            _meta: &ModelMetadata<'_>,
            _model: &[u8],
        ) -> Result<Box<dyn Model>, Error> {
            struct Dummy;
            impl Model for Dummy {
                fn infer(
                    &mut self,
                    _inputs: &[&[u8]],
                    _outputs: &mut [&mut [u8]],
                ) -> Result<(), Error> {
                    todo!()
                }

                fn input_shapes(&self) -> &[Shape<'_>] { todo!() }

                fn output_shapes(&self) -> &[Shape<'_>] { todo!() }
            }
            Ok(Box::new(Dummy))
        }

        fn model_infer(
            &self,
            _id: u32,
            _inputs: &[&[u8]],
            _outputs: &mut [&mut [u8]],
        ) -> Result<(), Error> {
            todo!()
        }

        fn get_resource(&self, _name: &str) -> Option<&[u8]> { Some(&[]) }

        fn log(&self, _record: &Record<'_>) {}

        fn loaded(&self, _rune: &RuneGraph<'_>) -> Result<(), Error> {
            self.loaded.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    #[test]
    fn load_all_existing_runes() {
        for rune in all_runes() {
            let wasm = std::fs::read(&rune).unwrap();
            let state = Arc::new(Spy::default());

            let callbacks = Arc::clone(&state) as Arc<dyn Callbacks>;
            let mut engine = Wasm3Engine::load(&wasm, callbacks).unwrap();

            engine.init().unwrap();

            assert!(state.loaded.load(Ordering::SeqCst));
        }
    }

    fn all_runes() -> Vec<PathBuf> {
        let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let project_root = crate_dir.parent().unwrap().parent().unwrap();

        let mut runes = Vec::new();
        runes.extend(find_runes(project_root.join("examples")));
        runes.extend(find_runes(project_root.join("integration-tests")));

        runes
    }

    fn find_runes(root: impl AsRef<Path>) -> Vec<PathBuf> {
        let root = root.as_ref();
        let mut runes = Vec::new();

        for entry in root.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() && path.extension() == Some(OsStr::new("rune")) {
                runes.push(path);
            } else if path.is_dir() {
                runes.extend(find_runes(&path));
            }
        }

        runes
    }
}
