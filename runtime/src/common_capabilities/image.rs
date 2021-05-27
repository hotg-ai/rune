use std::{
    convert::TryFrom,
    fmt::{self, Formatter, Debug},
};
use anyhow::Error;
use image::{GenericImageView, DynamicImage};
use crate::{Capability, ParameterError};
use runic_types::PixelFormat;

#[derive(Clone, PartialEq)]
pub struct Image {
    original: DynamicImage,
    cached: Option<DynamicImage>,
    processed: ImageProcessing,
}

impl Image {
    pub fn new(image: DynamicImage) -> Self {
        let processed = ImageProcessing::default();

        Image {
            original: image,
            processed,
            cached: None,
        }
    }
}

impl Capability for Image {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let Image {
            original,
            cached,
            processed,
        } = self;

        let bytes = cached
            .get_or_insert_with(|| processed.process(original))
            .as_bytes();

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

                self.processed.set_pixel_format(format);
                Ok(())
            },
            _ => {
                log::warn!("Unknown parameter: \"{}\" = {}", name, value);
                Ok(())
            },
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
struct ImageProcessing {
    desired_pixel_format: Option<PixelFormat>,
}

impl ImageProcessing {
    fn set_pixel_format(&mut self, pixel_format: PixelFormat) {
        self.desired_pixel_format = Some(pixel_format);
    }

    fn process(&self, image: &DynamicImage) -> DynamicImage {
        let mut processed = image.clone();

        if let Some(pixel_format) = self.desired_pixel_format {
            processed = apply_pixel_format(&processed, pixel_format);
        }

        processed
    }
}

fn apply_pixel_format(
    image: &DynamicImage,
    pixel_format: PixelFormat,
) -> DynamicImage {
    match pixel_format {
        PixelFormat::GrayScale => DynamicImage::ImageLuma8(image.to_luma8()),
        PixelFormat::RGB => DynamicImage::ImageRgb8(image.to_rgb8()),
        PixelFormat::BGR => DynamicImage::ImageBgr8(image.to_bgr8()),
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Image {
            original,
            cached,
            processed,
        } = self;

        f.debug_struct("ProcessedImage")
            .field("original", &DebugImage(original))
            .field("cached", &cached.as_ref().map(DebugImage))
            .field("processed", processed)
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
