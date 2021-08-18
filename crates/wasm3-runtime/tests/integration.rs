use std::{
    mem,
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

#[test]
fn load_and_call_a_rune_that_does_nothing() {
    let src = include_str!("fixtures/empty.rs");
    let mut runtime = compile_standalone_wasm(src, Empty).unwrap();
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
    let image = Tracked {
        calls: Arc::clone(&calls),
    };
    let _rt = compile_standalone_wasm(src, image).unwrap();

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

fn compile_standalone_wasm<I>(src: &str, image: I) -> Result<Runtime, Error>
where
    I: for<'a> Image<Registrar<'a>>,
{
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
    let rt = Runtime::load(&raw, image)?;
    // FIXME: work around soundness bug by leaking the raw WASM bytecode
    // (https://github.com/wasm3/wasm3-rs/issues/25)
    mem::forget(raw);
    Ok(rt)
}
