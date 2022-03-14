use std::{
    ops::Deref,
    os::raw::{c_char, c_int},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum ErrorKind {
    /// An unidentified error.
    Other = 0,
    /// The WebAssembly isn't valid.
    ///
    /// This typically happens when the WebAssembly failed validation or isn't
    /// actually WebAssembly.
    InvalidWebAssembly = 1,
    /// There was an issue resolving imports.
    ///
    /// This typically occurs when the Rune is expecting different host
    /// functions or some of those functions have different signatures.
    BadImports = 2,
    /// The call into WebAssembly raised an error.
    CallFailed = 3,
}

/// An error that may be returned by the Rune native library.
///
/// # Error Handling
///
/// Fallible functions will return a `*mut Error` which *must* be checked before
/// continuing.
///
/// This might look like...
///
/// ```cpp
/// Runtime *runtime;
/// Config cfg = {...};
///
/// Error *error = rune_runtime_load(&cfg, &runtime);
///
/// if (error) {
///     const char *msg = rune_error_to_string(error);
///
///     printf("Unable to load the Rune: %s\n", msg);
///
///     free(msg);
///     rune_error_free(error);
///     exit(1);
/// }
/// ```
///
/// Additional "return" values are returned via output parameters (typically
/// named `xxx_out`). If an error occurs, the state of the output parameter is
/// unspecified, otherwise it is guaranteed to be in a valid state.
///
/// If an error is present, it is the caller's responsibility to free it
/// afterwards.
pub struct Error(anyhow::Error);

impl Error {
    pub fn boxed(error: impl Into<anyhow::Error>) -> *mut Error {
        let boxed = Box::new(Error(error.into()));
        Box::into_raw(boxed)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Error { Error(e) }
}

impl Deref for Error {
    type Target = anyhow::Error;

    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Create a new `Error` with the provided error message.
///
/// It is the caller's responsibility to free the `Error` using
/// `rune_error_free()` once they are done with it.
#[no_mangle]
pub unsafe extern "C" fn rune_error_new(
    msg: *const c_char,
    len: c_int,
) -> *mut Error {
    let msg = std::slice::from_raw_parts(msg as *const u8, len as usize);
    let error = anyhow::Error::msg(String::from_utf8_lossy(msg));

    Error::boxed(error)
}

/// Get a simple, oneline message describing the error.
///
/// Note: It is the caller's responsibility to free this string afterwards.
#[no_mangle]
pub unsafe extern "C" fn rune_error_to_string(e: *const Error) -> *mut c_char {
    let e = &*e;
    let msg = e.to_string();

    crate::c_str(&msg)
}

/// Print the error, plus any errors that may have caused it.
///
/// If the `RUST_BACKTRACE` environment variable is set, this will also include
/// a backtrace from where the error was first created.
///
/// Note: It is the caller's responsibility to free this string afterwards.
#[no_mangle]
pub unsafe extern "C" fn rune_error_to_string_verbose(
    e: *const Error,
) -> *mut c_char {
    let e = &*e;
    let msg = format!("{:?}", e.0);

    crate::c_str(&msg)
}

/// Free an error once you are done with it.
///
/// Note: Freeing the same `Error` twice is an error and may cause a crash at
/// runtime.
#[no_mangle]
pub unsafe extern "C" fn rune_error_free(e: *mut Error) {
    if e.is_null() {
        return;
    }

    let _ = Box::from_raw(e);
}

/// Programmatically find out what kind of error this is.
#[no_mangle]
pub unsafe extern "C" fn rune_error_kind(e: *const Error) -> ErrorKind {
    if e.is_null() {
        return ErrorKind::Other;
    }

    error_kind_for_anyhow(&*e)
}

fn error_kind_for_anyhow(error: &anyhow::Error) -> ErrorKind {
    for error in error.chain() {
        if let Some(error_kind) = specific_error_kind(error) {
            return error_kind;
        }
    }

    ErrorKind::Other
}

fn specific_error_kind(
    e: &(dyn std::error::Error + 'static),
) -> Option<ErrorKind> {
    if let Some(load_error) = e.downcast_ref::<hotg_rune_runtime::LoadError>() {
        if let Some(kind) = rune_load_error(load_error) {
            return Some(kind);
        }
    }

    #[cfg(feature = "wasmer")]
    if let Some(runtime_error) =
        e.downcast_ref::<hotg_rune_runtime::wasmer::RuntimeError>()
    {
        return Some(wasmer_runtime_error(runtime_error));
    }

    None
}

fn rune_load_error(
    load_error: &hotg_rune_runtime::LoadError,
) -> Option<ErrorKind> {
    match load_error {
        hotg_rune_runtime::LoadError::Other(anyhow) => {
            Some(error_kind_for_anyhow(anyhow))
        },
        #[cfg(feature = "wasmer")]
        hotg_rune_runtime::LoadError::WasmerInstantiation(e) => {
            Some(wasmer_instantiation_error(e))
        },
        #[cfg(feature = "wasmer")]
        hotg_rune_runtime::LoadError::WasmerCompile(e) => {
            Some(wasmer_compile_error(e))
        },
        _ => None,
    }
}

#[cfg(feature = "wasmer")]
fn wasmer_instantiation_error(
    error: &hotg_rune_runtime::wasmer::InstantiationError,
) -> ErrorKind {
    use hotg_rune_runtime::wasmer::{InstantiationError, LinkError};

    match error {
        InstantiationError::Link(LinkError::Import(..)) => {
            ErrorKind::BadImports
        },
        InstantiationError::Start(error) => wasmer_runtime_error(error),
        _ => ErrorKind::Other,
    }
}

#[cfg(feature = "wasmer")]
fn wasmer_runtime_error(
    _error: &hotg_rune_runtime::wasmer::RuntimeError,
) -> ErrorKind {
    // TODO: We need to add a bunch of downcasts here if we want more precise
    // error kinds.
    ErrorKind::CallFailed
}

#[cfg(feature = "wasmer")]
fn wasmer_compile_error(
    error: &hotg_rune_runtime::wasmer::CompileError,
) -> ErrorKind {
    use hotg_rune_runtime::wasmer::{CompileError, WasmError};

    match error {
        CompileError::Wasm(WasmError::InvalidWebAssembly { .. })
        | CompileError::Validate(_) => ErrorKind::InvalidWebAssembly,
        _ => ErrorKind::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_should_be_thin_pointers() {
        assert_eq!(
            std::mem::size_of::<*mut Error>(),
            std::mem::size_of::<*mut usize>()
        );
    }
}
