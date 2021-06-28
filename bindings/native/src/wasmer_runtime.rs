use rune_wasmer_runtime::Runtime;
use safer_ffi::{derive_ReprC, slice::slice_ref, ffi_export, boxed::Box};
use crate::{error::Error, image::RunicosBaseImage};
#[allow(unused_imports)]
use std::ops::Not;

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
    runtime_out: *mut Option<Box<WasmerRuntime>>,
) -> Option<Box<Error>> {
    let image: std::boxed::Box<_> = image.into();

    match Runtime::load(&*rune, *image) {
        Ok(r) => {
            let runtime = WasmerRuntime { inner: r };
            unsafe {
                runtime_out.write(Some(Box::new(runtime)));
            }

            None
        },
        Err(e) => {
            unsafe {
                runtime_out.write(None);
            }
            Some(Box::new(e.into()))
        },
    }
}

/// Free a `WasmerRuntime` once you are done with it.
#[ffi_export]
pub fn rune_wasmer_runtime_free(runtime: Box<WasmerRuntime>) { drop(runtime); }

/// Evaluate the Rune pipeline.
#[ffi_export]
pub fn rune_wasmer_runtime_call(
    runtime: &mut WasmerRuntime,
) -> Option<Box<Error>> {
    match runtime.inner.call() {
        Ok(_) => None,
        Err(e) => Some(Box::new(e.into())),
    }
}
