use hotg_rune_core::{Tensor, Value, Shape};
use crate::intrinsics;
use core::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct Capability<T> {
    id: u32,
    shape: Shape<'static>,
    _type: PhantomData<fn() -> T>,
}

impl<T: Clone + Default> Capability<T> {
    pub fn new(kind: u32, shape: Shape<'static>) -> Self {
        unsafe {
            let id = intrinsics::request_capability(kind);

            Capability {
                id,
                shape,
                _type: PhantomData,
            }
        }
    }

    pub fn generate(&mut self) -> Tensor<T> {
        let output_dimensions = self.shape.dimensions();

        let mut buffer = Tensor::zeroed(output_dimensions.to_vec());

        let elements = buffer.make_elements_mut();
        let byte_length = (elements.len() * core::mem::size_of::<T>()) as u32;

        unsafe {
            let response_size = intrinsics::request_provider_response(
                elements.as_mut_ptr() as _,
                byte_length,
                self.id,
            );

            debug_assert_eq!(response_size, byte_length);
        }

        buffer
    }

    pub fn set_parameter(
        &mut self,
        key: &str,
        value: impl Into<Value>,
    ) -> &mut Self {
        let value = value.into();

        unsafe {
            let mut buffer = Value::buffer();
            let bytes_written = value.to_le_bytes(&mut buffer);

            intrinsics::request_capability_set_param(
                self.id,
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
