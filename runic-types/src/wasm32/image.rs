use crate::{Source, wasm32::intrinsics, capabilities, Buffer, Value};

pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    index: u32,
}

impl<const WIDTH: usize, const HEIGHT: usize> Image<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        let index = unsafe {
            intrinsics::request_capability(capabilities::IMAGE as u32)
        };

        Image { index }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Image<WIDTH, HEIGHT> {
    fn default() -> Self { Image::new() }
}

impl<const WIDTH: usize, const HEIGHT: usize> Source for Image<WIDTH, HEIGHT> {
    type Output = [[u8; HEIGHT]; WIDTH];

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

// impl<const WIDTH: usize, const HEIGHT: usize> Source for Image<WIDTH, HEIGHT>
// {    type Output = [[[u8; 3]; HEIGHT]; WIDTH];

//    fn generate(&mut self) -> Self::Output {
//        let mut buffer = Self::Output::zeroed();
//        super::copy_capability_data_to_buffer(self.index, &mut buffer);
//        buffer
//    }

//    fn set_parameter(
//        &mut self,
//        key: &str,
//        value: impl Into<Value>,
//    ) -> &mut Self {
//        super::set_capability_parameter(self.index, key, value.into());
//        self
//    }
//}
