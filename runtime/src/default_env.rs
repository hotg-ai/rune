use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};

use crate::Environment;

#[derive(Debug, Clone)]
pub struct DefaultEnvironment {
    rng: SmallRng,
}

impl DefaultEnvironment {
    pub fn with_seed(seed: [u8; 32]) -> Self {
        DefaultEnvironment {
            rng: SmallRng::from_seed(seed),
        }
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
}
