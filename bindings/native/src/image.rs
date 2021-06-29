use anyhow::Context;
use log::Record;
use rune_runtime::Image;
use runicos_base_runtime::BaseImage;
use rune_core::Value;
use crate::result::Result;
use safer_ffi::{
    boxed::Box,
    char_p::char_p_ref,
    closure::{BoxDynFnMut0, BoxDynFnMut1},
    derive_ReprC, ffi_export,
    slice::{slice_mut, slice_raw, slice_ref},
};
use std::{
    convert::TryInto,
    ffi::c_void,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::Mutex,
};
#[allow(unused_imports)]
use std::ops::Not;
use crate::error::Error;

type StdResult<T, E> = std::result::Result<T, E>;

#[derive_ReprC]
#[ReprC::opaque]
/// A table containing the various host functions to be provided to the Rune.
///
/// Each host function is a closure which may contain its own state.
#[cfg_attr(
    feature = "tflite",
    doc = "\n By default, the `tflite` crate will be used for model inference."
)]
#[cfg_attr(not(feature = "tflite"), doc = "\n")]
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
pub fn rune_image_new() -> Box<RunicosBaseImage> {
    Box::new(RunicosBaseImage {
        inner: BaseImage::new(),
    })
}

#[ffi_export]
pub fn rune_image_free(image: Box<RunicosBaseImage>) { drop(image); }

/// Set the closure to be called when the Rune emits log messages.
#[ffi_export]
pub fn rune_image_set_log(
    image: &mut RunicosBaseImage,
    log: BoxDynFnMut1<Result<u8, Box<Error>>, LogRecord>,
) {
    let log = Mutex::new(log);

    image.with_log(move |record| {
        let record = LogRecord::from(record);

        let mut log = log.lock().unwrap();

        match log.call(record) {
            Result::Ok(_) => Ok(()),
            Result::Err(e) => {
                let boxed: std::boxed::Box<Error> = e.into();
                Err(boxed.into_inner())
            },
        }
    });
}

#[ffi_export]
pub fn rune_image_set_raw(
    image: &mut RunicosBaseImage,
    raw: BoxDynFnMut0<Result<Capability, Box<Error>>>,
) {
    let raw = Mutex::new(raw);

    image.with_raw(move || match StdResult::from(raw.lock().unwrap().call()) {
        Ok(v) => Ok(std::boxed::Box::new(v)),
        Err(e) => {
            let boxed: std::boxed::Box<Error> = e.into();
            Err((*boxed).into_inner())
        },
    });
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
pub struct Capability {
    user_data: Option<NonNull<c_void>>,
    set_parameter: Option<unsafe extern "C" fn(*mut c_void)>,
    generate: Option<
        unsafe extern "C" fn(
            *mut c_void,
            buffer: slice_raw<u8>,
        ) -> Result<usize, Box<Error>>,
    >,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
}

impl rune_runtime::Capability for Capability {
    fn generate(
        &mut self,
        buffer: &mut [u8],
    ) -> StdResult<usize, anyhow::Error> {
        unsafe {
            let buffer = slice_mut::from(buffer);
            let buffer = slice_raw::from(buffer);

            let user_data = match self.user_data {
                Some(p) => p.as_ptr(),
                None => std::ptr::null_mut(),
            };

            let generate =
                self.generate.context("Generate function not initialized")?;
            match generate(user_data, buffer) {
                Result::Ok(v) => Ok(v),
                Result::Err(e) => {
                    let boxed: std::boxed::Box<Error> = e.into();
                    Err(boxed.into_inner())
                },
            }
        }
    }

    fn set_parameter(
        &mut self,
        _name: &str,
        _value: Value,
    ) -> StdResult<(), rune_runtime::ParameterError> {
        todo!()
    }
}

unsafe impl Send for Capability {}
unsafe impl Sync for Capability {}

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

#[ffi_export]
pub fn rune_log_level_name(level: LogLevel) -> char_p_ref<'static> {
    match level {
        LogLevel::Error => safer_ffi::c!("Error"),
        LogLevel::Warn => safer_ffi::c!("Warn"),
        LogLevel::Info => safer_ffi::c!("Info"),
        LogLevel::Debug => safer_ffi::c!("Debug"),
        LogLevel::Trace => safer_ffi::c!("Trace"),
    }
}
