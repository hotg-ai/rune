use std::{
    convert::{TryFrom, TryInto},
    fmt::{self, Formatter, Debug},
};
use anyhow::Error;
use image::{GenericImageView, DynamicImage};
use crate::{Capability, ParameterError};
use runic_types::{PixelFormat, Value};

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

    fn invalidate_cache(&mut self) { self.cached = None; }

    fn cached_image(&mut self) -> &DynamicImage {
        let Image {
            original,
            cached,
            processed,
        } = self;

        cached.get_or_insert_with(|| process(original, *processed))
    }
}

impl Capability for Image {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let bytes = self.cached_image().as_bytes();

        let len = std::cmp::min(bytes.len(), buffer.len());
        buffer[..len].copy_from_slice(&bytes[..len]);

        Ok(len)
    }

    fn set_parameter(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), ParameterError> {
        match name {
            "pixel_format" => update_processing_field(self, value, |p| {
                &mut p.desired_pixel_format
            }),
            "width" => {
                update_processing_field(self, value, |p| &mut p.desired_width)
            },
            "height" => {
                update_processing_field(self, value, |p| &mut p.desired_height)
            },
            _ => {
                log::warn!("Unknown parameter: \"{}\" = {}", name, value);
                Ok(())
            },
        }
    }
}

fn update_processing_field<F, T>(
    image: &mut Image,
    value: Value,
    getter: F,
) -> Result<(), ParameterError>
where
    T: TryFrom<Value>,
    T::Error: std::error::Error + Send + Sync + 'static,
    F: FnOnce(&mut ImageProcessing) -> &mut Option<T>,
{
    let dest = getter(&mut image.processed);
    *dest = Some(
        T::try_from(value)
            .map_err(|e| ParameterError::invalid_value(value, e))?,
    );

    image.invalidate_cache();

    Ok(())
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
struct ImageProcessing {
    desired_pixel_format: Option<PixelFormat>,
    desired_width: Option<i32>,
    desired_height: Option<i32>,
}

fn process(image: &DynamicImage, config: ImageProcessing) -> DynamicImage {
    let ImageProcessing {
        desired_pixel_format,
        desired_width,
        desired_height,
    } = config;

    let mut processed = image.clone();

    if let Some(pixel_format) = desired_pixel_format {
        processed = apply_pixel_format(&processed, pixel_format);
    }

    if let Some(resized) = apply_resize(
        &processed,
        desired_width.and_then(|w| w.try_into().ok()),
        desired_height.and_then(|h| h.try_into().ok()),
    ) {
        processed = resized;
    }

    processed
}

fn apply_resize(
    image: &DynamicImage,
    desired_width: Option<u32>,
    desired_height: Option<u32>,
) -> Option<DynamicImage> {
    let (current_width, current_height) = image.dimensions();
    let (desired_width, desired_height) = match (desired_width, desired_height)
    {
        (None, None) => return None,
        (None, Some(height)) => (current_width, height),
        (Some(width), None) => (width, current_height),
        (Some(width), Some(height)) => (width, height),
    };

    Some(image.resize_exact(
        desired_width,
        desired_height,
        image::imageops::FilterType::CatmullRom,
    ))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize_the_image() {
        let mut image =
            Image::new(DynamicImage::ImageRgb8(image::RgbImage::new(128, 128)));

        image.set_parameter("width", Value::from(64)).unwrap();
        assert!(image.cached.is_none());
        assert_eq!(image.processed.desired_width, Some(64));

        let got = image.cached_image();
        assert_eq!(got.dimensions(), (64, 128));
        assert!(image.cached.is_some());

        image.set_parameter("height", Value::from(256)).unwrap();
        assert!(image.cached.is_none());
        assert_eq!(image.processed.desired_height, Some(256));

        let got = image.cached_image();
        assert_eq!(got.dimensions(), (64, 256));
        assert!(image.cached.is_some());
    }
}
