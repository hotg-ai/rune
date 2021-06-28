use std::{
    fmt::{self, Debug, Formatter},
    fs::File,
    io::{Cursor, Read},
    path::Path,
    time::Duration,
};
use anyhow::{Context, Error};
use hound::WavReader;
use rune_core::{Value};
use crate::{Capability, ParameterError};

const DEFAULT_FREQUENCY: u32 = 16_000;

#[derive(Clone)]
pub struct Sound {
    samples: Vec<i16>,
    frequency: u32,
    duration: Duration,
}

impl Sound {
    pub fn new(samples: Vec<i16>) -> Self {
        Sound {
            samples,
            frequency: DEFAULT_FREQUENCY,
            duration: Duration::from_secs(1),
        }
    }

    pub fn from_wav_data(wav_data: &[u8]) -> Result<Self, Error> {
        let cursor = Cursor::new(wav_data);
        Sound::from_wav(cursor)
    }

    pub fn from_wav_file(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let filename = filename.as_ref();

        let cursor = File::open(filename).with_context(|| {
            format!("Unable to open \"{}\"", filename.display())
        })?;

        Sound::from_wav(cursor)
    }

    pub fn from_wav(reader: impl Read) -> Result<Self, Error> {
        let reader = WavReader::new(reader).unwrap();

        let samples = reader
            .into_samples::<i16>()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Sound::new(samples))
    }
}

impl Capability for Sound {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let chunk_size = std::mem::size_of::<i16>();
        let mut bytes_written = 0;

        for (chunk, sample) in buffer.chunks_mut(chunk_size).zip(&self.samples)
        {
            let sample = sample.to_ne_bytes();
            chunk.copy_from_slice(&sample);

            bytes_written += sample.len();
        }

        Ok(bytes_written)
    }

    fn set_parameter(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), ParameterError> {
        match name {
            "hz" | "frequency" => {
                self.frequency = super::try_from_int_value(value)?;
                Ok(())
            },
            "sample_duration_ms" => {
                let ms = super::try_from_int_value(value)?;
                self.duration = Duration::from_millis(ms);
                Ok(())
            },
            "sample_duration" => {
                let secs = super::try_from_int_value(value)?;
                self.duration = Duration::from_secs(secs);
                Ok(())
            },
            _ => Err(ParameterError::UnsupportedParameter),
        }
    }
}

impl Debug for Sound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Sound {
            samples,
            frequency,
            duration,
        } = self;

        f.debug_struct("Sound")
            .field("samples", &format_args!("({} samples)", samples.len()))
            .field("frequency", frequency)
            .field("duration", duration)
            .finish()
    }
}
