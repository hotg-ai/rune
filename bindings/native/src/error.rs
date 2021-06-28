use std::{convert::TryInto, ops::Deref};
#[allow(unused_imports)]
use std::ops::Not;
use safer_ffi::{boxed::Box, char_p::char_p_boxed, derive_ReprC, ffi_export};

/// An error that may be returned from this library.
#[derive_ReprC]
#[ReprC::opaque]
pub struct Error {
    inner: anyhow::Error,
}

impl Error {
    pub fn into_inner(self) -> anyhow::Error { self.inner }
}

impl From<anyhow::Error> for Error {
    fn from(inner: anyhow::Error) -> Self { Error { inner } }
}

impl Deref for Error {
    type Target = anyhow::Error;

    fn deref(&self) -> &Self::Target { &self.inner }
}

/// Construct a new error.
#[ffi_export]
pub fn rune_error_new(msg: safer_ffi::char_p::char_p_ref) -> Box<Error> {
    let msg = String::from_utf8_lossy(msg.to_bytes()).into_owned();

    Box::new(Error {
        inner: anyhow::Error::msg(msg),
    })
}

/// Free the error once you are done with it.
#[ffi_export]
pub fn rune_error_free(e: Box<Error>) { drop(e); }

/// Return a newly allocated string containing the error's backtrace.
#[ffi_export]
pub fn rune_error_backtrace(error: &Error) -> char_p_boxed {
    error
        .backtrace()
        .to_string()
        .try_into()
        .expect("Should never contain interior nulls")
}

/// Return a newly allocated string describing the error.
#[ffi_export]
pub fn rune_error_to_string(error: &Error) -> char_p_boxed {
    format!("{}", error.inner)
        .try_into()
        .expect("Should never contain interior nulls")
}

/// Return a newly allocated string describing the error and any errors that
/// may have caused it.
///
/// This will also contain a backtrace if the `RUST_BACKTRACE` environment
/// variable is set.
#[ffi_export]
pub fn rune_error_to_string_verbose(error: &Error) -> char_p_boxed {
    format!("{:?}", error.inner)
        .try_into()
        .expect("Should never contain interior nulls")
}
