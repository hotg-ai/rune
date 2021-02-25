#![no_std]

extern crate sonogram;
use sonogram::SpecOptionsBuilder;

pub use runic_types::{PipelineContext, Transform};

pub struct FFT {}

impl<const N: usize> runic_types::Transform<[i16; N]> for FFT {
    type Output = [u8; 1960];

    fn transform(
        &mut self,
        input: [i16; N],
        _ctx: &mut PipelineContext,
    ) -> Self::Output {
        // Build the spectrogram computation engine
        let mut spectrograph = SpecOptionsBuilder::new(49, 40)
        .load_data_from_memory(input.to_vec(), 16000)
        //.unwrap()
        .build();

        // Compute the spectrogram giving the number of bins and the window
        // overlap.
        spectrograph.compute(256, 0.66667);

        let result_f32 = spectrograph.create_in_memory(false);

        let result_f32_slice: &[f32] = result_f32.as_slice();
        let mut spectrogram_u8: [u8; 1960] = [0; 1960];

        let min_value = result_f32_slice
            .iter()
            .fold(f32::INFINITY, |a, &b| a.min(b));
        let max_value = result_f32_slice
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b));

        for i in 0..spectrogram_u8.len() {
            spectrogram_u8[i] = (255.0 * (result_f32_slice[i] - min_value)
                / (max_value - min_value))
                as u8;
        }

        return spectrogram_u8;
    }
}

#[test]
fn test_processing_block() {
    let waveform: [i16; 16000] = [0; 16000];

    let mut fft = FFT {};
    let mut pipeline = PipelineContext {};
    let result = fft.transform(waveform, &mut pipeline);
    assert_eq!(result.len(), 1960);
}
