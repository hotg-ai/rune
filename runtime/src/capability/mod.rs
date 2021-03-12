mod accelerometer;
mod random;

pub use self::random::Random;
pub use self::accelerometer::Accelerometer;

use anyhow::Error;
use runic_types::{CAPABILITY, Type, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CapabilityRequest {
    pub c_type: CAPABILITY,
    pub params: HashMap<String, Value>,
}

impl CapabilityRequest {
    pub fn new(c_type: CAPABILITY) -> CapabilityRequest {
        return CapabilityRequest {
            c_type,
            params: HashMap::new(),
        };
    }
}

pub trait Capability {
    /// Generate the desired input, writing it to the provided buffer and
    /// returning the number of bytes written.
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error>;

    fn set_parameter(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), ParameterError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ParameterError {
    #[error("The \"{}\" parameter isn't supported", name)]
    UnsupportedParameter { name: String },
    #[error("{:?} is an invalid value for \"{}\"", value, name)]
    InvalidValue {
        name: String,
        value: Value,
        #[source]
        reason: Error,
    },
    #[error("Expected a {:?} but found {:?}", expected, actual)]
    IncorrectType { expected: Type, actual: Type },
    #[error("Unable to generate data")]
    DataGenerationFailed(#[from] Error),
}
