use crate::{Source, wasm32::intrinsics, CAPABILITY, Buffer};

pub struct Audio<const N: usize> {
    index: u32,
}

impl<const N: usize> Audio<N> {
    pub fn new() -> Self {
        let index =
            unsafe { intrinsics::request_capability(CAPABILITY::SOUND as u32) };

        Accelerometer { index }
    }
}

impl<const N: usize> Default for Audio<N> {
    fn default() -> Self { Audio::new() }
}

impl<const N: usize> Source for Audio<N> {
    type Output = [i16; N];

    fn generate(&mut self) -> Self::Output {
        let mut buffer = Self::Output::zeroed();
        super::copy_capability_data_to_buffer(self.index, &mut buffer);
        buffer
    }
}
