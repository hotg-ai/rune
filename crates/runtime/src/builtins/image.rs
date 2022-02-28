use std::{num::NonZeroUsize, str::FromStr};

use anyhow::Error;
use image::{imageops::FilterType, DynamicImage};

use crate::{builtins::Arguments, ElementType, Tensor};

/// Load an input tensor from an image, applying any transformations requested
/// by the Rune.
pub fn image(args: &Arguments, img: &DynamicImage) -> Result<Tensor, Error> {
    let width: u32 = args.parse("width")?;
    let height: u32 = args.parse("height")?;
    let pixel_format: PixelFormat =
        args.parse_or_default("pixel_format", PixelFormat::RGB8)?;

    Ok(transform(img, width, height, pixel_format))
}

fn transform(
    img: &DynamicImage,
    width: u32,
    height: u32,
    pixel_format: PixelFormat,
) -> Tensor {
    let resized = img.resize_exact(width, height, FilterType::CatmullRom);

    let image = match pixel_format {
        PixelFormat::RGB8 => DynamicImage::ImageRgb8(resized.to_rgb8()),
        PixelFormat::BGR8 => DynamicImage::ImageBgr8(resized.to_bgr8()),
        PixelFormat::GrayScale => DynamicImage::ImageLuma8(resized.to_luma8()),
    };

    let dimensions = [
        NonZeroUsize::new(1).unwrap(),
        NonZeroUsize::new(width as usize).unwrap(),
        NonZeroUsize::new(height as usize).unwrap(),
        NonZeroUsize::new(pixel_format.channels()).unwrap(),
    ];
    let buffer = image.as_bytes().to_vec();

    Tensor::new_raw(pixel_format.element_type(), dimensions.to_vec(), buffer)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// Red-Green-Blue pixels stored as `u8`.
    RGB8,
    /// Blue-Green-Red pixels stored as `u8`.
    BGR8,
    /// Grayscale pixels stored as `u8`.
    GrayScale,
}

impl PixelFormat {
    pub fn channels(self) -> usize {
        match self {
            PixelFormat::RGB8 | PixelFormat::BGR8 => 3,
            PixelFormat::GrayScale => 1,
        }
    }

    fn element_type(&self) -> ElementType {
        match self {
            PixelFormat::RGB8 | PixelFormat::BGR8 | PixelFormat::GrayScale => {
                ElementType::U8
            },
        }
    }
}

impl FromStr for PixelFormat {
    type Err = UnknownPixelFormat;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RGB8" => Ok(PixelFormat::RGB8),
            // Legacy
            "@PixelFormat::RGB" | "0" => Ok(PixelFormat::RGB8),
            "@PixelFormat::BGR" | "2" => Ok(PixelFormat::BGR8),
            "@PixelFormat::GrayScale" | "3" => Ok(PixelFormat::GrayScale),
            _ => Err(UnknownPixelFormat),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
#[error("Unknown pixel format")]
pub struct UnknownPixelFormat;
