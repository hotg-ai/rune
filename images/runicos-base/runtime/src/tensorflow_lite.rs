use anyhow::{Context, Error};
use hotg_rune_core::{ElementType as RuneElementType, Shape, TFLITE_MIMETYPE};
use std::{borrow::Cow, cell::Cell, convert::TryInto, ffi::CStr, sync::Mutex};
use hotg_runecoral::{
    ElementType, InferenceContext, Tensor, TensorDescriptor, TensorMut,
    AccelerationBackend,
};

use crate::Model;

/// Create a new [`Model`] backed by [`hotg_runecoral`].
pub fn new_model(
    model_bytes: &[u8],
    inputs: Option<&[Shape<'_>]>,
    outputs: Option<&[Shape<'_>]>,
) -> Result<Box<dyn Model>, Error> {
    let inputs = inputs.context("The input shapes must be provided")?;
    let outputs = outputs.context("The output shapes must be provided")?;

    let input_descriptors = inputs
        .iter()
        .map(descriptor)
        .collect::<Result<Vec<_>, Error>>()
        .context("Invalid input")?;
    let output_descriptors = outputs
        .iter()
        .map(descriptor)
        .collect::<Result<Vec<_>, Error>>()
        .context("Invalid output")?;

    let ctx = InferenceContext::create_context(
        TFLITE_MIMETYPE,
        model_bytes,
        AccelerationBackend::NONE,
    )
    .context("Unable to create the inference context")?;

    let model_input_descriptors: Vec<_> = ctx.inputs().collect();
    ensure_shapes_equal(&input_descriptors, &model_input_descriptors)?;
    let model_output_descriptors: Vec<_> = ctx.outputs().collect();
    ensure_shapes_equal(&output_descriptors, &model_output_descriptors)?;

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

fn element_type(rune_type: RuneElementType) -> Result<ElementType, Error> {
    Ok(match rune_type {
        RuneElementType::I8 => ElementType::Int8,
        RuneElementType::U8 => ElementType::UInt8,
        RuneElementType::I16 => ElementType::Int16,
        RuneElementType::I32 => ElementType::Int32,
        RuneElementType::I64 => ElementType::Int64,
        RuneElementType::F32 => ElementType::Float32,
        RuneElementType::F64 => ElementType::Float64,
        RuneElementType::String => ElementType::String,
        _ => {
            anyhow::bail!(
                "librunecoral doesn't support {:?} tensors",
                rune_type
            )
        },
    })
}

fn ensure_shapes_equal(
    from_rune: &[TensorDescriptor<'_>],
    from_model: &[TensorDescriptor<'_>],
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
