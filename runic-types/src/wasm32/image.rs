use crate::{Source, wasm32::intrinsics, CAPABILITY};

pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    index: u32,
}

impl<const WIDTH: usize, const HEIGHT: usize> Image<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        let index =
            unsafe { intrinsics::request_capability(CAPABILITY::IMAGE as u32) };

        Image { index }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Image<WIDTH, HEIGHT> {
    fn default() -> Self { Image::new() }
}

impl<const WIDTH: usize, const HEIGHT: usize> Source for Image<WIDTH, HEIGHT> {
    type Output = [[[u8; 3]; HEIGHT]; WIDTH];

    fn generate(&mut self) -> Self::Output {
        let mut buffer = [[[0; 3]; HEIGHT]; WIDTH];
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
