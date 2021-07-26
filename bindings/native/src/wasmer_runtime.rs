use hotg_rune_wasmer_runtime::Runtime;
use safer_ffi::{derive_ReprC, slice::slice_ref, ffi_export, boxed::Box};
use crate::{image::RunicosBaseImage, BoxedError, RuneResult};
#[allow(unused_imports)]
use std::ops::Not;

type BoxedWasmerRuntime = Box<WasmerRuntime>;

decl_result_type! {
    type WasmerRuntimeResult = Result<BoxedWasmerRuntime, BoxedError>;
}

/// A Rune runtime backed by `wasmer`.
#[derive_ReprC]
#[ReprC::opaque]
pub struct WasmerRuntime {
    inner: Runtime,
}

/// Load a Rune backed by the provided image.
///
/// If loading is successful, `runtime_out` will be set to a new `WasmerRuntime`
/// instance, otherwise an error is returned.
#[ffi_export]
pub fn rune_wasmer_runtime_load(
    rune: slice_ref<u8>,
    image: Box<RunicosBaseImage>,
) -> Box<WasmerRuntimeResult> {
    let image: std::boxed::Box<_> = image.into();

    let result = match Runtime::load(&*rune, *image) {
        Ok(r) => Result::Ok(Box::new(WasmerRuntime { inner: r })),
        Err(e) => Result::Err(Box::new(e.into())),
    };

    Box::new(result.into())
}

/// Free a `WasmerRuntime` once you are done with it.
#[ffi_export]
pub fn rune_wasmer_runtime_free(runtime: Box<WasmerRuntime>) { drop(runtime); }

/// Evaluate the Rune pipeline.
#[ffi_export]
pub fn rune_wasmer_runtime_call(
    runtime: &mut WasmerRuntime,
) -> Box<RuneResult> {
    let result = match runtime.inner.call() {
        Ok(_) => Result::Ok(0),
        Err(e) => Result::Err(Box::new(e.into())),
    };

    Box::new(result.into())
}
