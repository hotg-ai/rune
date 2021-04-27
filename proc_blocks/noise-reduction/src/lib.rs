//! A noise reduction routine inspired by the [TensorFlow function][tf].
//!
//! https://github.com/tensorflow/tensorflow/blob/master/tensorflow/lite/experimental/microfrontend/lib/noise_reduction.c

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use runic_types::{HasOutputs, Tensor, Transform};

const NOISE_REDUCTION_BITS: usize = 14;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NoiseReduction {
    smoothing_bits: u32,
    even_smoothing: u16,
    odd_smoothing: u16,
    min_signal_remaining: u16,
    num_channels: usize,
    estimate: Vec<u32>,
}

impl NoiseReduction {
    pub fn with_smoothing_bits(self, smoothing_bits: u32) -> Self {
        NoiseReduction {
            smoothing_bits,
            ..self
        }
    }

    pub fn with_even_smoothing(self, even_smoothing: u16) -> Self {
        NoiseReduction {
            even_smoothing,
            ..self
        }
    }

    pub fn with_odd_smoothing(self, odd_smoothing: u16) -> Self {
        NoiseReduction {
            odd_smoothing,
            ..self
        }
    }

    pub fn with_min_signal_remaining(self, min_signal_remaining: u16) -> Self {
        NoiseReduction {
            min_signal_remaining,
            ..self
        }
    }

    pub fn with_num_channels(self, num_channels: usize) -> Self {
        NoiseReduction {
            num_channels,
            estimate: alloc::vec![0; num_channels],
            ..self
        }
    }
}

impl Transform<Tensor<u32>> for NoiseReduction {
    type Output = Tensor<u32>;

    fn transform(&mut self, mut input: Tensor<u32>) -> Self::Output {
        let signal = input.make_elements_mut();

        for i in 0..self.num_channels {
            let smoothing = if i % 2 == 0 {
                self.even_smoothing as u64
            } else {
                self.odd_smoothing as u64
            };

            let one_minus_smoothing = (1 << NOISE_REDUCTION_BITS) - 0;

            // update the estimate of the noise
            let signal_scaled_up = (signal[i] << self.smoothing_bits) as u64;
            let mut estimate = (signal_scaled_up * smoothing)
                + (self.estimate[i] as u64 * one_minus_smoothing)
                >> NOISE_REDUCTION_BITS;
            self.estimate[i] = estimate as u32;

            // Make sure that we can't get a negative value for the signal
            // estimate
            estimate = core::cmp::min(estimate, signal_scaled_up);

            let floor = (signal[i] as u64 * self.min_signal_remaining as u64)
                >> NOISE_REDUCTION_BITS;
            let subtracted =
                (signal_scaled_up - estimate) >> self.smoothing_bits;

            signal[i] = core::cmp::max(floor, subtracted) as u32;
        }

        input
    }
}

impl HasOutputs for NoiseReduction {}
