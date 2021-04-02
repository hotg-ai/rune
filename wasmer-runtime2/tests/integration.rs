use std::{
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use rune_runtime::{Function, Image, Registrar};
use tempfile::TempDir;
use anyhow::Error;
use wasmer::{Module, Store};
use wasmer_runtime2::Runtime;

#[test]
fn load_and_call_the_empty_rune() {
    let src = include_str!("fixtures/empty.rs");
    let store = Store::default();
    let module = compile_standalone_wasm(src, &store).unwrap();

    let mut runtime =
        Runtime::load_from_module(&module, &store, Empty).unwrap();
    runtime.call().unwrap();
}

struct Empty;

impl Image for Empty {
    fn initialize_imports(self, _: &mut dyn Registrar) {}
}

#[test]
fn call_image_function_from_manifest() {
    let calls = Arc::new(AtomicUsize::new(0));
    let src = include_str!("fixtures/image-function.rs");
    let store = Store::default();
    let module = compile_standalone_wasm(src, &store).unwrap();
    let image = Tracked {
        calls: Arc::clone(&calls),
    };

    let _runtime = Runtime::load_from_module(&module, &store, image).unwrap();

    let times_called = calls.load(Ordering::SeqCst);
    assert_eq!(times_called, 1);
}

struct Tracked {
    calls: Arc<AtomicUsize>,
}

impl Image for Tracked {
    fn initialize_imports(self, registrar: &mut dyn Registrar) {
        let handle = Arc::clone(&self.calls);

        let func = Function::new(move |()| {
            handle.fetch_add(1, Ordering::SeqCst);
            Ok(())
        });
        registrar.register_function("env", "tick", func);
    }
}

fn compile_standalone_wasm(src: &str, store: &Store) -> Result<Module, Error> {
    let temp = TempDir::new()?;
    let temp = temp.path();
    let lib_rs = temp.join("lib.rs");

    std::fs::write(&lib_rs, src.as_bytes())?;

    let dest = temp.join("library.wasm");
    let status = Command::new("rustc")
        .arg(&lib_rs)
        .arg("--crate-type=cdylib")
        .arg("--target=wasm32-unknown-unknown")
        .arg("-o")
        .arg(&dest)
        .arg("-g")
        .status()?;
    anyhow::ensure!(status.success());

    let raw = std::fs::read(&dest)?;
    Module::new(&store, &raw).map_err(Error::from)
}
