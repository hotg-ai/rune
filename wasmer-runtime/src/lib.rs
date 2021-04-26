use std::collections::HashMap;
use anyhow::{Context, Error};
use rune_runtime::{Image, Signature, WasmType, WasmValue};
use wasmer::{
    Array, Exports, Function, Instance, LazyInit, Memory, Module, NativeFunc,
    RuntimeError, Store, Val, WasmPtr,
};
use wasmer::ImportObject;
use wasmer_vm::Trap;

#[derive(Clone, Debug)]
pub struct Runtime {
    instance: Instance,
}

impl Runtime {
    pub fn load(wasm: &[u8], image: impl Image) -> Result<Self, Error> {
        let store = Store::default();
        let module = Module::new(&store, wasm)?;
        Runtime::load_from_module(&module, &store, image)
    }

    pub fn load_from_module(
        module: &Module,
        store: &Store,
        image: impl Image,
    ) -> Result<Self, Error> {
        log::debug!("Loading image");
        let mut registrar = Registrar::new(store);
        image.initialize_imports(&mut registrar);
        let imports = registrar.into_import_object();

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
            .call(0, 0, 0)
            .context("Unable to call the _call function")?;

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct Registrar<'s> {
    namespaces: HashMap<String, Exports>,
    store: &'s Store,
}

impl<'s> Registrar<'s> {
    fn new(store: &'s Store) -> Self {
        Registrar {
            namespaces: HashMap::default(),
            store,
        }
    }

    fn into_import_object(self) -> ImportObject {
        let mut obj = ImportObject::default();

        for (name, namespace) in self.namespaces {
            obj.register(name, namespace);
        }

        obj
    }
}

impl<'s> rune_runtime::Registrar for Registrar<'s> {
    fn register_function(
        &mut self,
        namespace: &str,
        name: &str,
        f: rune_runtime::Function,
    ) {
        let ns = self.namespaces.entry(namespace.to_string()).or_default();

        let wrapped_func = Function::new_with_env(
            self.store,
            signature_to_wasmer(f.signature()),
            CallContext::default(),
            move |ctx, args| {
                let converted = args
                    .iter()
                    .map(wasmer_to_value)
                    .collect::<Result<Vec<WasmValue>, RuntimeError>>()?;

                match f.call(ctx, &converted) {
                    Ok(ret) => {
                        Ok(ret.into_iter().map(value_to_wasmer).collect())
                    },
                    Err(e) => {
                        let trap = Trap::new_from_user(e.into());
                        Err(RuntimeError::from_trap(trap))
                    },
                }
            },
        );
        ns.insert(name, wrapped_func);
    }
}

#[derive(Default, Clone, wasmer::WasmerEnv)]
struct CallContext {
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

impl rune_runtime::CallContext for CallContext {
    fn memory(&self, address: u32, len: u32) -> Result<&[u8], Error> {
        let memory = self
            .memory
            .get_ref()
            .context("Call context not initialized")?;
        let data = WasmPtr::<u8, Array>::new(address)
            .deref(&memory, 0, len)
            .context("Bad pointer")?;

        // SAFETY: All runtime methods take &mut and host functions don't
        // recursively call back into WebAssembly, so there are no concurrency
        // bugs here. When ownership rules are obeyed it's also valid to
        // transmute from a &[Cell<T>] to &[T].
        unsafe {
            Ok(std::slice::from_raw_parts(data.as_ptr().cast(), data.len()))
        }
    }

    unsafe fn memory_mut(
        &self,
        address: u32,
        len: u32,
    ) -> Result<&mut [u8], Error> {
        let memory = self
            .memory
            .get_ref()
            .context("Call context not initialized")?;

        // SAFETY: The caller ensures this memory won't be aliased. It's also
        // valid to transmute from a &[Cell<T>] to &[T].
        let data = WasmPtr::<u8, Array>::new(address)
            .deref_mut(&memory, 0, len)
            .context("Bad pointer")?;

        Ok(std::slice::from_raw_parts_mut(
            data.as_mut_ptr().cast(),
            data.len(),
        ))
    }
}

fn signature_to_wasmer(signature: &Signature) -> wasmer::FunctionType {
    let params: Vec<_> = signature
        .parameters()
        .iter()
        .copied()
        .map(rune_type_to_wasmer_type)
        .collect();
    let returns: Vec<_> = signature
        .returns()
        .iter()
        .copied()
        .map(rune_type_to_wasmer_type)
        .collect();

    wasmer::FunctionType::new(params, returns)
}

fn rune_type_to_wasmer_type(ty: WasmType) -> wasmer::Type {
    match ty {
        WasmType::F32 => wasmer::Type::F32,
        WasmType::F64 => wasmer::Type::F64,
        WasmType::I32 => wasmer::Type::I32,
        WasmType::I64 => wasmer::Type::I64,
    }
}

fn wasmer_to_value(v: &Val) -> Result<WasmValue, RuntimeError> {
    match v {
        Val::I32(int) => Ok(WasmValue::I32(*int)),
        Val::I64(long) => Ok(WasmValue::I64(*long)),
        Val::F32(int) => Ok(WasmValue::F32(*int)),
        Val::F64(int) => Ok(WasmValue::F64(*int)),
        _ => Err(RuntimeError::new("Unsupported wasm type")),
    }
}
fn value_to_wasmer(value: WasmValue) -> Val {
    match value {
        WasmValue::F32(float) => Val::from(float),
        WasmValue::F64(double) => Val::from(double),
        WasmValue::I32(int) => Val::from(int),
        WasmValue::I64(long) => Val::from(long),
    }
}
