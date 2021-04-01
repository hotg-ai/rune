use std::fmt::{self, Formatter, Debug};
use anyhow::{Context, Error};
use image::{GenericImageView, RgbImage};
use super::{Capability, ParameterError};

#[derive(Clone)]
pub struct Image {
    image: RgbImage,
}

impl Image {
    pub fn new(image: RgbImage) -> Self { Image { image } }
}

impl Capability for Image {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let raw = &self.image.as_flat_samples();
        let raw = raw.image_slice().context("The image was malformed")?;

        // lol, let's hope the caller knows how big their image is.
        let len = std::cmp::min(raw.len(), buffer.len());
        buffer[..len].copy_from_slice(&raw[..len]);

        Ok(len)
    }

    fn set_parameter(
        &mut self,
        _name: &str,
        _value: runic_types::Value,
    ) -> Result<(), ParameterError> {
        Err(ParameterError::UnsupportedParameter)
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Image { image } = self;

        let dims = image.dimensions();
        let pixel = pixel_type_name(image);

        f.debug_struct("Image")
            .field("dimensions", &dims)
            .field("pixel_type", &pixel)
            .finish()
    }
}

fn pixel_type_name<I>(_image: &I) -> &'static str
where
    I: GenericImageView,
{
    std::any::type_name::<I::Pixel>()
}
