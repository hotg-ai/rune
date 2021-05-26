#![no_std]

#[macro_use]
extern crate alloc;

#[macro_use]
mod macros;
mod gain_control;
mod noise_reduction;

pub use noise_reduction::NoiseReduction;
pub use gain_control::GainControl;

use runic_types::{HasOutputs, Tensor, Transform};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NoiseFiltering {
    gain_control: GainControl,
    noise_reduction: NoiseReduction,
}

impl NoiseFiltering {
    defered_builder_methods! {
        gain_control.strength: f32;
        gain_control.offset: f32;
        gain_control.gain_bits: i32;
        noise_reduction.smoothing_bits: u32;
        noise_reduction.even_smoothing: f32;
        noise_reduction.odd_smoothing: f32;
        noise_reduction.min_signal_remaining: f32;
    }
}

impl Transform<Tensor<u32>> for NoiseFiltering {
    type Output = Tensor<i8>;

    fn transform(&mut self, input: Tensor<u32>) -> Tensor<i8> {
        let cleaned = self.noise_reduction.transform(input);

        let amplified = self
            .gain_control
            .transform(cleaned, &self.noise_reduction.noise_estimate())
            .map(|_, energy| libm::log2((*energy as f64) + 1.0));

        let min_value = amplified
            .elements()
            .to_vec()
            .iter()
            .fold(f64::INFINITY, |a, &b| a.min(b));

        let max_value = amplified
            .elements()
            .to_vec()
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let scaled = amplified.map(|_, energy| {
            ((255.0 * (energy - min_value) / (max_value - min_value)) - 128.0)
                as i8
        });
        scaled
    }
}

impl HasOutputs for NoiseFiltering {
    fn set_output_dimensions(&mut self, _dimensions: &[usize]) {}
}
