use std::fmt::Debug;
use rand::{RngCore, SeedableRng, rngs::SmallRng};
use crate::{Capability, ParameterError};

/// A [`Capability`] that defers to a random number generator.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Random<R>(R);

impl<R> Random<R> {
    pub const fn new(rng: R) -> Self { Random(rng) }
}

impl Random<SmallRng> {
    pub fn from_os() -> Self { Random::new(SmallRng::from_entropy()) }

    pub fn seeded(seed: u64) -> Self {
        Random::new(SmallRng::seed_from_u64(seed))
    }
}

impl Random<DummyRng> {
    pub fn with_repeated_data(data: Vec<u8>) -> Self {
        Random::new(DummyRng::new(data))
    }
}

impl<R: RngCore + Debug + Send + 'static> Capability for Random<R> {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, anyhow::Error> {
        self.0.try_fill_bytes(buffer)?;
        Ok(buffer.len())
    }

    fn set_parameter(
        &mut self,
        _name: &str,
        _value: rune_core::Value,
    ) -> Result<(), ParameterError> {
        Err(ParameterError::UnsupportedParameter)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DummyRng {
    data: Vec<u8>,
}

impl DummyRng {
    fn new(data: Vec<u8>) -> Self {
        assert!(!data.is_empty());
        DummyRng { data }
    }
}

impl RngCore for DummyRng {
    fn next_u32(&mut self) -> u32 {
        let mut buffer = [0; std::mem::size_of::<u32>()];
        self.fill_bytes(&mut buffer);
        u32::from_ne_bytes(buffer)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buffer = [0; std::mem::size_of::<u64>()];
        self.fill_bytes(&mut buffer);
        u64::from_ne_bytes(buffer)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(self.data.len()) {
            let len = chunk.len();
            chunk.copy_from_slice(&self.data[..len]);
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        Ok(self.fill_bytes(dest))
    }
}
