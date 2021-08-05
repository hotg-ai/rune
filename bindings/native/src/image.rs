use anyhow::Context;
use log::Record;
use hotg_rune_runtime::Image;
use hotg_runicos_base_runtime::{BaseImage, ModelFactory};
use hotg_rune_core::{Shape, Value};
use safer_ffi::{
    boxed::Box,
    char_p::{char_p_raw, char_p_ref},
    closure::{BoxDynFnMut0, BoxDynFnMut1},
    derive_ReprC, ffi_export,
    slice::{slice_mut, slice_raw, slice_ref},
};
use std::{
    convert::{TryInto, TryFrom},
    cell::Cell,
    ffi::c_void,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::Mutex,
};
#[allow(unused_imports)]
use std::ops::Not;
use crate::{error::Error, RuneResult, BoxedError};

decl_result_type! {
    type IntegerOrErrorResult = Result<usize, BoxedError>;
    type CapabilityResult = Result<Capability, BoxedError>;
    type ModelResult = Result<Model, BoxedError>;
}

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

impl<R> Image<R> for RunicosBaseImage
where
    BaseImage: Image<R>,
{
    fn initialize_imports(self, registrar: &mut R) {
        self.inner.initialize_imports(registrar);
    }
}

/// Create a vtable containing the various host functions to be provided to the
/// Rune.
///
/// Each host function is a closure which may contain its own state.
///
/// On Linux and MacOS, interence is performed using the `tflite` crate
/// (bindings to TensorFlow Lite). Other platforms will need to specify the
/// model factory manually (see `rune_image_register_model_handler()`).
#[ffi_export]
pub fn rune_image_new() -> Box<RunicosBaseImage> {
    Box::new(RunicosBaseImage {
        inner: BaseImage::with_defaults(),
    })
}

#[ffi_export]
pub fn rune_image_free(image: Box<RunicosBaseImage>) { drop(image); }

/// Set the closure to be called when the Rune emits log messages.
#[ffi_export]
pub fn rune_image_set_log(
    image: &mut RunicosBaseImage,
    factory: BoxDynFnMut1<Box<RuneResult>, LogRecord>,
) {
    let factory = Mutex::new(factory);

    image.with_logger(move |record| {
        let record = LogRecord::from(record);

        let mut factory = factory.lock().unwrap();
        let result: std::boxed::Box<_> = factory.call(record).into();

        match result.into_std() {
            Result::Ok(_) => Ok(()),
            Result::Err(e) => {
                let boxed: std::boxed::Box<Error> = e.into();
                Err(boxed.into_inner())
            },
        }
    });
}

#[derive_ReprC]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CapabilityType {
    Rand = hotg_rune_core::capabilities::RAND,
    Sound = hotg_rune_core::capabilities::SOUND,
    Accel = hotg_rune_core::capabilities::ACCEL,
    Image = hotg_rune_core::capabilities::IMAGE,
    Raw = hotg_rune_core::capabilities::RAW,
}

#[ffi_export]
pub fn rune_image_set_capability_handler(
    image: &mut RunicosBaseImage,
    capability: CapabilityType,
    factory: BoxDynFnMut0<Box<CapabilityResult>>,
) {
    let factory = Mutex::new(factory);

    image.register_capability(capability as u32, move || {
        let mut factory = factory.lock().unwrap();
        let result: std::boxed::Box<_> = factory.call().into();

        match result.into_std() {
            Ok(v) => {
                let boxed: std::boxed::Box<_> = v.into();
                Ok(boxed as std::boxed::Box<dyn hotg_rune_runtime::Capability>)
            },
            Err(e) => {
                let boxed: std::boxed::Box<Error> = e.into();
                Err((*boxed).into_inner())
            },
        }
    });
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
/// An object which can be used to generate data.
///
/// # Safety
///
/// It is the implementor's responsibility to ensure this type is thread-safe.
pub struct Capability {
    user_data: Option<NonNull<c_void>>,
    set_parameter: Option<unsafe extern "C" fn(*mut c_void)>,
    generate: Option<
        unsafe extern "C" fn(
            *mut c_void,
            buffer: slice_raw<u8>,
        ) -> Box<IntegerOrErrorResult>,
    >,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
}

impl hotg_rune_runtime::Capability for Capability {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, anyhow::Error> {
        unsafe {
            let buffer = slice_mut::from(buffer);
            let buffer = slice_raw::from(buffer);

            let user_data = match self.user_data {
                Some(p) => p.as_ptr(),
                None => std::ptr::null_mut(),
            };

            let generate =
                self.generate.context("Generate function not initialized")?;

            let result: std::boxed::Box<_> = generate(user_data, buffer).into();
            match result.into_std() {
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
    ) -> Result<(), hotg_rune_runtime::ParameterError> {
        todo!()
    }
}

unsafe impl Send for Capability {}
unsafe impl Sync for Capability {}

/// Register a function for generating models with the specified mimetype.
///
/// Note: TensorFlow Lite models have a mimetype of "application/tflite-model".
#[ffi_export]
pub fn rune_image_register_model_handler(
    image: &mut RunicosBaseImage,
    mimetype: char_p_ref,
    factory: BoxDynFnMut1<Box<ModelResult>, slice_raw<u8>>,
) {
    let factory = Mutex::new(factory);

    image.register_model(mimetype.to_str(), NativeModelFactory(factory));
}

struct NativeModelFactory(Mutex<BoxDynFnMut1<Box<ModelResult>, slice_raw<u8>>>);

impl ModelFactory for NativeModelFactory {
    fn new_model(
        &self,
        model_bytes: &[u8],
        inputs: Option<&[hotg_rune_core::Shape<'_>]>,
        outputs: Option<&[hotg_rune_core::Shape<'_>]>,
    ) -> Result<
        std::boxed::Box<dyn hotg_runicos_base_runtime::Model>,
        anyhow::Error,
    > {
        let mut factory = self.0.lock().unwrap();
        let result: std::boxed::Box<ModelResult> =
            factory.call(slice_ref::from(model_bytes).into()).into();

        match result.into_std() {
            Ok(v) => {
                let model = NativeModel::try_from(v)?;

                if let Some(inputs) = inputs {
                    ensure_shapes_equal(inputs, &model.inputs)
                        .context("Invalid inputs")?;
                }
                if let Some(outputs) = outputs {
                    ensure_shapes_equal(outputs, &model.outputs)
                        .context("Invalid outputs")?;
                }

                Ok(std::boxed::Box::new(model)
                    as std::boxed::Box<dyn hotg_runicos_base_runtime::Model>)
            },
            Err(e) => {
                let boxed: std::boxed::Box<Error> = e.into();
                Err((*boxed).into_inner())
            },
        }
    }
}

fn ensure_shapes_equal(
    from_rune: &[Shape],
    from_model: &[Shape],
) -> Result<(), anyhow::Error> {
    if from_rune == from_model {
        return Ok(());
    }

    fn pretty_shapes(shapes: &[Shape<'_>]) -> String {
        format!(
            "[{}]",
            shapes
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    anyhow::bail!(
            "The Rune said tensors would be {}, but the model said they would be {}",
            pretty_shapes(from_rune),
            pretty_shapes(from_model),
        );
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
/// An object which can do model inference.
///
/// # Safety
///
/// It is the implementor's responsibility to ensure this type is thread-safe.
pub struct Model {
    user_data: Option<NonNull<c_void>>,
    generate: Option<
        unsafe extern "C" fn(
            *mut c_void,
            input: slice_raw<slice_raw<u8>>,
            output: slice_raw<slice_raw<u8>>,
        ) -> Box<IntegerOrErrorResult>,
    >,
    inputs: Option<unsafe extern "C" fn(*mut c_void) -> slice_raw<char_p_raw>>,
    outputs: Option<unsafe extern "C" fn(*mut c_void) -> slice_raw<char_p_raw>>,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
}

/// A wrapper around a [`Model`] which implements the interface expected by our
/// base image.
struct NativeModel {
    vtable: Model,
    inputs: Vec<Shape<'static>>,
    outputs: Vec<Shape<'static>>,
}

impl TryFrom<Model> for NativeModel {
    type Error = anyhow::Error;

    fn try_from(vtable: Model) -> Result<NativeModel, Self::Error> {
        let inputs = Vec::new();
        let outputs = Vec::new();

        Ok(NativeModel {
            vtable,
            inputs,
            outputs,
        })
    }
}

impl hotg_runicos_base_runtime::Model for NativeModel {
    unsafe fn infer(
        &mut self,
        input: &[&[Cell<u8>]],
        output: &[&[Cell<u8>]],
    ) -> Result<(), anyhow::Error> {
        let generate = self
            .vtable
            .generate
            .context("No generate function provided")?;
        let user_data = match self.vtable.user_data {
            Some(u) => u.as_ptr(),
            None => std::ptr::null_mut(),
        };

        let input: Vec<_> = input
            .iter()
            .map(|s| {
                std::slice::from_raw_parts(s.as_ptr() as *const u8, s.len())
            })
            .map(slice_ref::from)
            .map(slice_raw::from)
            .collect();
        let output: Vec<_> = output
            .iter()
            .map(|s| {
                std::slice::from_raw_parts(s.as_ptr() as *const u8, s.len())
            })
            .map(slice_ref::from)
            .map(slice_raw::from)
            .collect();
        generate(
            user_data,
            slice_ref::from(input.as_slice()).into(),
            slice_ref::from(output.as_slice()).into(),
        );

        Ok(())
    }

    fn input_shapes(&self) -> &[hotg_rune_core::Shape<'_>] { &self.inputs }

    fn output_shapes(&self) -> &[hotg_rune_core::Shape<'_>] { &self.outputs }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

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
