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

pub fn start_recording() -> Result<(Stream, Arc<RwLock<Samples>>), Error> {
    println!("\n\n----Started recording----\n\n");

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
        sample_rate: SampleRate(44_100),
        buffer_size: BufferSize::Default,
    };
    log::debug!("Building the input stream with {:?}", stream_config);
    println!("Building the input stream with {:?}", stream_config);

    // TODO: Remove this WAV writer once testing is over

    // let filename = "samples.wav";
    // let f = File::create(filename).with_context(|| {
    //     format!("Unable to open \"{}\" for writing", filename)
    // })?;
    // let spec = WavSpec {
    //     channels: 1,
    //     sample_rate: stream_config.sample_rate.0,
    //     bits_per_sample: 32,
    //     sample_format: SampleFormat::Float,
    // };
    // let mut wav_writer = hound::WavWriter::new(f, spec)?;

    let stream = microphone
        .build_input_stream(
            &stream_config,
            move |data: &[f32], _| {
                let mut samples = samples_2.write().unwrap();

                let mut data_1 = signal::from_iter(data.iter().cloned());

                let vec_1: Vec<_> = data_1.from_hz_to_hz(
                    dasp_interpolate::linear::Linear,
                    stream_config.sample_rate.0 as f64,
                    16000.0,
                );

                // samples.append(data);
                samples.append(vec_1[vec_1.len() - 1]);
            },
            |err| panic!("Error: {}", err),
        )
        .context("Unable to establish the input stream")?;

    Ok((stream, samples))
}

/// A circular buffer containing the last N audio samples.
#[derive(Debug)]
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

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = f32> + '_ {
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

        samples.append(&[1.0, 2.0, 3.0]);
        assert_eq!(samples.len(), 3);

        samples.append(&[4.0, 5.0, 6.0]);
        assert_eq!(samples.len(), 4);

        assert_eq!(samples.buffer, &[3.0, 4.0, 5.0, 6.0]);
    }
}
