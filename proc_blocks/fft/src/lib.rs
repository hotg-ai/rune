#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod gain_control;
mod noise_reduction;
mod stft;

pub use crate::{
    noise_reduction::NoiseReduction, gain_control::GainControl,
    stft::ShortTimeFourierTransform,
};

use runic_types::{HasOutputs, Tensor, Transform};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Fft {
    stft: ShortTimeFourierTransform,
    noise_reduction: NoiseReduction,
    gain_control: GainControl,
}

impl Fft {
    pub fn with_strength(mut self, strength: f32) -> Self {
        self.gain_control = self.gain_control.with_strength(strength);
        self
    }

    pub fn with_offset(mut self, offset: f32) -> Self {
        self.gain_control = self.gain_control.with_offset(offset);
        self
    }

    pub fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.stft = self.stft.with_sample_rate(sample_rate);
        self
    }

    pub fn with_bins(mut self, bins: usize) -> Self {
        self.stft = self.stft.with_bins(bins);
        self
    }

    pub fn with_smoothing_bits(mut self, smoothing_bits: u32) -> Self {
        self.noise_reduction =
            self.noise_reduction.with_smoothing_bits(smoothing_bits);
        self
    }

    pub fn with_even_smoothing(mut self, even_smoothing: f32) -> Self {
        self.noise_reduction =
            self.noise_reduction.with_even_smoothing(even_smoothing);
        self
    }

    pub fn with_odd_smoothing(mut self, odd_smoothing: f32) -> Self {
        self.noise_reduction =
            self.noise_reduction.with_odd_smoothing(odd_smoothing);
        self
    }

    pub fn with_min_signal_remaining(
        mut self,
        min_signal_remaining: f32,
    ) -> Self {
        self.noise_reduction = self
            .noise_reduction
            .with_min_signal_remaining(min_signal_remaining);
        self
    }
}

impl Transform<Tensor<i16>> for Fft {
    type Output = Tensor<i16>;

    fn transform(&mut self, input: Tensor<i16>) -> Tensor<i16> {
        let spectrograph =
            self.stft.transform(input).map(|_, &energy| energy as u32);

        let cleaned = self.noise_reduction.transform(spectrograph);

        let amplified = self
            .gain_control
            .transform(cleaned, &self.noise_reduction.noise_estimate())
            .map(|_, &energy| energy as i16);

        amplified
    }
}

impl HasOutputs for Fft {
    fn set_output_dimensions(&mut self, _dimensions: &[usize]) {}
}
