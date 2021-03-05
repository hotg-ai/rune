use crate::{wasm32::intrinsics, AsParamType, Source, CAPABILITY, Buffer};
use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct Random<B> {
    index: u32,
    _type: PhantomData<fn() -> B>,
}

impl<B: Buffer> Random<B> {
    pub fn new() -> Self {
        unsafe {
            let index = intrinsics::request_capability(CAPABILITY::RAND as u32);

            // ask for the correct length
            let key = "n";
            let value = i32::to_le_bytes(B::OVERALL_LENGTH as i32);
            intrinsics::request_capability_set_param(
                index,
                key.as_ptr(),
                key.len() as u32,
                value.as_ptr(),
                value.len() as u32,
                i32::VALUE as u32,
            );

            Random {
                index,
                _type: PhantomData,
            }
        }
    }
}

impl<B: Buffer> Default for Random<B> {
    fn default() -> Self { Random::new() }
}

impl<B: Buffer> Source for Random<B> {
    type Output = B;

    fn generate(&mut self) -> Self::Output {
        unsafe {
            let mut buffer = B::zeroed();
            let byte_length = core::mem::size_of_val(&buffer);

            let response_size = intrinsics::request_provider_response(
                buffer.as_mut_ptr() as _,
                byte_length as u32,
                self.index as u32,
            );

            debug_assert_eq!(response_size, byte_length as u32);

            buffer
        }
    }
}
