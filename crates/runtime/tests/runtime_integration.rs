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

#[cfg(feature = "wasm3")]
mod wasm3_runtime {
    use super::*;
    use hotg_rune_runtime::wasm3::{Runtime, Registrar};

    #[test]
    fn load_and_call_a_rune_that_does_nothing() {
        let src = include_str!("fixtures/empty.rs");
        let wasm = compile_standalone_wasm(src).unwrap();

        let mut runtime = Runtime::load(&wasm, Empty).unwrap();
        runtime.call().unwrap();
    }

    #[test]
    fn call_host_function_from_manifest() {
        let calls = Arc::new(AtomicUsize::new(0));
        let src = include_str!("fixtures/image-function.rs");
        let image = Tracked {
            calls: Arc::clone(&calls),
        };
        let wasm = compile_standalone_wasm(src).unwrap();

        let _runtime = Runtime::load(&wasm, image).unwrap();

        let times_called = calls.load(Ordering::SeqCst);
        assert_eq!(times_called, 1);
    }

    impl Image<Registrar<'_>> for Tracked {
        fn initialize_imports(self, registrar: &mut Registrar<'_>) {
            let calls = self.calls;
            registrar.register_function("env", "tick", move |_, ()| {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            });
        }
    }
}

#[cfg(feature = "wasmer")]
mod wasmer_runtime {
    use super::*;
    use hotg_rune_runtime::wasmer::{Registrar, Runtime};
    use ::wasmer::Function;

    #[test]
    fn load_and_call_a_rune_that_does_nothing() {
        let src = include_str!("fixtures/empty.rs");
        let wasm = compile_standalone_wasm(src).unwrap();

        let mut runtime = Runtime::load(&wasm, Empty).unwrap();
        runtime.call().unwrap();
    }

    #[test]
    fn call_host_function_from_manifest() {
        let calls = Arc::new(AtomicUsize::new(0));
        let src = include_str!("fixtures/image-function.rs");
        let image = Tracked {
            calls: Arc::clone(&calls),
        };
        let wasm = compile_standalone_wasm(src).unwrap();

        let _runtime = Runtime::load(&wasm, image).unwrap();

        let times_called = calls.load(Ordering::SeqCst);
        assert_eq!(times_called, 1);
    }

    impl Image<Registrar<'_>> for Tracked {
        fn initialize_imports(self, registrar: &mut Registrar<'_>) {
            let func = Function::new_native_with_env(
                registrar.store(),
                self,
                |t: &Tracked| {
                    t.calls.fetch_add(1, Ordering::SeqCst);
                },
            );

            registrar.register_function("env", "tick", func);
        }
    }
}

struct Empty;

impl<R> Image<R> for Empty {
    fn initialize_imports(self, _: &mut R) {}
}

#[derive(Clone)]
#[cfg_attr(feature = "wasmer", derive(wasmer::WasmerEnv))]
struct Tracked {
    calls: Arc<AtomicUsize>,
}

fn compile_standalone_wasm(src: &str) -> Result<Vec<u8>, Error> {
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

    std::fs::read(&dest).map_err(Error::from)
}
