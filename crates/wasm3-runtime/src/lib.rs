use anyhow::{anyhow, Error};
use hotg_rune_runtime::Image;
use wasm3::{
    CallContext, Environment, Function, Module, ParsedModule, WasmArgs,
    WasmType,
};

const STACK_SIZE: u32 = 1024 * 16;

// FIXME: `wasm3`'s error type isn't Send or Sync since it contains a raw
// pointer, so it's not `anyhow`-compatible. We work around that by formatting
// it.
trait Wasm3ResultExt<T> {
    fn to_anyhow(self) -> Result<T, anyhow::Error>;
}

impl<T> Wasm3ResultExt<T> for Result<T, wasm3::error::Error> {
    fn to_anyhow(self) -> Result<T, anyhow::Error> {
        self.map_err(|e| anyhow!("{}", e))
    }
}

#[derive(Debug)]
pub struct Runtime {
    rt: wasm3::Runtime,
}

impl Runtime {
    pub fn load<I>(wasm: &[u8], image: I) -> Result<Self, Error>
    where
        I: for<'a> Image<Registrar<'a>>,
    {
        let env = Environment::new().to_anyhow()?;
        let module = ParsedModule::parse(&env, wasm).to_anyhow()?;

        let rt = env.create_runtime(STACK_SIZE).to_anyhow()?;

        log::debug!("Instantiating the WebAssembly module");
        let instance = rt.load_module(module).to_anyhow()?;

        log::debug!("Loading image");
        let mut registrar = Registrar::new(instance);
        image.initialize_imports(&mut registrar);

        // TODO: Rename the _manifest() method to _start() so it gets
        // automatically invoked while instantiating.
        let manifest: Function<(), i32> =
            registrar.module.find_function("_manifest").to_anyhow()?;
        manifest.call().to_anyhow()?;

        log::debug!("Loaded the Rune");

        Ok(Runtime { rt })
    }

    pub fn call(&mut self) -> Result<(), Error> {
        log::debug!("Running the rune");

        let call_func: Function<(i32, i32, i32), i32> =
            self.rt.find_function("_call").to_anyhow()?;

        // For some reason we pass in the RAND capability ID when it's meant
        // to be the Rune's responsibility to remember it. Similarly we are
        // passing in the sine model's output type as the "input_type" parameter
        // even though the model should know that.
        //
        // We should be able to change the _call function's signature once
        // hotg-ai/rune#28 lands.
        call_func.call(0, 0, 0).to_anyhow()?;

        Ok(())
    }
}

pub struct Registrar<'m> {
    module: Module<'m>,
}

impl<'m> Registrar<'m> {
    pub fn new(module: Module<'m>) -> Self { Self { module } }

    pub fn register_function<Args, Ret, F>(
        &mut self,
        namespace: &str,
        name: &str,
        f: F,
    ) -> &mut Self
    where
        Args: WasmArgs,
        Ret: WasmType,
        F: for<'cc> FnMut(CallContext<'cc>, Args) -> Ret + 'static,
    {
        match self.module.link_closure(namespace, name, f) {
            Ok(()) => {},
            Err(wasm3::error::Error::FunctionNotFound) => {
                // This error occurs when we try to link a function into the
                // program that the program doesn't import. We
                // just ignore that error here, since that is fine.
            },
            Err(e) => {
                panic!(
                    "wasm3 register_function failed for `{}::{}`: {}",
                    namespace, name, e
                );
            },
        }
        self
    }
}
