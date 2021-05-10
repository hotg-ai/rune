pub mod audio;

use anyhow::{Context, Error};
use cpal::traits::StreamTrait;

fn main() -> Result<(), Error> {
    let (stream, _samples) =
        audio::start_recording().context("Unable to initialize the audio input")?;

    stream.play().context("Unable to start the stream")?;
    println!("Nothing Broke");

    Ok(())
}
