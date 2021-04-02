use std::collections::HashMap;

use anyhow::{Context, Error};
use rune_runtime::{Image, Signature, WasmValue};
use wasmer::{Exports, Function, Instance, Module, NativeFunc, Store, Val};
use wasmer::ImportObject;

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
                    let converted: Vec<_> =
                        args.iter().map(wasmer_to_value).collect();
                    let ret = f.call(&converted).unwrap_or_else(|e| unsafe {
                        wasmer::raise_user_trap(e.into())
                    });

                    Ok(ret.into_iter().map(value_to_wasmer).collect())
                },
            ),
        );
    }
}

fn signature_to_wasmer(_signature: &Signature) -> wasmer::FunctionType {
    todo!()
}

fn wasmer_to_value(_v: &Val) -> WasmValue { todo!() }
fn value_to_wasmer(_value: WasmValue) -> Val { todo!() }
