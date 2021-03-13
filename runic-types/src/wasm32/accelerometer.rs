use crate::{Source, wasm32::intrinsics, capabilities, Buffer, Value};

pub struct Accelerometer<const N: usize> {
    index: u32,
}

impl<const N: usize> Accelerometer<N> {
    pub fn new() -> Self {
        let index = unsafe {
            intrinsics::request_capability(capabilities::ACCEL as u32)
        };

        Accelerometer { index }
    }
}

impl<const N: usize> Default for Accelerometer<N> {
    fn default() -> Self { Accelerometer::new() }
}

impl<const N: usize> Source for Accelerometer<N> {
    type Output = [[f32; 3]; N];

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
