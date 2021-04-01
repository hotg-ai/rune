use std::fmt::Debug;

use rand::RngCore;

use super::{Capability, ParameterError};

/// A [`Capability`] that defers to a random number generator.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Random<R>(R);

impl<R> Random<R> {
    pub const fn new(rng: R) -> Self { Random(rng) }
}

impl<R: RngCore + Debug + Send + 'static> Capability for Random<R> {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, anyhow::Error> {
        self.0.try_fill_bytes(buffer)?;
        Ok(buffer.len())
    }

    fn set_parameter(
        &mut self,
        _name: &str,
        _value: runic_types::Value,
    ) -> Result<(), ParameterError> {
        Err(ParameterError::UnsupportedParameter)
    }
}
