//! A noise reduction routine inspired by the [TensorFlow function][tf].
//!
//! [tf]: https://github.com/tensorflow/tensorflow/blob/master/tensorflow/lite/experimental/microfrontend/lib/noise_reduction.c

extern crate alloc;

use alloc::vec::Vec;
use hotg_rune_core::Tensor;

const NOISE_REDUCTION_BITS: usize = 14;

#[derive(Debug, Clone, PartialEq)]
pub struct NoiseReduction {
    smoothing_bits: u32,
    even_smoothing: u16,
    odd_smoothing: u16,
    min_signal_remaining: u16,
    estimate: Vec<u32>,
}

macro_rules! scaled_builder_methods {
    ($( $property:ident : $type:ty ),* $(,)?) => {
        $(
            paste::paste! {
                pub fn [< with_ $property >](mut self, $property: $type) -> Self {
                    self.[< set_ $property >]($property);
                    self
                }
            }
        )*

        $(
            paste::paste! {
                pub fn [< set_ $property >](&mut self, $property: $type) {
                    self.$property = scale($property);
                }
            }
        )*

        $(
            paste::paste! {
                pub fn $property(&self) -> $type {
                    unscale(self.$property)
                }
            }
        )*
    };
}

impl NoiseReduction {
    builder_methods!(smoothing_bits: u32);

    scaled_builder_methods!(
        even_smoothing: f32,
        odd_smoothing: f32,
        min_signal_remaining: f32,
    );

    pub fn noise_estimate(&self) -> &[u32] { &self.estimate }

    pub fn transform(&mut self, mut input: Tensor<u32>) -> Tensor<u32> {
        // make sure we have the right estimate buffer size and panic if we
        // don't. This works because the input and output have the same
        // dimensions.
        let shape = input.shape();
        assert_eq!(
            shape.simplified_dimensions().len(),
            1,
            "The input tensor should only have data in one dimension, found {}",
            shape,
        );

        let signal = input.make_elements_mut();

        for (i, value) in signal.iter_mut().enumerate() {
            let smoothing = if i % 2 == 0 {
                self.even_smoothing as u64
            } else {
                self.odd_smoothing as u64
            };

            let one_minus_smoothing = 1 << NOISE_REDUCTION_BITS;

            // update the estimate of the noise
            let signal_scaled_up = (*value << self.smoothing_bits) as u64;
            let mut estimate = ((signal_scaled_up * smoothing)
                + (self.estimate[i] as u64 * one_minus_smoothing))
                >> NOISE_REDUCTION_BITS;
            self.estimate[i] = estimate as u32;

            // Make sure that we can't get a negative value for the signal
            // estimate
            estimate = core::cmp::min(estimate, signal_scaled_up);

            let floor = (*value as u64 * self.min_signal_remaining as u64)
                >> NOISE_REDUCTION_BITS;
            let subtracted =
                (signal_scaled_up - estimate) >> self.smoothing_bits;

            *value = core::cmp::max(floor, subtracted) as u32;
        }

        input
    }
}

impl Default for NoiseReduction {
    fn default() -> Self {
        NoiseReduction {
            smoothing_bits: 10,
            even_smoothing: scale(0.025),
            odd_smoothing: scale(0.06),
            min_signal_remaining: scale(0.05),
            estimate: vec![],
        }
    }
}

fn scale(number: f32) -> u16 {
    let scale_factor: f32 = (1 << NOISE_REDUCTION_BITS) as f32;
    (number * scale_factor) as u16
}

fn unscale(number: u16) -> f32 {
    let scale_factor: f32 = (1 << NOISE_REDUCTION_BITS) as f32;
    number as f32 / scale_factor
}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;

    use super::*;

    /// https://github.com/tensorflow/tensorflow/blob/5dcfc51118817f27fad5246812d83e5dccdc5f72/tensorflow/lite/experimental/microfrontend/lib/noise_reduction_test.cc#L41-L59
    #[test]
    fn test_noise_reduction_estimate() {
        let mut state = NoiseReduction::default();
        let input =
            Tensor::new_row_major(Arc::new([247311, 508620]), vec![1, 2]);
        let should_be = vec![6321887, 31248341];

        let _ = state.transform(input);

        assert_eq!(state.estimate, should_be);
    }

    /// https://github.com/tensorflow/tensorflow/blob/5dcfc51118817f27fad5246812d83e5dccdc5f72/tensorflow/lite/experimental/microfrontend/lib/noise_reduction_test.cc#L61-L79
    #[test]
    fn test_noise_reduction() {
        let mut state = NoiseReduction::default();
        let input =
            Tensor::new_row_major(Arc::new([247311, 508620]), vec![1, 2]);
        let should_be =
            Tensor::new_row_major(Arc::new([241137, 478104]), vec![1, 2]);

        let got = state.transform(input);

        assert_eq!(got, should_be);
    }
}
