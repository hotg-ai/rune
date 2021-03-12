use anyhow::{Context, Error};
use image::{RgbImage};

use super::{Capability, ParameterError};

#[derive(Clone, Debug)]
pub struct Image {
    image: RgbImage,
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
        name: &str,
        _value: runic_types::Value,
    ) -> Result<(), ParameterError> {
        Err(ParameterError::unsupported(name))
    }
}
