//! Foreign Function Interface to the Rune Runtime.

mod callbacks;
mod capability;
mod environment;
mod output;

pub use callbacks::Callbacks;
pub use capability::Capability;
use environment::Environment;
pub use output::Output;

use std::{
    os::raw::{c_char, c_int},
};

#[repr(C)]
pub enum RuntimeResult {
    Ok(*mut Runtime),
    Err(*mut Error),
}

/// A handle to the Rune runtime.
pub struct Runtime(rune_runtime::Runtime);

/// Load a new Rune from its WebAssembly, using the provided `Environment` to
/// interact with the outside world.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn rune_runtime_load(
    wasm: *const u8,
    len: c_int,
    callbacks: Callbacks,
) -> RuntimeResult {
    let wasm = std::slice::from_raw_parts(wasm, len as usize);
    let env = Environment::new(callbacks);

    match rune_runtime::Runtime::load(wasm, env) {
        Ok(r) => RuntimeResult::Ok(Box::into_raw(Box::new(Runtime(r)))),
        Err(e) => RuntimeResult::Err(Box::into_raw(Box::new(Error(e)))),
    }
}

/// Invoke the runtime.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn rune_runtime_call(
    runtime: *mut Runtime,
) -> *mut Error {
    let runtime = &mut *runtime;
    match runtime.0.call() {
        Ok(_) => std::ptr::null_mut(),
        Err(e) => Box::into_raw(Box::new(Error(e))),
    }
}

/// Free the `Runtime` once it is no longer needed.
#[no_mangle]
pub unsafe extern "C" fn rune_runtime_free(runtime: *mut Runtime) {
    if !runtime.is_null() {
        let _ = Box::from_raw(runtime);
    }
}

pub struct Error(anyhow::Error);

#[no_mangle]
pub unsafe extern "C" fn rune_error_free(error: *mut Error) {
    if !error.is_null() {
        let _ = Box::from_raw(error);
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn rune_error_msg(error: *const Error) -> *const c_char {
    let error = &*error;
    let mut msg = format!("{:?}", error.0);
    msg.push('\0');

    libc::strdup(msg.as_ptr().cast())
}
