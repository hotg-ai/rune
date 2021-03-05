use crate::{Source, wasm32::intrinsics, CAPABILITY};

pub struct Accelerometer<const N: usize> {
    index: u32,
}

impl<const N: usize> Accelerometer<N> {
    pub fn new() -> Self {
        let index =
            unsafe { intrinsics::request_capability(CAPABILITY::ACCEL as u32) };

        Accelerometer { index }
    }
}

impl<const N: usize> Default for Accelerometer<N> {
    fn default() -> Self { Accelerometer::new() }
}

impl<const N: usize> Source for Accelerometer<N> {
    type Output = [[f32; 3]; N];

    fn generate(&mut self) -> Self::Output {
        let mut buffer = [[0.0; 3]; N];
        let byte_length = core::mem::size_of_val(&buffer) as u32;

        unsafe {
            let response_size = intrinsics::request_provider_response(
                buffer.as_mut_ptr() as _,
                byte_length,
                self.index as u32,
            );

            debug_assert_eq!(response_size, byte_length);
        }

        buffer
    }
}
