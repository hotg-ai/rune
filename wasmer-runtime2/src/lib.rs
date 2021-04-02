use std::collections::HashMap;
use anyhow::{Context, Error};
use rune_runtime::{Image, Signature, WasmType, WasmValue};
use wasmer::{
    Exports, Function, Instance, Module, NativeFunc, RuntimeError, Store, Val,
};
use wasmer::ImportObject;
use wasmer_vm::Trap;

#[derive(Debug)]
pub struct Runtime {
    instance: Instance,
}

impl Runtime {
    pub fn load_from_module(
        module: &Module,
        store: &Store,
        image: impl Image,
    ) -> Result<Self, Error> {
        log::debug!("Loading image");
        let mut registrar = Registrar::new(store);
        image.initialize_imports(&mut registrar);
        let imports = registrar.import_object();

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
struct Registrar<'s> {
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

    fn import_object(self) -> ImportObject {
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
        ns.insert(
            name,
            Function::new(
                self.store,
                signature_to_wasmer(f.signature()),
                move |args| {
                    let converted = args
                        .iter()
                        .map(wasmer_to_value)
                        .collect::<Result<Vec<WasmValue>, RuntimeError>>()?;
                    f.call(&converted)
                        .map(|ret| {
                            ret.into_iter().map(value_to_wasmer).collect()
                        })
                        .map_err(|e| {
                            RuntimeError::from_trap(Trap::new_from_user(
                                e.into(),
                            ))
                        })
                },
            ),
        );
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
