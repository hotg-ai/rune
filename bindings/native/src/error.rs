use std::{
    ops::Deref,
    os::raw::{c_char, c_int},
};

/// An error that may be returned by the Rune native library.
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
