use crate::{wasm32::intrinsics, Source, capabilities, Buffer, Value};
use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct Random<B> {
    index: u32,
    _type: PhantomData<fn() -> B>,
}

impl<B: Buffer> Random<B> {
    pub fn new() -> Self {
        unsafe {
            let index =
                intrinsics::request_capability(capabilities::RAND as u32);

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
