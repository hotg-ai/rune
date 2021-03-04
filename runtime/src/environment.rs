use std::sync::Mutex;

use log::Record;
use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};
use anyhow::Error;

pub trait Environment: Send + Sync + 'static {
    fn fill_random(&self, _buffer: &mut [u8]) -> Result<(), Error> {
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
}

impl Default for DefaultEnvironment {
    fn default() -> Self { DefaultEnvironment::with_os_seed() }
}

impl Environment for DefaultEnvironment {
    fn fill_random(&self, buffer: &mut [u8]) -> Result<(), Error> {
        self.rng.lock().unwrap().fill_bytes(buffer);

        Ok(())
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
