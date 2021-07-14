use std::collections::HashMap;
use anyhow::{Context, Error};
use rune_runtime::Image;
use wasmer::{Exports, Function, Instance, Module, NativeFunc, Store};
use wasmer::ImportObject;

#[derive(Debug)]
pub struct Runtime {
    instance: Instance,
}

impl Runtime {
    pub fn load<I>(wasm: &[u8], image: I) -> Result<Self, Error>
    where
        I: for<'a> Image<Registrar<'a>>,
    {
        let store = Store::default();
        let module = Module::new(&store, wasm)?;
        Runtime::load_from_module(&module, &store, image)
    }

    pub fn load_from_module<I>(
        module: &Module,
        store: &Store,
        image: I,
    ) -> Result<Self, Error>
    where
        I: for<'a> Image<Registrar<'a>>,
    {
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
pub struct Registrar<'s> {
    namespaces: HashMap<String, Exports>,
    store: &'s Store,
}

impl<'s> Registrar<'s> {
    pub fn new(store: &'s Store) -> Self {
        Registrar {
            namespaces: HashMap::default(),
            store,
        }
    }

    pub fn into_import_object(self) -> ImportObject {
        let mut obj = ImportObject::default();

        for (name, namespace) in self.namespaces {
            obj.register(name, namespace);
        }

        obj
    }

    pub fn store(&self) -> &'s Store { self.store }

    pub fn register_function(
        &mut self,
        namespace: &str,
        name: &str,
        f: Function,
    ) -> &mut Self {
        self.namespaces
            .entry(namespace.to_string())
            .or_default()
            .insert(name, f);

        self
    }
}
