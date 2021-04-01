#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use sonogram::SpecOptionsBuilder;

pub use runic_types::{Transform};

#[derive(Clone, PartialEq)]
pub struct Fft {
    pub sample_rate: u32,
    pub bins: usize,
    pub window_overlap: f32,
}

const DEFAULT_SAMPLE_RATE: u32 = 16000;
const DEFAULT_BINS: usize = 256;
const DEFAULT_WINDOW_OVERLAP: f32 = 6.0 / 10.0;

impl Fft {
    pub const fn new() -> Self {
        Fft {
            sample_rate: DEFAULT_SAMPLE_RATE,
            bins: DEFAULT_BINS,
            window_overlap: DEFAULT_WINDOW_OVERLAP,
        }
    }

    pub fn default() -> Self { Fft::new() }

    // `Self` is the type and `self` is the pointer
    pub fn with_sample_rate(self, sample_rate: u32) -> Self {
        Fft {
            sample_rate,
            ..self
        }
    }

    pub fn with_bins(self, bins: usize) -> Self { Fft { bins, ..self } }

    pub fn with_window_overlap(self, window_overlap: f32) -> Self {
        Fft {
            window_overlap,
            ..self
        }
    }

    fn transform_inner(&mut self, input: Vec<i16>) -> [u8; 1960] {
        // Build the spectrogram computation engine
        let mut spectrograph = SpecOptionsBuilder::new(49, 40)
        .load_data_from_memory(input, self.sample_rate)
        //.unwrap()
        .build();

        // // Compute the spectrogram giving the number of bins and the window
        // // overlap.
        spectrograph.compute(self.bins, self.window_overlap);

        let result_f32 = spectrograph.create_in_memory(false);

        let min_value = result_f32.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_value =
            result_f32.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

        let res: Vec<u8> = result_f32
            .into_iter()
            .map(|freq| 255.0 * (freq - min_value) / (max_value - min_value))
            .map(|freq| freq as u8)
            .collect();
        let mut out = [0; 1960];

        for i in 0..1960 {
            out[i] = res[i];
        }

        return out;
    }
}

impl Default for Fft {
    fn default() -> Self { Fft::new() }
}

impl<const N: usize> runic_types::Transform<[i16; N]> for Fft {
    type Output = [u8; 1960];

    fn transform(&mut self, input: [i16; N]) -> Self::Output {
        self.transform_inner(input.to_vec())
    }
}

impl<'a> runic_types::Transform<&'a [i16]> for Fft {
    type Output = [u8; 1960];

    fn transform(&mut self, input: &'a [i16]) -> Self::Output {
        self.transform_inner(input.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut fft_pb = Fft::new().with_sample_rate(16000);
        let input = [0; 16000];

        let res = fft_pb.transform(input);
        assert_eq!(res.len(), 1960)
    }
}
