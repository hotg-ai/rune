extern crate sonogram;
use sonogram::SpecOptionsBuilder;

pub use runic_types::{Transform, PipelineContext};

pub struct FFT {}

impl runic_types::Transform<Vec<i16>> for FFT {
    // N = 1960
    type Output = Vec<u8>;

    fn transform(&mut self,
        input: Vec<i16>,
        _ctx: &mut PipelineContext) -> Self::Output {

        // Build the spectrogram computation engine
        let mut spectrograph = SpecOptionsBuilder::new(49, 40)
        .load_data_from_memory(input, 16000)
        //.unwrap()
        .build();
    
        // Compute the spectrogram giving the number of bins and the window overlap.
        spectrograph.compute(256, 0.66667);
    
        let result_f32 = spectrograph.create_in_memory(false);

        let result_f32_slice: &[f32] = result_f32.as_slice();
        let mut spectrogram_u8: [u8; 1960]= [0; 1960];
    
        let min_value = result_f32_slice.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_value = result_f32_slice.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    
        for i in 0..spectrogram_u8.len() {
            spectrogram_u8[i] = (255.0*(result_f32_slice[i]-min_value)/(max_value-min_value)) as u8;
        }

        return spectrogram_u8.to_vec();
        
      }
    
}


#[test]
fn test_processing_block(){

    let waveform: Vec<i16> = vec![0; 16000];

    let fft = FFT{};
    let mut pipeline = PipelineContext{};
    let result = fft.transform(waveform, & mut pipeline);
    assert_eq!(result.len(), 1960);
    
}