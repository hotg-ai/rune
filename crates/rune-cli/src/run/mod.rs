mod accelerometer;
mod command;
mod image;
pub mod multi;
mod raw;
mod runecoral_inference;
mod sound;

use hotg_rune_runtime::ParameterError;

pub use self::{
    accelerometer::Accelerometer, sound::Sound, image::Image, raw::Raw,
    multi::new_multiplexer, command::Run,
};

use hotg_rune_core::Value;
use std::convert::{TryFrom, TryInto};

pub(crate) fn try_from_int_value<T>(
    dest: &mut Option<T>,
    value: Value,
) -> Result<(), ParameterError>
where
    T: TryFrom<i32>,
    T::Error: std::error::Error + Send + Sync + 'static,
{
    try_from_int_value_and_then(value, |v| *dest = Some(v))
}

pub(crate) fn try_from_int_value_and_then<T>(
    value: Value,
    and_then: impl FnOnce(T),
) -> Result<(), ParameterError>
where
    T: TryFrom<i32>,
    T::Error: std::error::Error + Send + Sync + 'static,
{
    let integer: i32 = value
        .try_into()
        .map_err(|e| ParameterError::IncorrectType(e))?;

    match T::try_from(integer) {
        Ok(value) => {
            and_then(value);
            Ok(())
        },
        Err(e) => Err(ParameterError::invalid_value(value, e)),
    }
}
