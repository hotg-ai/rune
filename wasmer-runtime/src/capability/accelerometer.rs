use anyhow::Error;
use runic_types::{Value};
use std::fmt::{self, Formatter, Debug};
use super::{Capability, ParameterError};

type Sample = [f32; 3];

#[derive(Clone, PartialEq)]
pub struct Accelerometer {
    samples: Vec<Sample>,
    desired_sample_count: usize,
}

impl Accelerometer {
    pub fn new(samples: impl Into<Vec<Sample>>) -> Self {
        let samples = samples.into();
        let desired_sample_count = samples.len();

        Accelerometer {
            samples,
            desired_sample_count,
        }
    }

    pub fn from_csv(csv: &str) -> Result<Self, Error> {
        let mut samples = Vec::new();

        for line in csv.lines() {
            let mut words = line.split(",").map(|s| s.trim());

            match (words.next(), words.next(), words.next()) {
                (Some(a), Some(b), Some(c)) => {
                    samples.push([a.parse()?, b.parse()?, c.parse()?])
                },
                (None, None, None) => {},
                _ => anyhow::bail!("Expected 3 columns"),
            }
            anyhow::ensure!(words.next().is_none(), "Expected 3 columns");
        }

        Ok(Accelerometer::new(samples))
    }
}

impl Capability for Accelerometer {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if self.samples.is_empty() {
            return Ok(0);
        }

        let chunk_size = std::mem::size_of::<Sample>();
        let mut bytes_written = 0;

        let samples = &self.samples[..self.desired_sample_count];

        for (chunk, sample) in buffer.chunks_mut(chunk_size).zip(samples) {
            let bytes = as_byte_array(sample);
            chunk.copy_from_slice(bytes);

            bytes_written += chunk.len();
        }

        Ok(bytes_written)
    }

    fn set_parameter(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), ParameterError> {
        match name {
            "n" | "samples" => {
                let desired_sample_count: usize =
                    super::try_from_int_value(value)?;
                if desired_sample_count > self.samples.len() {
                    let reason = anyhow::anyhow!(
                        "{} samples were requested but only {} are available",
                        desired_sample_count,
                        self.samples.len()
                    );
                    return Err(ParameterError::InvalidValue { value, reason });
                }

                self.desired_sample_count = desired_sample_count;

                Ok(())
            },
            _ => Err(ParameterError::UnsupportedParameter),
        }
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
        let Accelerometer {
            samples,
            desired_sample_count,
        } = self;

        f.debug_struct("Accelerometer")
            .field("samples", &format_args!("({} samples)", samples.len()))
            .field("desired_sample_count", desired_sample_count)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn read_some_samples() {
        let mut accel = Accelerometer::new(vec![
            [0.0, 1.0, 2.0],
            [1.0, 2.0, 3.0],
            [2.0, 3.0, 4.0],
        ]);
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
        let mut accel = Accelerometer::new(vec![[0.0, 1.0, 2.0]]);
        let mut buffer = [0; std::mem::size_of::<Sample>() * 2];

        let bytes_written = accel.generate(&mut buffer).unwrap();

        assert_eq!(bytes_written, std::mem::size_of::<Sample>());
    }
}
