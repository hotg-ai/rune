#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use sonogram::SpecOptionsBuilder;

pub use runic_types::{Transform};

pub struct Fft {
    sample_rate: u32,
    bins: usize,
    window_overlap: f32,
}

const DEFAULT_SAMPLE_RATE: u32 = 16000;
const DEFAULT_BINS: usize = 256;
const DEFAULT_WINDOW_OVERLAP: f32 = 6.0 / 10.0;

impl Fft {
    pub fn new() -> Self {
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
}

impl<const N: usize> runic_types::Transform<[i16; N]> for Fft {
    type Output = Vec<u8>;

    fn transform(&mut self, input: [i16; N]) -> Self::Output {
        // Build the spectrogram computation engine
        let mut spectrograph = SpecOptionsBuilder::new(49, 40)
        .load_data_from_memory(input.to_vec(), self.sample_rate)
        //.unwrap()
        .build();

        // // Compute the spectrogram giving the number of bins and the window
        // // overlap.
        spectrograph.compute(self.bins, self.window_overlap);

        let result_f32 = spectrograph.create_in_memory(false);

        let min_value = result_f32.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_value =
            result_f32.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

        result_f32
            .into_iter()
            .map(|freq| 255.0 * (freq - min_value) / (max_value - min_value))
            .map(|freq| freq as u8)
            .collect()
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
