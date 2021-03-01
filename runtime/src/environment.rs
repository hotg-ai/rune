use log::Record;
use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};

pub trait Environment: 'static {
    fn rng(&mut self) -> Option<&mut dyn RngCore> { None }

    fn log(&mut self, _msg: &str) {}
}

#[derive(Debug, Clone)]
pub struct DefaultEnvironment {
    rng: SmallRng,
    accelerometer_samples: Vec<[f32; 3]>,
}

impl DefaultEnvironment {
    pub fn with_seed(seed: [u8; 32]) -> Self {
        DefaultEnvironment {
            rng: SmallRng::from_seed(seed),
            accelerometer_samples: Vec::new(),
        }
    }

    /// Reset the [`Environment::rng()`]'s seed.
    pub fn seed_rng(&mut self, seed: u64) {
        self.rng = SmallRng::seed_from_u64(seed);
    }

    pub fn set_accelerometer_data(&mut self, samples: Vec<[f32; 3]>) {
        self.accelerometer_samples = samples;
    }
}

impl Default for DefaultEnvironment {
    fn default() -> Self {
        let mut seed = [0; 32];
        rand::thread_rng().fill(&mut seed);

        DefaultEnvironment::with_seed(seed)
    }
}

impl Environment for DefaultEnvironment {
    fn rng(&mut self) -> Option<&mut dyn RngCore> { Some(&mut self.rng) }

    fn log(&mut self, msg: &str) {
        // TODO: Update the _debug() function to take a file name and line
        // number.
        log::logger().log(
            &Record::builder()
                .module_path(Some("current_rune"))
                .args(format_args!("{}", msg))
                .build(),
        );
    }
}
