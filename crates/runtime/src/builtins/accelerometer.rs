use std::{
    path::{Path, PathBuf},
    fs::File,
    io::Read,
    num::ParseFloatError,
    ops::Deref,
    str::FromStr,
};

use anyhow::Error;
use csv::{StringRecord, Position};

use crate::{builtins::Arguments, Tensor};

/// Load an input tensor from a set of accelerometer samples.
pub fn accelerometer(
    args: &Arguments,
    samples: &AccelerometerSamples,
) -> Result<Tensor, Error> {
    let requested_samples: usize =
        args.parse_or_default("samples", samples.len())?;

    if requested_samples > samples.len() {
        anyhow::bail!(
            "{} samples were requested but only {} are available",
            requested_samples,
            samples.len(),
        );
    }

    let mut buffer = Vec::with_capacity(samples.len() * 3);

    for sample in &samples[..requested_samples] {
        let AccelerometerSample { x, y, z } = *sample;
        buffer.push(x);
        buffer.push(y);
        buffer.push(z);
    }

    Ok(Tensor::new(&buffer, &[samples.len(), 3]))
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AccelerometerSample {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccelerometerSamples(pub Vec<AccelerometerSample>);

impl AccelerometerSamples {
    pub fn from_file(
        path: impl AsRef<Path>,
    ) -> Result<Self, AccelerometerParseError> {
        let path = path.as_ref();
        let f = File::open(path).map_err(|reason| {
            AccelerometerParseError::OpenFile {
                filename: path.to_path_buf(),
                reason,
            }
        })?;

        AccelerometerSamples::from_reader(f)
    }

    pub fn from_reader(
        reader: impl Read,
    ) -> Result<AccelerometerSamples, AccelerometerParseError> {
        let mut samples = Vec::new();

        let mut reader = csv::Reader::from_reader(reader);
        let mut record = StringRecord::new();

        while reader.read_record(&mut record)? {
            let pos = reader.position();

            if record.len() != 3 {
                return Err(AccelerometerParseError::IncorrectNumberOfFields {
                    actual: record.len(),
                    expected: 3,
                    line: pos.line(),
                });
            }

            let sample = parse_sample(&record, pos)?;
            samples.push(sample);
        }

        Ok(AccelerometerSamples(samples))
    }
}

impl FromStr for AccelerometerSamples {
    type Err = AccelerometerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reader = std::io::Cursor::new(s.as_bytes());
        AccelerometerSamples::from_reader(reader)
    }
}

impl Deref for AccelerometerSamples {
    type Target = [AccelerometerSample];

    fn deref(&self) -> &Self::Target { &self.0 }
}

fn parse_sample(
    record: &StringRecord,
    pos: &Position,
) -> Result<AccelerometerSample, AccelerometerParseError> {
    let line = pos.line();

    let x = parse_field(&record[0], line)?;
    let y = parse_field(&record[1], line)?;
    let z = parse_field(&record[2], line)?;

    Ok(AccelerometerSample { x, y, z })
}

fn parse_field(value: &str, line: u64) -> Result<f64, AccelerometerParseError> {
    value
        .parse()
        .map_err(|reason| AccelerometerParseError::InvalidSample {
            line,
            value: value.to_string(),
            reason,
        })
}

#[derive(Debug, thiserror::Error)]
pub enum AccelerometerParseError {
    #[error("Unable to parse \"{}\" on line {}", value, line)]
    InvalidSample {
        line: u64,
        value: String,
        #[source]
        reason: ParseFloatError,
    },
    #[error(
        "Line {} should have {} fields but it actually had {}",
        line,
        expected,
        actual
    )]
    IncorrectNumberOfFields {
        expected: usize,
        actual: usize,
        line: u64,
    },
    #[error("Unable to open \"{}\"", filename.display())]
    OpenFile {
        filename: PathBuf,
        #[source]
        reason: std::io::Error,
    },
    #[error(transparent)]
    Csv(#[from] csv::Error),
}
