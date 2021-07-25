use rune_runtime::{Capability, ParameterError};
use crate::run::multi::{Builder, SourceBackedCapability};

#[derive(Debug, Clone, PartialEq)]
pub struct Raw {
    bytes: Vec<u8>,
}

impl Raw {
    pub const fn new(bytes: Vec<u8>) -> Self { Raw { bytes } }
}

impl Capability for Raw {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, anyhow::Error> {
        let len = std::cmp::min(buffer.len(), self.bytes.len());
        buffer[..len].copy_from_slice(&self.bytes[..len]);
        Ok(len)
    }

    fn set_parameter(
        &mut self,
        _name: &str,
        _value: rune_core::Value,
    ) -> Result<(), ParameterError> {
        Ok(())
    }
}

impl SourceBackedCapability for Raw {
    type Builder = NullBuilder;
    type Source = Vec<u8>;

    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, anyhow::Error> {
        let len = std::cmp::min(buffer.len(), self.bytes.len());
        buffer[..len].copy_from_slice(&self.bytes[..len]);
        Ok(len)
    }

    fn from_builder(
        _builder: NullBuilder,
        source: &Vec<u8>,
    ) -> Result<Self, anyhow::Error> {
        Ok(Raw::new(source.clone()))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct NullBuilder;

impl Builder for NullBuilder {
    fn set_parameter(
        &mut self,
        _key: &str,
        _value: rune_core::Value,
    ) -> Result<(), ParameterError> {
        Err(ParameterError::UnsupportedParameter)
    }
}
