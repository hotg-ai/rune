use std::{borrow::Cow, sync::Mutex};
use log::Record;
use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};
use anyhow::{Context, Error};
use image::RgbImage;
use image::imageops::FilterType;

pub trait Environment: Send + Sync + 'static {
    fn fill_random(&self, _buffer: &mut [u8]) -> Result<usize, Error> {
        Err(Error::new(NotSupportedError))
    }

    fn fill_accelerometer(
        &self,
        _buffer: &mut [[f32; 3]],
    ) -> Result<usize, Error> {
        Err(Error::new(NotSupportedError))
    }

    fn fill_sound(&self, _buffer: &mut [i16]) -> Result<usize, Error> {
        Err(Error::new(NotSupportedError))
    }

    /// Fill the provided buffer with RGB pixels.
    fn fill_image(
        &self,
        _buffer: &mut [u8],
        _width: u32,
        _height: u32,
    ) -> Result<usize, Error> {
        Err(Error::new(NotSupportedError))
    }

    fn log(&self, _msg: &str) {}
}

#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
#[error("Not Supported")]
pub struct NotSupportedError;

#[derive(Debug)]
pub struct DefaultEnvironment {
    rng: Mutex<SmallRng>,
    name: String,
    accelerometer_samples: Vec<[f32; 3]>,
    image: Option<RgbImage>,
    sound: Vec<i16>,
}

impl DefaultEnvironment {
    pub fn with_os_seed() -> Self {
        let mut seed = [0; 32];
        rand::thread_rng().fill(&mut seed);

        DefaultEnvironment::with_seed(seed)
    }

    pub fn with_seed(seed: [u8; 32]) -> Self {
        DefaultEnvironment {
            rng: Mutex::new(SmallRng::from_seed(seed)),
            name: String::from("current_rune"),
            accelerometer_samples: Vec::new(),
            sound: Vec::new(),
            image: None,
        }
    }

    /// Reset the Random Number Generator's seed.
    pub fn seed_rng(&mut self, seed: u64) {
        self.rng = Mutex::new(SmallRng::seed_from_u64(seed));
    }

    pub fn set_accelerometer_data(&mut self, samples: Vec<[f32; 3]>) {
        self.accelerometer_samples = samples;
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn set_image(&mut self, image: RgbImage) { self.image = Some(image); }

    pub fn set_sound(&mut self, sound: Vec<i16>) { self.sound = sound; }
}

impl Default for DefaultEnvironment {
    fn default() -> Self { DefaultEnvironment::with_os_seed() }
}

impl Clone for DefaultEnvironment {
    fn clone(&self) -> Self {
        let DefaultEnvironment {
            rng,
            name,
            accelerometer_samples,
            image,
            sound,
        } = self;
        let rng = rng.lock().unwrap();

        DefaultEnvironment {
            rng: Mutex::new(rng.clone()),
            name: name.clone(),
            image: image.clone(),
            sound: sound.clone(),
            accelerometer_samples: accelerometer_samples.clone(),
        }
    }
}

impl Environment for DefaultEnvironment {
    fn fill_random(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.rng.lock().unwrap().fill_bytes(buffer);

        Ok(buffer.len())
    }

    fn fill_accelerometer(
        &self,
        buffer: &mut [[f32; 3]],
    ) -> Result<usize, Error> {
        if self.accelerometer_samples.is_empty() {
            return Err(Error::new(NotSupportedError));
        }

        let len = std::cmp::min(buffer.len(), self.accelerometer_samples.len());
        buffer.copy_from_slice(&self.accelerometer_samples[..len]);

        Ok(len)
    }

    fn fill_image(
        &self,
        buffer: &mut [u8],
        width: u32,
        height: u32,
    ) -> Result<usize, Error> {
        let img = self.image.as_ref().ok_or(NotSupportedError)?;

        let img = if width == img.width() && height == img.height() {
            Cow::Borrowed(img)
        } else if width == 0 && height == 0 {
            // no dimensions provided, copy the current one across and hope for
            // the best
            Cow::Borrowed(img)
        } else {
            log::debug!(
                "Resizing image from {:?} to {:?}",
                img.dimensions(),
                (width, height)
            );

            Cow::Owned(image::imageops::resize(
                img,
                width,
                height,
                FilterType::Nearest,
            ))
        };

        let raw = img.as_flat_samples();
        let raw = raw.image_slice().context("The image was malformed")?;

        let len = std::cmp::min(raw.len(), buffer.len());
        buffer[..len].copy_from_slice(&raw[..len]);

        Ok(len)
    }

    fn fill_sound(&self, buffer: &mut [i16]) -> Result<usize, Error> {
        if self.sound.is_empty() {
            return Err(Error::new(NotSupportedError));
        }

        let len = std::cmp::min(self.sound.len(), buffer.len());

        buffer[..len].copy_from_slice(&self.sound[..len]);

        Ok(len)
    }

    fn log(&self, msg: &str) {
        // TODO: Update the _debug() function to take a file name and line
        // number.
        log::logger().log(
            &Record::builder()
                .module_path(Some(&self.name))
                .args(format_args!("{}", msg))
                .build(),
        );
    }
}
