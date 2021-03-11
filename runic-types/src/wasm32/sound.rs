use crate::{Source, wasm32::intrinsics, CAPABILITY, Buffer, Value};

pub struct Sound<const N: usize> {
    index: u32,
}

impl<const N: usize> Sound<N> {
    pub fn new() -> Self {
        let index =
            unsafe { intrinsics::request_capability(CAPABILITY::SOUND as u32) };

        Sound { index }
    }
}

impl<const N: usize> Default for Sound<N> {
    fn default() -> Self { Sound::new() }
}

impl<const N: usize> Source for Sound<N> {
    type Output = [i16; N];

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
