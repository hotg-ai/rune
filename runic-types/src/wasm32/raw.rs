use crate::{wasm32::intrinsics, Source, capabilities, Buffer, Value};
use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct Raw<B> {
    index: u32,
    _type: PhantomData<fn() -> B>,
}

impl<B: Buffer> Raw<B> {
    pub fn new() -> Self {
        unsafe {
            let index =
                intrinsics::request_capability(capabilities::RAW as u32);

            Raw {
                index,
                _type: PhantomData,
            }
        }
    }
}

impl<B: Buffer> Default for Raw<B> {
    fn default() -> Self { Raw::new() }
}

impl<B: Buffer> Source for Raw<B> {
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

