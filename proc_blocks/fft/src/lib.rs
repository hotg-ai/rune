#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
mod macros;
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
    defered_builder_methods! {
        gain_control.strength: f32;
        gain_control.offset: f32;
        gain_control.gain_bits: i32;
        stft.sample_rate: u32;
        stft.bins: usize;
        noise_reduction.smoothing_bits: u32;
        noise_reduction.even_smoothing: f32;
        noise_reduction.odd_smoothing: f32;
        noise_reduction.min_signal_remaining: f32;
    }
}

impl Transform<Tensor<i16>> for Fft {
    type Output = Tensor<i8>;

    fn transform(&mut self, input: Tensor<i16>) -> Tensor<i8> {
        let spectrograph =
            self.stft.transform(input).map(|_, &energy| energy as u32);

        let cleaned = self.noise_reduction.transform(spectrograph);

        let amplified = self
            .gain_control
            .transform(cleaned, &self.noise_reduction.noise_estimate());

        let log = amplified.map(|_, &energy| libm::logf(energy as f32));

        // normalize::normalize(log.make_elements_mut());

        log.map(|_, &energy| libm::floorf(energy * 127.0) as i8)
    }
}

impl HasOutputs for Fft {
    fn set_output_dimensions(&mut self, _dimensions: &[usize]) {}
}
