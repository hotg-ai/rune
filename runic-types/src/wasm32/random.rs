use crate::{wasm32::intrinsics, AsType, Source, CAPABILITY, Buffer, Value};
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
                i32::TYPE as u32,
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
        let mut buffer = Self::Output::zeroed();
        super::copy_capability_data_to_buffer(self.index, &mut buffer);
        buffer
    }

    fn set_parameter(
        &mut self,
        key: &str,
        value: impl Into<Value>,
    ) -> &mut Self {
        super::set_capability_parameter(self.index, key, value.into());
        self
    }
}
