#[cfg(feature = "wasm3-runtime")]
mod wasm3_impl;

#[cfg(feature = "wasmer-runtime")]
mod wasmer_impl;

use std::{
    cell::Cell,
    collections::HashMap,
    io::Read,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};
use log::{Level, Record};
use anyhow::Error;
use hotg_rune_core::{Shape, capabilities, outputs, TFLITE_MIMETYPE};
use hotg_rune_runtime::{Capability, Output, common_outputs::Serial};

use crate::random::Random;

type LogFunc = dyn Fn(&Record<'_>) -> Result<(), Error> + Send + Sync + 'static;

pub struct BaseImage {
    capabilities: HashMap<u32, Box<dyn CapabilityFactory>>,
    models: HashMap<String, Box<dyn ModelFactory>>,
    outputs: HashMap<u32, Box<dyn OutputFactory>>,
    resources: HashMap<String, Box<dyn ResourceFactory>>,
    log: Arc<LogFunc>,
}

impl BaseImage {
    pub fn new() -> Self {
        BaseImage {
            capabilities: HashMap::new(),
            models: HashMap::new(),
            outputs: HashMap::new(),
            resources: HashMap::new(),
            log: Arc::new(|_| Ok(())),
        }
    }

    pub fn with_defaults() -> Self {
        let mut image = BaseImage::new();

        image
            .with_logger(default_log_function)
            .register_output(outputs::SERIAL, serial_factory)
            .register_capability(capabilities::RAND, || {
                Ok(Box::new(Random::from_os()) as Box<dyn Capability>)
            });

        image.register_model(TFLITE_MIMETYPE, runecoral::new_model);

        image
    }

    pub fn with_logger<F>(&mut self, log_func: F) -> &mut Self
    where
        F: Fn(&Record<'_>) -> Result<(), Error> + Send + Sync + 'static,
    {
        self.log = Arc::new(log_func);
        self
    }

    pub fn register_capability(
        &mut self,
        id: u32,
        factory: impl CapabilityFactory,
    ) -> &mut Self {
        self.capabilities.insert(id, Box::new(factory));
        self
    }

    pub fn register_output(
        &mut self,
        id: u32,
        factory: impl OutputFactory,
    ) -> &mut Self {
        self.outputs.insert(id, Box::new(factory));
        self
    }

    pub fn register_model(
        &mut self,
        mimetype: &str,
        factory: impl ModelFactory,
    ) -> &mut Self {
        self.models.insert(mimetype.to_string(), Box::new(factory));
        self
    }

    pub fn register_resource(
        &mut self,
        name: &str,
        factory: impl ResourceFactory,
    ) -> &mut Self {
        self.resources.insert(name.to_string(), Box::new(factory));
        self
    }
}

fn default_log_function(record: &Record<'_>) -> Result<(), Error> {
    log::logger().log(record);

    if record.level() > Level::Error {
        Ok(())
    } else {
        Err(anyhow::anyhow!("{}", record.args())
            .context("The Rune encountered a fatal error"))
    }
}

#[derive(Debug, Default, Clone)]
struct Identifiers(Arc<AtomicU32>);

impl Identifiers {
    fn next(&self) -> u32 { self.0.fetch_add(1, Ordering::SeqCst) }
}

impl Default for BaseImage {
    fn default() -> Self { BaseImage::new() }
}

pub trait Model: Send + Sync + 'static {
    /// Run inference on the input tensors, writing the results to `outputs`.
    ///
    /// # Safety
    ///
    /// Implementations can assume that they have unique access to `outputs`
    /// (i.e. converting the `&[Cell<u8>]` to `&mut [u8]` is valid).
    ///
    /// The `inputs` parameter may be aliased.
    unsafe fn infer(
        &mut self,
        inputs: &[&[Cell<u8>]],
        outputs: &[&[Cell<u8>]],
    ) -> Result<(), Error>;

    fn input_shapes(&self) -> &[Shape<'_>];
    fn output_shapes(&self) -> &[Shape<'_>];
}

pub trait ModelFactory: Send + Sync + 'static {
    fn new_model(
        &self,
        model_bytes: &[u8],
        inputs: Option<&[Shape<'_>]>,
        outputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn Model>, Error>;
}

impl<F> ModelFactory for F
where
    F: Fn(
            &[u8],
            Option<&[Shape<'_>]>,
            Option<&[Shape<'_>]>,
        ) -> Result<Box<dyn Model>, Error>
        + Send
        + Sync
        + 'static,
{
    fn new_model(
        &self,
        model_bytes: &[u8],
        inputs: Option<&[Shape<'_>]>,
        outputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn Model>, Error> {
        (*self)(model_bytes, inputs, outputs)
    }
}

pub trait CapabilityFactory: Send + Sync + 'static {
    fn new_capability(&self) -> Result<Box<dyn Capability>, Error>;
}

impl<F> CapabilityFactory for F
where
    F: Fn() -> Result<Box<dyn Capability>, Error> + Send + Sync + 'static,
{
    fn new_capability(&self) -> Result<Box<dyn Capability>, Error> { (*self)() }
}

pub trait OutputFactory: Send + Sync + 'static {
    fn new_output(
        &self,
        inputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn Output>, Error>;
}

impl<F> OutputFactory for F
where
    F: Fn(Option<&[Shape<'_>]>) -> Result<Box<dyn Output>, Error>
        + Send
        + Sync
        + 'static,
{
    fn new_output(
        &self,
        inputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn Output>, Error> {
        (*self)(inputs)
    }
}

fn serial_factory(_: Option<&[Shape<'_>]>) -> Result<Box<dyn Output>, Error> {
    Ok(Box::new(Serial::default()))
}

pub trait ResourceFactory: Send + Sync + 'static {
    fn open_resource(
        &self,
    ) -> Result<Box<dyn Read + Send + Sync + 'static>, Error>;
}

impl<F> ResourceFactory for F
where
    F: Fn() -> Result<Box<dyn Read + Send + Sync + 'static>, Error>
        + Send
        + Sync
        + 'static,
{
    fn open_resource(
        &self,
    ) -> Result<Box<dyn Read + Send + Sync + 'static>, Error> {
        (self)()
    }
}

mod runecoral {
    use super::*;
    use anyhow::{Context, Error};
    use hotg_rune_core::reflect::Type;
    use std::{borrow::Cow, cell::Cell, convert::TryInto, ffi::CStr, sync::Mutex};
    use hotg_runecoral::{
        ElementType, InferenceContext, Tensor, TensorDescriptor, TensorMut,
        AccelerationBackend,
    };

    pub(crate) fn new_model(
        model_bytes: &[u8],
        inputs: Option<&[Shape<'_>]>,
        outputs: Option<&[Shape<'_>]>,
    ) -> Result<Box<dyn Model>, Error> {
        let inputs = inputs.context("The input shapes must be provided")?;
        let outputs = outputs.context("The output shapes must be provided")?;

        let input_descriptors = inputs
            .iter()
            .map(|s| descriptor(s))
            .collect::<Result<Vec<_>, Error>>()
            .context("Invalid input")?;
        let output_descriptors = outputs
            .iter()
            .map(|s| descriptor(s))
            .collect::<Result<Vec<_>, Error>>()
            .context("Invalid output")?;

        let ctx = InferenceContext::create_context(
            TFLITE_MIMETYPE,
            model_bytes,
            AccelerationBackend::NONE,
        )
        .context("Unable to create the inference context")?;

        ensure_shapes_equal(&input_descriptors, &ctx.inputs().collect())?;
        ensure_shapes_equal(&output_descriptors, &ctx.outputs().collect())?;

        Ok(Box::new(RuneCoralModel {
            ctx: Mutex::new(ctx),
            inputs: inputs.iter().map(|s| s.to_owned()).collect(),
            input_descriptors,
            outputs: outputs.iter().map(|s| s.to_owned()).collect(),
            output_descriptors,
        }))
    }

    fn descriptor(s: &Shape) -> Result<TensorDescriptor<'static>, Error> {
        let dimensions: Vec<i32> = s
            .dimensions()
            .iter()
            .copied()
            .map(|d| d.try_into().unwrap())
            .collect();

        Ok(TensorDescriptor {
            name: CStr::from_bytes_with_nul(b"\0").unwrap(),
            element_type: element_type(s.element_type())?,
            shape: Cow::Owned(dimensions),
        })
    }

    struct RuneCoralModel {
        ctx: Mutex<InferenceContext>,
        inputs: Vec<Shape<'static>>,
        input_descriptors: Vec<TensorDescriptor<'static>>,
        outputs: Vec<Shape<'static>>,
        output_descriptors: Vec<TensorDescriptor<'static>>,
    }

    fn element_type(rune_type: &Type) -> Result<ElementType, Error> {
        Ok(match *rune_type {
            Type::i8 => ElementType::Int8,
            Type::u8 => ElementType::UInt8,
            Type::i16 => ElementType::Int16,
            Type::i32 => ElementType::Int32,
            Type::i64 => ElementType::Int64,
            Type::f32 => ElementType::Float32,
            Type::f64 => ElementType::Float64,
            Type::str => ElementType::String,
            _ => {
                anyhow::bail!(
                    "librunecoral doesn't support {:?} tensors",
                    rune_type
                )
            },
        })
    }

    fn ensure_shapes_equal(
        from_rune: &Vec<TensorDescriptor<'_>>,
        from_model: &Vec<TensorDescriptor<'_>>,
    ) -> Result<(), Error> {
        if from_rune.len() == from_model.len()
            && from_rune.iter().zip(from_model.iter()).all(|(x, y)| {
                x.element_type == y.element_type && x.shape == y.shape
            })
        {
            return Ok(());
        }

        fn pretty_shapes(descriptors: &[TensorDescriptor<'_>]) -> String {
            format!(
                "[{}]",
                descriptors
                    .iter()
                    .map(|d| format!("{}", d))
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

    impl super::Model for RuneCoralModel {
        unsafe fn infer(
            &mut self,
            inputs: &[&[Cell<u8>]],
            outputs: &[&[Cell<u8>]],
        ) -> Result<(), Error> {
            let mut ctx = self.ctx.lock().expect("Lock was poisoned");

            let inputs: Vec<Tensor<'_>> = self
                .input_descriptors
                .iter()
                .zip(inputs)
                .map(|(desc, data)| Tensor {
                    element_type: desc.element_type,
                    shape: Cow::Borrowed(&desc.shape),
                    // Safety:
                    buffer: unsafe {
                        std::slice::from_raw_parts(
                            data.as_ptr() as *const u8,
                            data.len(),
                        )
                    },
                })
                .collect();

            let mut outputs: Vec<TensorMut<'_>> = self
                .output_descriptors
                .iter()
                .zip(outputs)
                .map(|(desc, data)| TensorMut {
                    element_type: desc.element_type,
                    shape: Cow::Borrowed(&desc.shape),
                    buffer: unsafe {
                        std::slice::from_raw_parts_mut(
                            data.as_ptr() as *const Cell<u8> as *mut u8,
                            data.len(),
                        )
                    },
                })
                .collect();

            ctx.infer(&inputs, &mut outputs)
                .context("Inference failed")?;

            Ok(())
        }

        fn input_shapes(&self) -> &[Shape<'_>] { &self.inputs }

        fn output_shapes(&self) -> &[Shape<'_>] { &self.outputs }
    }
}
