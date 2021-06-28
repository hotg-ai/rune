use crate::Capability;

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
    ) -> Result<(), crate::ParameterError> {
        Ok(())
    }
}
