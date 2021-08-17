use std::{
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use hotg_rune_runtime::Image;
use tempfile::TempDir;
use anyhow::Error;
use hotg_rune_wasm3_runtime::{Registrar, Runtime};
use wasm3::{Environment, ParsedModule};

#[test]
fn load_and_call_a_rune_that_does_nothing() {
    let src = include_str!("fixtures/empty.rs");
    let env = Environment::new().unwrap();
    let module = compile_standalone_wasm(src, &env).unwrap();

    let mut runtime = Runtime::load_from_module(module, &env, Empty).unwrap();
    runtime.call().unwrap();
}

struct Empty;

impl Image<Registrar<'_>> for Empty {
    fn initialize_imports(self, _: &mut Registrar<'_>) {}
}

#[test]
fn call_host_function_from_manifest() {
    let calls = Arc::new(AtomicUsize::new(0));
    let src = include_str!("fixtures/image-function.rs");
    let store = Environment::new().unwrap();
    let module = compile_standalone_wasm(src, &store).unwrap();
    let image = Tracked {
        calls: Arc::clone(&calls),
    };

    let _runtime = Runtime::load_from_module(module, &store, image).unwrap();

    let times_called = calls.load(Ordering::SeqCst);
    assert_eq!(times_called, 1);
}

#[derive(Clone)]
struct Tracked {
    calls: Arc<AtomicUsize>,
}

impl Image<Registrar<'_>> for Tracked {
    fn initialize_imports(self, registrar: &mut Registrar<'_>) {
        let calls = self.calls.clone();
        registrar.register_function("env", "tick", move |_, ()| {
            calls.fetch_add(1, Ordering::SeqCst);
        });
    }
}

fn compile_standalone_wasm(
    src: &str,
    env: &Environment,
) -> Result<ParsedModule, Error> {
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
    Ok(ParsedModule::parse(env, &raw).unwrap())
}
