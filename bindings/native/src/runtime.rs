use std::{
    ops::{Deref, DerefMut},
    os::raw::{c_char, c_int, c_void},
    ptr, slice,
};

use hotg_rune_core::SerializableRecord;
use hotg_rune_runtime::{LoadError, Runtime as RustRuntime};
use log::Record;

use crate::{Error, InputTensors, Metadata, OutputTensors};

/// A loaded Rune.
pub struct Runtime {
    inner: RustRuntime,
}

impl Deref for Runtime {
    type Target = RustRuntime;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl DerefMut for Runtime {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

/// Data used when loading a Rune.
#[repr(C)]
pub struct Config {
    pub rune: *const u8,
    pub rune_len: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn rune_runtime_free(runtime: *mut Runtime) {
    if runtime.is_null() {
        return;
    }

    let rt = Box::from_raw(runtime);
    drop(rt);
}

/// Execute the rune, reading from the input tensors that were provided and
/// writing to the output tensors.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn rune_runtime_predict(
    runtime: *mut Runtime,
) -> *mut Error {
    expect!(!runtime.is_null());
    let runtime = &mut *runtime;

    match runtime.inner.predict() {
        Ok(_) => ptr::null_mut(),
        Err(e) => Error::boxed(e),
    }
}

/// Get a set of all the input nodes in this Rune.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn rune_runtime_inputs(
    runtime: *const Runtime,
    metadata_out: *mut *mut Metadata,
) -> *mut Error {
    expect!(!runtime.is_null());
    expect!(!metadata_out.is_null());
    let runtime = &*runtime;

    metadata_out.write(Box::into_raw(Box::new(Metadata::from(
        runtime.capabilities(),
    ))));

    ptr::null_mut()
}

/// Get a set of all the output nodes in this Rune.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn rune_runtime_outputs(
    runtime: *const Runtime,
    metadata_out: *mut *mut Metadata,
) -> *mut Error {
    expect!(!runtime.is_null());
    expect!(!metadata_out.is_null());
    let runtime = &*runtime;

    metadata_out
        .write(Box::into_raw(Box::new(Metadata::from(runtime.outputs()))));

    ptr::null_mut()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn rune_runtime_input_tensors(
    runtime: *mut Runtime,
) -> *mut InputTensors {
    if runtime.is_null() {
        return ptr::null_mut();
    }
    let runtime = &mut *runtime;

    Box::into_raw(Box::new(runtime.input_tensors().into()))
}

/// Get a reference to the tensors associated with each output node.
///
/// This will return `null` if `runtime` is `null`.
///
/// # Safety
///
/// This reference points directly into the runtime's internals, so any use of
/// the runtime while this reference is alive may invalidate it.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn rune_runtime_output_tensors<'rt>(
    runtime: *mut Runtime,
) -> *mut OutputTensors<'rt> {
    if runtime.is_null() {
        return ptr::null_mut();
    }

    let runtime = &*runtime;
    let output_tensors = OutputTensors::from(runtime.output_tensors());

    Box::into_raw(Box::new(output_tensors))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn rune_runtime_load(
    cfg: &Config,
    runtime_out: *mut *mut Runtime,
) -> *mut Error {
    expect!(!cfg.rune.is_null());
    expect!(cfg.rune_len > 0);
    expect!(!runtime_out.is_null());

    let wasm = slice::from_raw_parts(cfg.rune, cfg.rune_len as usize);

    match load(wasm) {
        Ok(inner) => {
            runtime_out.write(Box::into_raw(Box::new(Runtime { inner })));
            std::ptr::null_mut()
        },
        Err(e) => Error::boxed(e),
    }
}

fn load(wasm: &[u8]) -> Result<RustRuntime, LoadError> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "wasmer")] {
            return RustRuntime::wasmer(wasm);
        } else if #[cfg(feature = "wasm3")] {
            return RustRuntime::wasm3(wasm);
        } else {
            let _ = wasm;
            return Err(LoadError::Other(anyhow::Error::msg("")));
        }
    }
}

pub type Logger = unsafe extern "C" fn(*mut c_void, *const c_char, c_int);
type Destructor = unsafe extern "C" fn(*mut c_void);

#[no_mangle]
pub unsafe extern "C" fn rune_runtime_set_logger(
    runtime: *mut Runtime,
    logger: Logger,
    user_data: *mut c_void,
    destructor: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    struct LogThunk {
        logger: Logger,
        user_data: *mut c_void,
        destructor: Option<Destructor>,
    }

    impl LogThunk {
        fn log(&self, record: &Record<'_>) {
            let record = SerializableRecord::from(record);

            if let Ok(serialized) = serde_json::to_string(&record) {
                unsafe {
                    (self.logger)(
                        self.user_data,
                        serialized.as_ptr().cast(),
                        serialized.len() as c_int,
                    );
                }
            }
        }
    }

    impl Drop for LogThunk {
        fn drop(&mut self) {
            if let Some(destructor) = self.destructor {
                unsafe {
                    destructor(self.user_data);
                }
            }
        }
    }

    // Safey: Ensured by the caller.
    unsafe impl Send for LogThunk {}
    unsafe impl Sync for LogThunk {}

    if runtime.is_null() {
        return;
    }

    let runtime = &mut *runtime;
    let thunk = LogThunk {
        logger,
        user_data,
        destructor,
    };

    runtime.set_logger(move |r| thunk.log(r));
}

/// The WebAssembly edngine to use when running a Rune.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum Engine {
    Wasm3 = 0,
    Wasmer = 1,
}
