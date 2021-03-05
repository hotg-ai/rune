use crate::{wasm32::intrinsics, Transform, Buffer};
use core::marker::PhantomData;

/// A machine learning model.
pub struct Model<Input, Output> {
    /// A unique identifier we can use to refer to the model.
    // FIXME: Change the VM to allow multiple models.
    #[allow(dead_code)]
    index: u32,
    _type: PhantomData<fn(Input) -> Output>,
}

impl<In, Out, const M: usize, const N: usize> Model<[In; M], [Out; N]> {
    /// Loads a model into the VM.
    pub fn load(raw_blob: &[u8]) -> Self {
        unsafe {
            let ix = intrinsics::tfm_preload_model(
                raw_blob.as_ptr(),
                raw_blob.len() as u32,
                M as u32,
                N as u32,
            );

            Model {
                index: ix,
                _type: PhantomData,
            }
        }
    }
}

impl<In, Out> Transform<In> for Model<In, Out>
where
    Out: Buffer,
    In: Buffer,
{
    type Output = Out;

    fn transform(&mut self, input: In) -> Out {
        unsafe {
            let input_length = core::mem::size_of_val(&input);

            let mut output = Out::zeroed();
            let output_length = core::mem::size_of_val(&output);

            let _ret = intrinsics::tfm_model_invoke(
                self.index,
                input.as_ptr() as *const u8,
                input_length as u32,
                output.as_mut_ptr() as *mut u8,
                output_length as u32,
            );

            output
        }
    }
}
