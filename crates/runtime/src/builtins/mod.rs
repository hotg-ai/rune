//! Builtin modules.

mod accelerometer;
mod arguments;
mod image;
mod random;
mod raw;
mod sound;

use anyhow::Error;

pub use self::{
    accelerometer::{
        accelerometer, AccelerometerParseError, AccelerometerSample,
        AccelerometerSamples,
    },
    arguments::Arguments,
    image::{image, UnknownPixelFormat},
    random::{random, seeded_random},
    raw::raw,
    sound::{sound, AudioClip},
};

/// Use the `"source"` argument to figure out which input to read.
pub fn source<'src, T>(
    sources: &'src [T],
    args: &Arguments,
) -> Result<&'src T, Error> {
    let index: usize = args.parse_or_default("source", 0)?;

    match sources.get(index) {
        Some(source) => Ok(source),
        None if sources.len() == 0 => anyhow::bail!(
            "The user asked for source {}, but no sources were provided",
            index
        ),
        None => anyhow::bail!(
            "The user asked for source {}, but there are only {} sources \
             available",
            index,
            sources.len(),
        ),
    }
}
