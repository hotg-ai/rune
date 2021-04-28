use crate::{Tensor, Source, wasm32::intrinsics, Value, HasOutputs};
use core::marker::PhantomData;
use alloc::vec::Vec;

pub type Random<T> = GenericCapability<T, { crate::capabilities::RAND }>;
pub type Accelerometer = GenericCapability<f32, { crate::capabilities::ACCEL }>;
pub type Image = GenericCapability<u8, { crate::capabilities::IMAGE }>;
pub type Sound = GenericCapability<i16, { crate::capabilities::SOUND }>;
pub type Raw<T> = GenericCapability<T, { crate::capabilities::RAW }>;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericCapability<T, const KIND: u32> {
    index: u32,
    output_dimensions: Option<Vec<usize>>,
    _type: PhantomData<fn() -> T>,
}

impl<T, const KIND: u32> GenericCapability<T, KIND> {
    pub fn new() -> Self {
        unsafe {
            let index = intrinsics::request_capability(KIND);

            GenericCapability {
                index,
                output_dimensions: None,
                _type: PhantomData,
            }
        }
    }
}

impl<T, const KIND: u32> Default for GenericCapability<T, KIND> {
    fn default() -> Self { GenericCapability::new() }
}

impl<T: Default + Copy, const KIND: u32> Source for GenericCapability<T, KIND> {
    type Output = Tensor<T>;

    fn generate(&mut self) -> Self::Output {
        let output_dimensions = self
            .output_dimensions
            .as_ref()
            .expect("Please specify the capability's output dimensions");

        let mut buffer = Tensor::zeroed(output_dimensions.to_vec());

        let elements = buffer.make_elements_mut();
        let byte_length = (elements.len() * core::mem::size_of::<T>()) as u32;

        unsafe {
            let response_size = intrinsics::request_provider_response(
                elements.as_mut_ptr() as _,
                byte_length,
                self.index,
            );

            debug_assert_eq!(response_size, byte_length);
        }

        buffer
    }

    fn set_parameter(
        &mut self,
        key: &str,
        value: impl Into<Value>,
    ) -> &mut Self {
        let value = value.into();

        unsafe {
            let mut buffer = Value::buffer();
            let bytes_written = value.to_le_bytes(&mut buffer);

            intrinsics::request_capability_set_param(
                self.index,
                key.as_ptr(),
                key.len() as u32,
                buffer.as_ptr(),
                bytes_written as u32,
                value.ty().into(),
            );
        }

        self
    }
}

impl<T, const KIND: u32> HasOutputs for GenericCapability<T, KIND> {
    fn set_output_dimensions(&mut self, dimensions: &[usize]) {
        self.output_dimensions = Some(dimensions.to_vec());
    }
}
