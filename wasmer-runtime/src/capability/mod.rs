mod accelerometer;
mod image;
mod random;
mod sound;

pub use self::{
    accelerometer::Accelerometer, random::Random, sound::Sound, image::Image,
};

use anyhow::Error;
use runic_types::{InvalidConversionError, Value};
use std::fmt::Debug;
use std::convert::{TryFrom, TryInto};

pub trait Capability: Send + Debug + 'static {
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
    #[error("The parameter isn't supported")]
    UnsupportedParameter,
    #[error("{:?} is an invalid value", value)]
    InvalidValue {
        value: Value,
        #[source]
        reason: Error,
    },
    #[error("{}", _0)]
    IncorrectType(InvalidConversionError),
}

fn try_from_int_value<T>(value: Value) -> Result<T, ParameterError>
where
    T: TryFrom<i32>,
    T::Error: Into<Error>,
{
    let integer: i32 = value
        .try_into()
        .map_err(|e| ParameterError::IncorrectType(e))?;

    T::try_from(integer).map_err(|e| ParameterError::InvalidValue {
        value,
        reason: e.into(),
    })
}
