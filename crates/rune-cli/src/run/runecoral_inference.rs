use std::{borrow::Cow, cell::Cell, convert::TryInto, path::Path, sync::Mutex};

use anyhow::{Context, Error};
use hotg_rune_core::{Shape, TFLITE_MIMETYPE, reflect::Type};
use hotg_runicos_base_runtime::{BaseImage, Model, ModelFactory};
use hotg_runecoral::{ElementType, InferenceContext, RuneCoral, Tensor, TensorDescriptor, TensorMut};

/// Overrides the TensorFlow Lite model handler with `librunecoral`, if
/// available.
pub fn override_model_handler(
    img: &mut BaseImage,
    library_path: Option<&Path>,
) -> Result<(), Error> {
    let rune_coral = if let Some(path) = library_path {
        // First we try to use the *.so specified by the user
        log::debug!("Loading librunecoral from {:?}", path);
        RuneCoral::load(path)?
    } else if let Ok(rune_coral) = RuneCoral::load("runecoral") {
        // Otherwise we fall back to whatever is installed/accessible on the
        // machine, ignoring any load errors.
        rune_coral
    } else {
        return Ok(());
    };

    img.register_model(
        hotg_rune_core::TFLITE_MIMETYPE,
        RuneCoralModelFactory(rune_coral),
    );
    log::debug!("Installed the librunecoral inference backend");

    Ok(())
}

struct RuneCoralModelFactory(RuneCoral);

impl ModelFactory for RuneCoralModelFactory {
    fn new_model(
        &self,
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

        let ctx = self
            .0
            .create_inference_context(
                TFLITE_MIMETYPE,
                model_bytes,
                &input_descriptors,
                &output_descriptors,
            )
            .context("Unable to create the inference context")?;

        Ok(Box::new(RuneCoralModel {
            ctx: Mutex::new(ctx),
            inputs: inputs.iter().map(|s| s.to_owned()).collect(),
            input_descriptors,
            outputs: outputs.iter().map(|s| s.to_owned()).collect(),
            output_descriptors,
        }))
    }
}

fn descriptor(s: &Shape) -> Result<TensorDescriptor<'static>, Error> {
    let dimensions: Vec<u64> = s
        .dimensions()
        .iter()
        .copied()
        .map(|d| d.try_into().unwrap())
        .collect();

    Ok(TensorDescriptor {
        element_type: element_type(s.element_type())?,
        shape: Cow::Owned(dimensions),
    })
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

struct RuneCoralModel {
    ctx: Mutex<InferenceContext>,
    inputs: Vec<Shape<'static>>,
    input_descriptors: Vec<TensorDescriptor<'static>>,
    outputs: Vec<Shape<'static>>,
    output_descriptors: Vec<TensorDescriptor<'static>>,
}

impl Model for RuneCoralModel {
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
