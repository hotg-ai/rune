use crate::{wasm32::intrinsics, Transform, Tensor, HasOutputs};
use alloc::vec::Vec;
use core::marker::PhantomData;

/// A machine learning model.
pub struct Model<Input, Output> {
    /// A unique identifier we can use to refer to the model.
    // FIXME: Change the VM to allow multiple models.
    #[allow(dead_code)]
    index: u32,
    output_dimensions: Option<Vec<usize>>,
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
                output_dimensions: None,
                _type: PhantomData,
            }
        }
    }
}

impl<In, Out: Default> Transform<Tensor<In>> for Model<In, Out> {
    type Output = Tensor<Out>;

    fn transform(&mut self, input: Tensor<In>) -> Tensor<Out> {
        unsafe {
            let (input_ptr, input_len) = input.as_ptr_and_byte_length();

            let output_dimensions = self
                .output_dimensions
                .as_ref()
                .expect("Please specify the model's output dimensions");

            let len: usize = output_dimensions.iter().product();
            let mut output_buffer = Vec::new();
            output_buffer.resize_with(len, Out::default);

            let _ret = intrinsics::tfm_model_invoke(
                self.index,
                input_ptr,
                input_len as u32,
                output_buffer.as_mut_ptr().cast(),
                (core::mem::size_of::<Out>() * len) as u32,
            );

            Tensor::new_row_major(
                output_buffer.into(),
                output_dimensions.to_vec(),
            )
        }
    }
}

impl<In, Out> HasOutputs for Model<In, Out> {
    fn set_output_dimensions(&mut self, dimensions: &[usize]) {
        self.output_dimensions = Some(dimensions.to_vec());
    }
}
