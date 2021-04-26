use std::{
    collections::VecDeque,
    fs::File,
    sync::{Arc, RwLock},
};

use anyhow::{Error, Context};
use cpal::{
    BufferSize, SampleRate, Stream, StreamConfig,
    traits::{DeviceTrait, HostTrait},
};
use hound::{SampleFormat, WavSpec};

pub fn start_recording() -> Result<(Stream, Arc<RwLock<Samples>>), Error> {
    let host = cpal::default_host();

    let microphone = host
        .default_input_device()
        .context("Unable to connected to your microphone")?;

    let samples = Arc::new(RwLock::new(Samples::new(1000)));
    let samples_2 = Arc::clone(&samples);

    let stream_config = StreamConfig {
        channels: 1,
        // TODO: Figure out how to get the sample rate out of the rune (e.g.
        // when setting the "hz" property on our sound capability)
        sample_rate: SampleRate(16_000),
        buffer_size: BufferSize::Default,
    };
    log::debug!("Building the input stream with {:?}", stream_config);

    // TODO: Remove this WAV writer once testing is over
    let filename = "samples.wav";
    let f = File::create(filename).with_context(|| {
        format!("Unable to open \"{}\" for writing", filename)
    })?;
    let spec = WavSpec {
        channels: 1,
        sample_rate: stream_config.sample_rate.0 * 2,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut wav_writer = hound::WavWriter::new(f, spec)?;

    let stream = microphone
        .build_input_stream(
            &stream_config,
            move |data: &[i16], _| {
                let mut samples = samples_2.write().unwrap();
                for sample in data {
                    wav_writer.write_sample(*sample).unwrap();
                }
                wav_writer.flush().unwrap();
                samples.append(data);
            },
            |err| panic!("Error: {}", err),
        )
        .context("Unable to establish the input stream")?;

    Ok((stream, samples))
}

/// A circular buffer containing the last N audio samples.
#[derive(Debug)]
pub struct Samples {
    buffer: VecDeque<i16>,
    max_samples: usize,
}

impl Samples {
    pub fn new(max_samples: usize) -> Self {
        Samples {
            buffer: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn append(&mut self, samples: &[i16]) {
        self.buffer.extend(samples.iter().copied());
        self.trim();
    }

    pub fn len(&self) -> usize { self.buffer.len() }

    pub fn iter(&self) -> impl Iterator<Item = i16> + '_ {
        self.buffer.iter().copied()
    }

    pub fn set_capacity(&mut self, capacity: usize) {
        self.max_samples = capacity;
        self.trim();
    }

    fn trim(&mut self) {
        if self.buffer.len() <= self.max_samples {
            return;
        }

        let samples_to_remove = self.len() - self.max_samples;
        let _ = self.buffer.drain(..samples_to_remove);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_samples_and_stay_within_capacity() {
        let mut samples = Samples::new(4);
        assert_eq!(samples.len(), 0);

        samples.append(&[1, 2, 3]);
        assert_eq!(samples.len(), 3);

        samples.append(&[4, 5, 6]);
        assert_eq!(samples.len(), 4);

        assert_eq!(samples.buffer, &[3, 4, 5, 6]);
    }
}
