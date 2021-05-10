use std::{
    collections::VecDeque,
    fmt::Debug,
    sync::{Arc, RwLock},
};
use anyhow::{Context, Error};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    BufferSize, SampleRate, Stream, StreamConfig,
};
use dasp::{Signal, interpolate::linear::Linear};

#[derive(Debug, Clone, PartialEq)]
pub struct AudioSystem {
    pub samples: Samples,
    pub sample_rate: i32,
}

impl AudioSystem {
    pub fn update_buffer_capacity(&mut self, sample_duration_ms: i32) {
        let capacity = self.sample_rate * sample_duration_ms / 1000;
        self.samples.set_capacity(capacity as usize);
    }
}

pub fn start_recording() -> Result<(Stream, Arc<RwLock<AudioSystem>>), Error> {
    log::info!("Started recording");
    let host = cpal::default_host();

    let microphone = host
        .default_input_device()
        .context("Unable to connected to your microphone")?;

    let audio = AudioSystem {
        samples: Samples::new(1000),
        sample_rate: 16_000,
    };
    let samples = Arc::new(RwLock::new(audio));
    let samples_2 = Arc::clone(&samples);

    let stream_config = StreamConfig {
        channels: 1,
        // TODO: Figure out how to get the sample rate out of the rune (e.g.
        // when setting the "hz" property on our sound capability)
        sample_rate: SampleRate(44_100),
        buffer_size: BufferSize::Default,
    };
    log::debug!("Building the input stream with {:?}", stream_config);

    let source_hz = stream_config.sample_rate.0 as f64;

    let stream = microphone
        .build_input_stream(
            &stream_config,
            move |data: &[f32], _| {
                let mut audio = samples_2.write().unwrap();

                let mut signal = dasp::signal::from_iter(data.iter().cloned());

                let first_sample = signal.next();
                let second_sample = signal.next();
                let interpolator = Linear::new(first_sample, second_sample);

                let dest_hz = 16000.0;

                let converter = signal.from_hz_to_hz(
                    interpolator,
                    source_hz,
                    audio.sample_rate as f64,
                );

                let sample_count =
                    (data.len() as f64 * dest_hz / source_hz).round() as usize;
                audio.samples.extend(converter.take(sample_count));
            },
            |err| panic!("Error: {}", err),
        )
        .context("Unable to establish the input stream")?;

    Ok((stream, samples))
}

/// A circular buffer containing the last N audio samples.
#[derive(Debug, Clone, PartialEq)]
pub struct Samples {
    buffer: VecDeque<f32>,
    max_samples: usize,
}

impl Samples {
    pub fn new(max_samples: usize) -> Self {
        Samples {
            buffer: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn append(&mut self, samples: &[f32]) {
        self.buffer.extend(samples.iter().copied());
        self.trim();
    }

    pub fn len(&self) -> usize { self.buffer.len() }

    pub fn iter(&self) -> impl Iterator<Item = f32> + '_ {
        self.buffer.iter().copied()
    }

    pub fn set_capacity(&mut self, capacity: usize) {
        self.max_samples = capacity;
        self.trim();
    }

    fn trim(&mut self) {
        if self.buffer.len() <= self.max_samples {
            let samples_required = self.max_samples - self.buffer.len();
            self.buffer
                .extend(std::iter::repeat(0.0).take(samples_required));
            return;
        }

        let samples_to_remove = self.len() - self.max_samples;
        let _ = self.buffer.drain(..samples_to_remove);
    }
}

impl Extend<f32> for Samples {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = f32>,
    {
        self.buffer.extend(iter);
        self.trim();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_samples_and_stay_within_capacity() {
        let mut samples = Samples::new(4);
        assert_eq!(samples.len(), 0);

        samples.append(&[1.0, 2.0, 3.0]);
        assert_eq!(samples.len(), 3);

        samples.append(&[4.0, 5.0, 6.0]);
        assert_eq!(samples.len(), 4);

        assert_eq!(samples.buffer, &[3.0, 4.0, 5.0, 6.0]);
    }
}
