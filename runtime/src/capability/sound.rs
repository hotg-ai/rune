use std::{
    fmt::{self, Debug, Formatter},
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use anyhow::{Context, Error};
use hound::WavReader;

use super::{Capability, ParameterError};

#[derive(Clone)]
pub struct Sound {
    samples: Vec<i16>,
    next_index: usize,
}

impl Sound {
    pub fn new(samples: Vec<i16>) -> Self {
        Sound {
            samples,
            next_index: 0,
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

    fn samples(&mut self) -> impl Iterator<Item = i16> + '_ { Samples(self) }
}

impl Capability for Sound {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let chunk_size = std::mem::size_of::<i16>();
        let mut bytes_written = 0;

        for (chunk, sample) in buffer.chunks_mut(chunk_size).zip(self.samples())
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
        _value: runic_types::Value,
    ) -> Result<(), ParameterError> {
        Err(ParameterError::unsupported(name))
    }
}

struct Samples<'a>(&'a mut Sound);

impl<'a> Iterator for Samples<'a> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let Samples(Sound {
            samples,
            next_index,
        }) = self;

        let sample = samples.get(*next_index)?;

        *next_index = (*next_index + 1) % samples.len();

        Some(*sample)
    }
}

impl Debug for Sound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Sound {
            samples,
            next_index,
        } = self;

        f.debug_struct("Sound")
            .field("samples", &format_args!("({} samples)", samples.len()))
            .field("next_index", next_index)
            .finish()
    }
}
