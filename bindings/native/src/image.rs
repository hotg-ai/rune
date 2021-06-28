use log::Record;
use rune_runtime::Image;
use runicos_base_runtime::BaseImage;
use safer_ffi::{
    boxed::Box, closure::BoxDynFnMut1, derive_ReprC, ffi_export,
    slice::slice_ref,
};
use std::{
    convert::TryInto,
    ops::{Deref, DerefMut},
    sync::Mutex,
};
#[allow(unused_imports)]
use std::ops::Not;

use crate::error::Error;

/// A table containing the various host functions to be provided to the Rune.
///
/// Each host function is a closure which may contain its own state.
#[cfg_attr(
    feature = "tflite",
    doc = "\n By default, the `tflite` crate will be used for model inference."
)]
#[cfg_attr(not(feature = "tflite"), doc = "\n")]
#[derive_ReprC]
#[ReprC::opaque]
pub struct RunicosBaseImage {
    inner: BaseImage,
}

impl Deref for RunicosBaseImage {
    type Target = BaseImage;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl DerefMut for RunicosBaseImage {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl Image for RunicosBaseImage {
    fn initialize_imports(self, registrar: &mut dyn rune_runtime::Registrar) {
        self.inner.initialize_imports(registrar);
    }
}

#[ffi_export]
pub fn rune_image_new() -> Box<RunicosBaseImage> { todo!() }

#[ffi_export]
pub fn rune_image_free(image: Box<RunicosBaseImage>) { drop(image); }

/// Set the closure to be called when the Rune emits log messages.
#[ffi_export]
pub fn rune_image_set_log(
    image: &mut RunicosBaseImage,
    log: BoxDynFnMut1<Option<Box<Error>>, LogRecord>,
) {
    let log = Mutex::new(log);

    image.with_log(move |record| {
        let record = LogRecord::from(record);

        let mut log = log.lock().unwrap();

        match log.call(record) {
            Some(error) => {
                let boxed_error: std::boxed::Box<Error> = error.into();
                Err((*boxed_error).into_inner())
            },
            None => Ok(()),
        }
    });
}

#[derive_ReprC]
#[repr(C)]
pub struct LogRecord {
    level: LogLevel,
    /// A UTF-8 string specifying where this log record was emitted.
    target: safer_ffi::slice::slice_raw<u8>,
    /// The log message itself as a UTF-8 string.
    message: safer_ffi::char_p::char_p_boxed,
}

impl<'a, 'b> From<&'a Record<'b>> for LogRecord {
    fn from(r: &'a Record<'b>) -> Self {
        let message = r.args().to_string();

        LogRecord {
            level: r.level().into(),
            target: slice_ref::from(r.target().as_bytes()).into(),
            message: message
                .try_into()
                .expect("Log messages don't contain internal nulls"),
        }
    }
}

#[derive_ReprC]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum LogLevel {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = 1,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}

impl From<log::Level> for LogLevel {
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Error => LogLevel::Error,
            log::Level::Warn => LogLevel::Warn,
            log::Level::Info => LogLevel::Info,
            log::Level::Debug => LogLevel::Debug,
            log::Level::Trace => LogLevel::Trace,
        }
    }
}
