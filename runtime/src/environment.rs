use std::{fmt::Debug, sync::Mutex};
use log::Record;
use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};
use anyhow::Error;
use image::RgbImage;
use crate::{
    capability::{Accelerometer, Capability, Image, Random, Sound},
    outputs::{Output, Serial},
};

pub trait Environment: Send + Sync + 'static {
    /// A callback triggered at the start of every pipeline run.
    fn before_call(&self) {}

    /// A callback triggered after every pipeline run, allowing the
    /// [`Environment`] to do any necessary cleanup or synchronisation.
    fn after_call(&self) {}

    fn log(&self, _msg: &str) {}

    fn new_random(&self) -> Result<Box<dyn Capability>, Error> {
        Err(Error::new(NotSupportedError))
    }

    fn new_accelerometer(&self) -> Result<Box<dyn Capability>, Error> {
        Err(Error::new(NotSupportedError))
    }

    fn new_sound(&self) -> Result<Box<dyn Capability>, Error> {
        Err(Error::new(NotSupportedError))
    }

    fn new_image(&self) -> Result<Box<dyn Capability>, Error> {
        Err(Error::new(NotSupportedError))
    }

    fn new_serial(&self) -> Result<Box<dyn Output>, Error> {
        Err(Error::new(NotSupportedError))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
#[error("Not Supported")]
pub struct NotSupportedError;

/// Helper trait which lets us put multiple traits behind a single trait object.
trait DebuggableRng: Debug + RngCore + Send + Sync + 'static {
    fn clone_boxed(&self) -> Box<dyn DebuggableRng>;
}

impl<D: Debug + RngCore + Clone + Send + Sync + 'static> DebuggableRng for D {
    fn clone_boxed(&self) -> Box<dyn DebuggableRng> { Box::new(self.clone()) }
}

#[derive(Debug)]
pub struct DefaultEnvironment {
    rng: Mutex<Box<dyn DebuggableRng>>,
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
            rng: Mutex::new(Box::new(SmallRng::from_seed(seed))),
            name: String::from("current_rune"),
            accelerometer_samples: Vec::new(),
            sound: Vec::new(),
            image: None,
        }
    }

    /// Reset the Random Number Generator's seed.
    pub fn seed_rng(&mut self, seed: u64) {
        self.rng = Mutex::new(Box::new(SmallRng::seed_from_u64(seed)));
    }

    pub fn set_random_data(&mut self, random_data: Vec<u8>) {
        self.rng = Mutex::new(Box::new(PhonyRng::new(random_data)));
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
            rng: Mutex::new(rng.clone_boxed()),
            name: name.clone(),
            image: image.clone(),
            sound: sound.clone(),
            accelerometer_samples: accelerometer_samples.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct PhonyRng {
    data: std::iter::Cycle<std::vec::IntoIter<u8>>,
}

impl PhonyRng {
    fn new(data: Vec<u8>) -> Self {
        PhonyRng {
            data: data.into_iter().cycle(),
        }
    }
}

impl RngCore for PhonyRng {
    fn next_u32(&mut self) -> u32 { rand_core::impls::next_u32_via_fill(self) }

    fn next_u64(&mut self) -> u64 { rand_core::impls::next_u64_via_fill(self) }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for (src, dest) in self.data.by_ref().zip(dest) {
            *dest = src;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl Environment for DefaultEnvironment {
    fn new_random(&self) -> Result<Box<dyn Capability>, Error> {
        let rng = self.rng.lock().unwrap().clone_boxed();

        Ok(Box::new(Random::new(rng)))
    }

    fn new_accelerometer(&self) -> Result<Box<dyn Capability>, Error> {
        if self.accelerometer_samples.is_empty() {
            return Err(Error::new(NotSupportedError));
        }

        Ok(Box::new(Accelerometer::new(
            self.accelerometer_samples.clone(),
        )))
    }

    fn new_image(&self) -> Result<Box<dyn Capability>, Error> {
        match &self.image {
            Some(image) => Ok(Box::new(Image::new(image.clone()))),
            None => Err(Error::from(NotSupportedError)),
        }
    }

    fn new_sound(&self) -> Result<Box<dyn Capability>, Error> {
        if self.sound.is_empty() {
            return Err(Error::new(NotSupportedError));
        }

        Ok(Box::new(Sound::new(self.sound.clone())))
    }

    fn new_serial(&self) -> Result<Box<dyn Output>, Error> {
        Ok(Box::new(Serial::default()))
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
