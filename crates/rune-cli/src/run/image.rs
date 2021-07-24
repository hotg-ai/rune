use std::{
    fmt::{self, Debug, Formatter},
};
use anyhow::{Context, Error};
use image::{DynamicImage, GenericImageView};
use rune_core::PixelFormat;
use rune_runtime::{ParameterError};
use crate::run::multi::{Builder, SourceBackedCapability};

#[derive(Clone, PartialEq)]
pub struct Image {
    processed: DynamicImage,
}

impl SourceBackedCapability for Image {
    type Builder = ImageSettings;
    type Source = DynamicImage;

    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, anyhow::Error> {
        let bytes = self.processed.as_bytes();

        let len = std::cmp::min(bytes.len(), buffer.len());
        buffer[..len].copy_from_slice(&bytes[..len]);

        Ok(len)
    }

    fn from_builder(
        builder: ImageSettings,
        image: &DynamicImage,
    ) -> Result<Self, anyhow::Error> {
        let (pixel_format, width, height) = builder
            .deconstruct()
            .context("Not all parameters were provided")?;

        let image = image.resize_exact(
            width,
            height,
            image::imageops::FilterType::CatmullRom,
        );

        let image = match pixel_format {
            PixelFormat::GrayScale => {
                DynamicImage::ImageLuma8(image.to_luma8())
            },
            PixelFormat::RGB => DynamicImage::ImageRgb8(image.to_rgb8()),
            PixelFormat::BGR => DynamicImage::ImageBgr8(image.to_bgr8()),
        };

        Ok(Image { processed: image })
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Image { processed } = self;

        f.debug_struct("Image")
            .field("processed", processed)
            .finish()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ImageSettings {
    pixel_format: Option<PixelFormat>,
    width: Option<u32>,
    height: Option<u32>,
}

impl ImageSettings {
    fn deconstruct(self) -> Result<(PixelFormat, u32, u32), Error> {
        let ImageSettings {
            pixel_format,
            width,
            height,
        } = self;

        let pixel_format = pixel_format
            .context("The \"pixel_format\" parameter wasn't set")?;
        let width = width.context("The \"width\" parameter wasn't set")?;
        let height = height.context("The \"height\" parameter wasn't set")?;

        Ok((pixel_format, width, height))
    }
}

impl Builder for ImageSettings {
    fn set_parameter(
        &mut self,
        key: &str,
        value: rune_core::Value,
    ) -> Result<(), ParameterError> {
        let ImageSettings {
            pixel_format,
            width,
            height,
        } = self;

        match key {
            "pixel_format" => super::try_from_int_value(pixel_format, value),
            "width" => super::try_from_int_value(width, value),
            "height" => super::try_from_int_value(height, value),
            _ => Err(ParameterError::UnsupportedParameter),
        }
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
            .finish_non_exhaustive()
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
