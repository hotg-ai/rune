use std::{
    fmt::{self, Formatter, Debug},
    convert::TryFrom,
};
use anyhow::Error;
use image::{GenericImageView, DynamicImage};
use crate::{Capability, ParameterError};
use runic_types::PixelFormat;

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    processed_image: ProcessedImage,
}

impl Image {
    pub fn new(image: DynamicImage) -> Self {
        Image {
            processed_image: ProcessedImage::new(image),
        }
    }
}

impl Capability for Image {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let bytes = self.processed_image.processed.as_bytes();

        let len = std::cmp::min(bytes.len(), buffer.len());
        buffer[..len].copy_from_slice(&bytes[..len]);

        Ok(len)
    }

    fn set_parameter(
        &mut self,
        name: &str,
        value: runic_types::Value,
    ) -> Result<(), ParameterError> {
        match name {
            "pixel_format" => {
                let format = PixelFormat::try_from(value).map_err(|e| {
                    ParameterError::InvalidValue {
                        value,
                        reason: Error::msg(e),
                    }
                })?;

                self.processed_image.set_pixel_format(format);
                Ok(())
            },
            _ => {
                log::warn!("Unknown parameter: \"{}\" = {}", name, value);
                Ok(())
            },
        }
    }
}

#[derive(Clone, PartialEq)]
struct ProcessedImage {
    original: DynamicImage,
    processed: DynamicImage,
}

impl ProcessedImage {
    fn new(original: DynamicImage) -> Self {
        let processed = original.clone();

        ProcessedImage {
            original,
            processed,
        }
    }

    fn set_pixel_format(&mut self, pixel_format: PixelFormat) {
        match pixel_format {
            PixelFormat::GrayScale => {
                self.processed =
                    DynamicImage::ImageLuma8(self.original.to_luma8());
            },
            PixelFormat::RGB => {
                self.processed =
                    DynamicImage::ImageRgb8(self.original.to_rgb8());
            },
            PixelFormat::BGR => {
                self.processed =
                    DynamicImage::ImageBgr8(self.original.to_bgr8());
            },
        }
    }
}

impl Debug for ProcessedImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ProcessedImage {
            original,
            processed,
        } = self;

        f.debug_struct("ProcessedImage")
            .field("original_", &DebugImage(original))
            .field("pixel_type", &DebugImage(processed))
            .finish()
    }
}

struct DebugImage<'a>(&'a DynamicImage);

impl<'a> Debug for DebugImage<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let dims = self.0.dimensions();
        let pixel = pixel_type_name(self.0);

        f.debug_struct("Image")
            .field("dimensions", &dims)
            .field("pixel_type", &pixel)
            .finish()
    }
}

fn pixel_type_name(image: &DynamicImage) -> &'static str {
    match image {
        DynamicImage::ImageLuma8(_) => "Luma8",
        DynamicImage::ImageLumaA8(_) => "LumaA8",
        DynamicImage::ImageRgb8(_) => "Rgb8",
        DynamicImage::ImageRgba8(_) => "Rgba8",
        DynamicImage::ImageBgr8(_) => "Bgr8",
        DynamicImage::ImageBgra8(_) => "Bgra8",
        DynamicImage::ImageLuma16(_) => "Luma16",
        DynamicImage::ImageLumaA16(_) => "LumaA16",
        DynamicImage::ImageRgb16(_) => "Rgb16",
        DynamicImage::ImageRgba16(_) => "Rgba16",
    }
}
