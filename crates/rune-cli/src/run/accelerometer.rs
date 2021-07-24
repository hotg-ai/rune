use anyhow::{Context, Error};
use rune_core::{Value};
use std::{
    fmt::{self, Formatter, Debug},
    path::Path,
};
use rune_runtime::ParameterError;

use crate::run::multi::{Builder, SourceBackedCapability};

type Sample = [f32; 3];

#[derive(Clone, PartialEq)]
pub struct Accelerometer {
    samples: Vec<Sample>,
}

impl SourceBackedCapability for Accelerometer {
    type Builder = AccelerometerSetting;
    type Source = Samples;

    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if self.samples.is_empty() {
            return Ok(0);
        }

        let chunk_size = std::mem::size_of::<Sample>();
        let mut bytes_written = 0;

        for (chunk, sample) in buffer.chunks_mut(chunk_size).zip(&self.samples)
        {
            let bytes = as_byte_array(sample);
            chunk.copy_from_slice(bytes);

            bytes_written += chunk.len();
        }

        Ok(bytes_written)
    }

    fn from_builder(
        builder: AccelerometerSetting,
        source: &Samples,
    ) -> Result<Self, Error> {
        let AccelerometerSetting { sample_count } = builder;
        let sample_count = sample_count.unwrap_or(source.samples.len());

        anyhow::ensure!(
            sample_count <= source.samples.len(),
            "{} samples were requested but only {} are available",
            sample_count,
            source.samples.len()
        );

        Ok(Accelerometer {
            samples: source.samples[..sample_count].to_vec(),
        })
    }
}

fn as_byte_array(floats: &[f32]) -> &[u8] {
    // Safety: It's always valid to reinterpret a float array as bytes.
    unsafe {
        std::slice::from_raw_parts(
            floats.as_ptr().cast(),
            floats.len() * std::mem::size_of::<f32>(),
        )
    }
}

impl Debug for Accelerometer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Accelerometer { samples } = self;

        f.debug_struct("Accelerometer")
            .field("samples", &format_args!("({} samples)", samples.len()))
            .finish()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct AccelerometerSetting {
    sample_count: Option<usize>,
}

impl Builder for AccelerometerSetting {
    fn set_parameter(
        &mut self,
        key: &str,
        value: Value,
    ) -> Result<(), ParameterError> {
        let AccelerometerSetting { sample_count } = self;

        match key {
            "n" | "samples" => super::try_from_int_value(sample_count, value),
            _ => Err(ParameterError::UnsupportedParameter),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Samples {
    samples: Vec<Sample>,
}

impl Samples {
    pub fn new(samples: impl Into<Vec<Sample>>) -> Self {
        let samples = samples.into();

        Samples { samples }
    }

    pub fn from_csv_file(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let filename = filename.as_ref();
        let contents =
            std::fs::read_to_string(filename).with_context(|| {
                format!("Unable to read \"{}\"", filename.display())
            })?;

        Samples::from_csv(&contents)
    }

    pub fn from_csv(csv: &str) -> Result<Self, Error> {
        let mut samples = Vec::new();

        for line in csv.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let mut words = line.split(",").map(|s| s.trim());

            match (words.next(), words.next(), words.next()) {
                (Some(a), Some(b), Some(c)) => {
                    samples.push([a.parse()?, b.parse()?, c.parse()?])
                },
                (None, None, None) => {},
                _ => anyhow::bail!(
                    "Expected a row with 3 columns but found {:?}",
                    line
                ),
            }
            anyhow::ensure!(
                words.next().is_none(),
                "There were more than 3 columns in {:?}",
                line
            );
        }

        Ok(Samples::new(samples))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn read_some_samples() {
        let samples = Samples::new(vec![
            [0.0, 1.0, 2.0],
            [1.0, 2.0, 3.0],
            [2.0, 3.0, 4.0],
        ]);
        let settings = AccelerometerSetting::default();
        let mut accel =
            Accelerometer::from_builder(settings, &samples).unwrap();
        let mut buffer = [0; std::mem::size_of::<Sample>() * 3];

        let bytes_written = accel.generate(&mut buffer).unwrap();

        assert_eq!(bytes_written, buffer.len());
        let should_be: Vec<u8> = accel
            .samples
            .iter()
            .flat_map(|sample| sample.iter())
            .flat_map(|float| {
                Vec::from_iter(float.to_ne_bytes().iter().copied())
            })
            .collect();
        assert_eq!(&buffer[..], should_be);
    }

    #[test]
    fn the_buffer_can_be_too_big() {
        let samples = Samples::new(vec![[0.0, 1.0, 2.0]]);
        let mut accel = Accelerometer::from_builder(
            AccelerometerSetting::default(),
            &samples,
        )
        .unwrap();
        let mut buffer = [0; std::mem::size_of::<Sample>() * 2];

        let bytes_written = accel.generate(&mut buffer).unwrap();

        assert_eq!(bytes_written, std::mem::size_of::<Sample>());
    }
}
