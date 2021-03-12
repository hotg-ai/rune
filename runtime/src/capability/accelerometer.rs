use anyhow::Error;
use runic_types::{Value};

use super::{Capability, ParameterError};

type Sample = [f32; 3];

#[derive(Debug, Clone, PartialEq)]
pub struct Accelerometer {
    samples: Vec<Sample>,
    next_sample: usize,
    infinite: bool,
}

impl Accelerometer {
    pub fn new(samples: impl Into<Vec<Sample>>) -> Self {
        Accelerometer {
            samples: samples.into(),
            next_sample: 0,
            infinite: true,
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

    fn samples(&mut self) -> impl Iterator<Item = Sample> + '_ { Samples(self) }
}

impl Capability for Accelerometer {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if self.samples.is_empty() {
            return Ok(0);
        }

        let chunk_size = std::mem::size_of::<Sample>();
        let mut bytes_written = 0;

        for (chunk, sample) in buffer.chunks_mut(chunk_size).zip(self.samples())
        {
            let bytes = as_byte_array(&sample);
            chunk.copy_from_slice(bytes);

            bytes_written += chunk.len();
        }

        Ok(bytes_written)
    }

    fn set_parameter(
        &mut self,
        name: &str,
        _value: Value,
    ) -> Result<(), ParameterError> {
        Err(ParameterError::unsupported(name))
    }
}

#[derive(Debug)]
struct Samples<'a>(&'a mut Accelerometer);

impl<'a> Iterator for Samples<'a> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let Samples(Accelerometer {
            infinite,
            next_sample,
            samples,
        }) = self;

        let sample = samples.get(*next_sample).copied()?;

        *next_sample += 1;
        if *infinite {
            *next_sample %= samples.len();
        }

        dbg!(&*self);

        Some(sample)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_all_the_samples() {
        let mut accel = Accelerometer::new(vec![
            [0.0, 1.0, 2.0],
            [1.0, 2.0, 3.0],
            [2.0, 3.0, 4.0],
        ]);
        let mut buffer = [0; std::mem::size_of::<Sample>()];

        let first = accel.generate(&mut buffer).unwrap();
        assert_eq!(first, buffer.len());
        assert_eq!(&buffer[..], as_byte_array(&accel.samples[0]));
        assert_eq!(accel.next_sample, 1);

        let second = accel.generate(&mut buffer).unwrap();
        assert_eq!(second, buffer.len());
        assert_eq!(&buffer[..], as_byte_array(&accel.samples[1]));
        assert_eq!(accel.next_sample, 2);

        let third = accel.generate(&mut buffer).unwrap();
        assert_eq!(third, buffer.len());
        assert_eq!(&buffer[..], as_byte_array(&accel.samples[2]));
        // Note: It should wrap around by default
        assert_eq!(accel.next_sample, 0);
    }

    #[test]
    fn finite_samples() {
        let mut accel = Accelerometer::new(vec![[0.0, 1.0, 2.0]]);
        accel.infinite = false;
        let mut buffer = [0; std::mem::size_of::<Sample>()];

        // the first call consumes all data
        let _ = accel.generate(&mut buffer).unwrap();
        // successive calls shouldn't yield anything
        let bytes_written = accel.generate(&mut buffer).unwrap();

        assert_eq!(bytes_written, 0);
    }
}
