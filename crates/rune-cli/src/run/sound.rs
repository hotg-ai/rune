use std::{
    fmt::{self, Debug, Formatter},
    convert::TryFrom,
    fs::File,
    io::Read,
    path::Path,
    time::Duration,
};
use anyhow::{Context, Error};
use hound::{WavReader, WavSpec};
use rune_core::{Value};
use rune_runtime::ParameterError;

use crate::run::multi::{Builder, SourceBackedCapability};

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

#[derive(Debug, Clone, PartialEq)]
pub struct Sound {
    pcm_samples: Vec<i16>,
}

impl SourceBackedCapability for Sound {
    type Builder = SoundSettings;
    type Source = AudioClip;

    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let chunk_size = std::mem::size_of::<i16>();
        let mut bytes_written = 0;

        for (chunk, sample) in
            buffer.chunks_mut(chunk_size).zip(&self.pcm_samples)
        {
            let sample = sample.to_ne_bytes();
            chunk.copy_from_slice(&sample);

            bytes_written += sample.len();
        }

        Ok(bytes_written)
    }

    fn from_builder(
        builder: SoundSettings,
        source: &AudioClip,
    ) -> Result<Self, Error> {
        // TODO: Resample to match the desired sample rate instead of just using
        // frequency and duration to calculate the number of samples.
        let total_samples = builder.total_samples(source.samples.len())?;

        Ok(Sound {
            pcm_samples: source.samples[..total_samples].to_vec(),
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SoundSettings {
    frequency: Option<u32>,
    duration: Option<Duration>,
}

impl SoundSettings {
    fn total_samples(&self, max_samples: usize) -> Result<usize, Error> {
        match *self {
            SoundSettings {
                frequency: None,
                duration: None,
            } => Ok(max_samples),
            SoundSettings {
                frequency: Some(frequency),
                duration: Some(duration),
            } => {
                let total_samples = usize::try_from(
                    (frequency as u128) * duration.as_micros() / 1_000_000,
                )?;

                if total_samples > max_samples {
                    anyhow::bail!(
                        "{} samples were requested but only {} are available",
                        total_samples,
                        max_samples
                    );
                }

                Ok(total_samples)
            },

            SoundSettings { duration: None, .. } => {
                anyhow::bail!("The \"sample_duration\" parameter wasn't set")
            },
            SoundSettings {
                frequency: None, ..
            } => anyhow::bail!("The \"frequency\" parameter wasn't set"),
        }
    }
}

impl Builder for SoundSettings {
    fn set_parameter(
        &mut self,
        key: &str,
        value: Value,
    ) -> Result<(), ParameterError> {
        let SoundSettings {
            frequency,
            duration,
        } = self;

        match key {
            "hz" | "frequency" => super::try_from_int_value(frequency, value),
            "sample_duration_ms" => {
                super::try_from_int_value_and_then(value, |ms| {
                    *duration = Some(Duration::from_millis(ms))
                })
            },
            "sample_duration" => {
                super::try_from_int_value_and_then(value, |secs| {
                    *duration = Some(Duration::from_secs(secs))
                })
            },

            _ => Err(ParameterError::UnsupportedParameter),
        }
    }
}
