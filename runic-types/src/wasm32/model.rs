use crate::{wasm32::intrinsics, Transform, Tensor};
use alloc::vec::Vec;
use core::marker::PhantomData;

/// A machine learning model.
pub struct Model<Input, Output> {
    /// A unique identifier we can use to refer to the model.
    // FIXME: Change the VM to allow multiple models.
    #[allow(dead_code)]
    index: u32,
    _type: PhantomData<fn(Input) -> Output>,
}

impl<In, Out> Model<In, Out> {
    /// Loads a model into the VM.
    pub fn load(raw_blob: &[u8]) -> Self {
        unsafe {
            let ix = intrinsics::tfm_preload_model(
                raw_blob.as_ptr(),
                raw_blob.len() as u32,
                1,
                1,
            );

            Model {
                index: ix,
                _type: PhantomData,
            }
        }
    }
}

impl<In, Out: Default> Transform<Tensor<In>> for Model<In, Out> {
    type Output = Tensor<Out>;

    fn transform(&mut self, input: Tensor<In>) -> Tensor<Out> {
        unsafe {
            let input_buffer = input.elements();

            // FIXME: properly calculate the output dimensions
            let dimensions = alloc::vec![0];

            let len: usize = dimensions.iter().product();
            let mut output_buffer = Vec::new();
            output_buffer.resize_with(len, Out::default);

            let _ret = intrinsics::tfm_model_invoke(
                self.index,
                input_buffer.as_ptr() as *const u8,
                core::mem::size_of_val(input_buffer) as u32,
                output_buffer.as_mut_ptr() as *mut u8,
                core::mem::size_of_val(output_buffer.as_slice()) as u32,
            );

            Tensor::new_row_major(output_buffer.into(), dimensions)
        }
    }
}
