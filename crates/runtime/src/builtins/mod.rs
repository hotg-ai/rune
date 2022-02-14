//! Builtin modules.

mod accelerometer;
mod arguments;
mod image;
mod random;
mod raw;
mod sound;

pub use self::{
    arguments::Arguments,
    accelerometer::{
        AccelerometerSample, AccelerometerSamples, accelerometer,
        AccelerometerParseError,
    },
    image::{image, UnknownPixelFormat},
    sound::{AudioClip, sound},
    raw::raw,
    random::{random, seeded_random},
};

use anyhow::{Error, Context};

/// Use the `"source"` argument to figure out which input to read.
pub fn source<'src, T>(
    sources: &'src [T],
    args: &Arguments,
) -> Result<&'src T, Error> {
    let index: usize = args.parse_or_default("source", 0)?;

    sources.get(index)
        .with_context(|| format!(
            "The user asked for source {}, but there are only {} sources available",
            index,
            sources.len(),
        ))
}
