use std::{
    path::Path,
    fs::File,
    io::Read,
    fmt::{Formatter, self, Debug},
    time::Duration,
    convert::TryFrom,
};
use anyhow::{Error, Context};
use hound::{WavSpec, WavReader};

use crate::{Tensor, builtins::Arguments};

/// Load an input from a sound clip, applying any transformations requested by
/// the Rune.
pub fn sound(args: &Arguments, clip: &AudioClip) -> Result<Tensor, Error> {
    let sample_rate: u32 = args.parse("hz")?;
    let sample_duration_ms = args.parse("sample_duration_ms")?;
    let duration = Duration::from_millis(sample_duration_ms);

    let AudioClip { spec, samples } = clip;
    transform_samples(sample_rate, duration, spec, samples)
}

fn transform_samples(
    sample_rate: u32,
    duration: Duration,
    _spec: &WavSpec,
    samples: &[i16],
) -> Result<Tensor, Error> {
    // TODO: actually resample the audio so it will have the correct sample rate
    // instead of blindly copying across the requested number of samples.

    let required_samples = usize::try_from(
        (sample_rate as u128) * duration.as_micros() / 1_000_000,
    )?;

    if samples.len() < required_samples {
        anyhow::bail!(
            "At least {} samples are required to generate this input, but only {} were provided",
            required_samples,
            samples.len(),
        );
    }

    let samples = &samples[..required_samples];

    Ok(Tensor::new(samples, &[1, samples.len()]))
}

#[derive(Clone, PartialEq)]
pub struct AudioClip {
    spec: WavSpec,
    samples: Vec<i16>,
}

impl AudioClip {
    pub fn from_wav_file(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let filename = filename.as_ref();
        let f = File::open(filename).with_context(|| {
            format!("Unable to open \"{}\" for reading", filename.display())
        })?;
        let wav = WavReader::new(f)?;

        AudioClip::load(wav)
    }

    pub fn load(reader: WavReader<impl Read>) -> Result<Self, Error> {
        let spec = reader.spec();
        let samples = reader
            .into_samples()
            .collect::<Result<Vec<i16>, hound::Error>>()
            .context("Unable to parse the WAV file")?;
        Ok(AudioClip { spec, samples })
    }
}

impl Debug for AudioClip {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let AudioClip { spec, samples } = self;
        f.debug_struct("AudioClip")
            .field("spec", spec)
            .field("samples", &format_args!("({} samples)", samples.len()))
            .finish()
    }
}
