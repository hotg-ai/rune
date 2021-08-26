use safer_ffi::{derive_ReprC, slice::slice_ref, ffi_export, boxed::Box};
use crate::{image::RunicosBaseImage, BoxedError, RuneResult};
#[allow(unused_imports)]
use std::ops::Not;

trait WasmRuntime {
    fn call(&mut self) -> anyhow::Result<()>;
}

#[cfg(feature = "wasmer-runtime")]
impl WasmRuntime for hotg_rune_wasmer_runtime::Runtime {
    fn call(&mut self) -> anyhow::Result<()> { self.call() }
}

#[cfg(feature = "wasm3-runtime")]
impl WasmRuntime for hotg_rune_wasm3_runtime::Runtime {
    fn call(&mut self) -> anyhow::Result<()> { self.call() }
}

type BoxedWasmRuntime = Box<RuntimeWrapper>;

decl_result_type! {
    type WasmRuntimeResult = Result<BoxedWasmRuntime, BoxedError>;
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct RuntimeWrapper {
    inner: std::boxed::Box<dyn WasmRuntime>,
}

#[cfg(feature = "wasmer-runtime")]
const _: () = {
    /// Load a Rune backed by the provided image.
    ///
    /// If loading is successful, `runtime_out` will be set to a new
    /// `WasmerRuntime` instance, otherwise an error is returned.
    #[ffi_export]
    pub fn rune_wasmer_runtime_load(
        rune: slice_ref<u8>,
        image: Box<RunicosBaseImage>,
    ) -> Box<WasmRuntimeResult> {
        let image: std::boxed::Box<_> = image.into();

        let result =
            match hotg_rune_wasmer_runtime::Runtime::load(&*rune, *image) {
                Ok(r) => Result::Ok(Box::new(RuntimeWrapper {
                    inner: std::boxed::Box::new(r),
                })),
                Err(e) => Result::Err(Box::new(e.into())),
            };

        Box::new(result.into())
    }

    // If wasmer is enabled, we expose it as the default runtime.
    #[ffi_export]
    pub fn rune_default_runtime_load(
        rune: slice_ref<u8>,
        image: Box<RunicosBaseImage>,
    ) -> Box<WasmRuntimeResult> {
        rune_wasmer_runtime_load(rune, image)
    }
};

#[cfg(feature = "wasm3-runtime")]
const _: () = {
    /// Load a Rune backed by the provided image.
    #[ffi_export]
    pub fn rune_wasm3_runtime_load(
        rune: slice_ref<u8>,
        image: Box<RunicosBaseImage>,
    ) -> Box<WasmRuntimeResult> {
        let image: std::boxed::Box<_> = image.into();

        let result =
            match hotg_rune_wasm3_runtime::Runtime::load(&*rune, *image) {
                Ok(r) => Result::Ok(Box::new(RuntimeWrapper {
                    inner: std::boxed::Box::new(r),
                })),
                Err(e) => Result::Err(Box::new(e.into())),
            };

        Box::new(result.into())
    }

    // Wasm3 is only our default runtime when wasmer is disabled.
    // FIXME: `safer-ffi` really dislikes `#[cfg]` so we have to use an
    // anonymous constant here.
    #[cfg(not(feature = "wasmer-runtime"))]
    const _: () = {
        #[ffi_export]
        pub fn rune_default_runtime_load(
            rune: slice_ref<u8>,
            image: Box<RunicosBaseImage>,
        ) -> Box<WasmRuntimeResult> {
            rune_wasm3_runtime_load(rune, image)
        }
    };
};

/// Free a `WasmRuntime` once you are done with it.
#[ffi_export]
pub fn rune_wasm_runtime_free(runtime: Box<RuntimeWrapper>) { drop(runtime); }

/// Evaluate the Rune pipeline.
#[ffi_export]
pub fn rune_wasm_runtime_call(runtime: &mut RuntimeWrapper) -> Box<RuneResult> {
    let result = match runtime.inner.call() {
        Ok(_) => Result::Ok(0),
        Err(e) => Result::Err(Box::new(e.into())),
    };

    Box::new(result.into())
}
