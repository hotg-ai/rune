use alloc::vec::Vec;
use runic_types::{HasOutputs, Tensor, Transform};
use sonogram::SpecOptionsBuilder;
use mel;
use nalgebra::DMatrix;

#[derive(Debug, Clone, PartialEq)]
pub struct ShortTimeFourierTransform {
    pub sample_rate: u32,
    pub bins: usize,
    pub window_overlap: f32,
}

const DEFAULT_SAMPLE_RATE: u32 = 16000;
const DEFAULT_BINS: usize = 480;
const DEFAULT_WINDOW_OVERLAP: f32 = 0.6666666666666667;

impl ShortTimeFourierTransform {
    pub const fn new() -> Self {
        ShortTimeFourierTransform {
            sample_rate: DEFAULT_SAMPLE_RATE,
            bins: DEFAULT_BINS,
            window_overlap: DEFAULT_WINDOW_OVERLAP,
        }
    }

    pub fn default() -> Self { ShortTimeFourierTransform::new() }

    // `Self` is the type and `self` is the pointer
    pub fn with_sample_rate(self, sample_rate: u32) -> Self {
        ShortTimeFourierTransform {
            sample_rate,
            ..self
        }
    }

    pub fn with_bins(self, bins: usize) -> Self { ShortTimeFourierTransform { bins, ..self } }

    pub fn with_window_overlap(self, window_overlap: f32) -> Self {
        ShortTimeFourierTransform {
            window_overlap,
            ..self
        }
    }

    fn transform_inner(&mut self, input: Vec<i16>) -> [i8; 1960] {
        // Build the spectrogram computation engine
        let mut spectrograph = SpecOptionsBuilder::new(49, 241)
            .set_window_fn(sonogram::hann_function)
            .load_data_from_memory(input, self.sample_rate as u32)
            .build();

        // Compute the spectrogram giving the number of bins in a window and the
        // overlap between neighbour windows.
        spectrograph.compute(self.bins, self.window_overlap);

        let spectrogram = spectrograph.create_in_memory(false);

        let filter_count: usize = 40;
        let power_spectrum_size = 241;
        let window_size = 480;
        let sample_rate_usize: usize = 16000;

        // build up the mel filter matrix
        let mut mel_filter_matrix =
            DMatrix::<f64>::zeros(filter_count, power_spectrum_size);
        for (row, col, coefficient) in mel::enumerate_mel_scaling_matrix(
            sample_rate_usize,
            window_size,
            power_spectrum_size,
            filter_count,
        ) {
            mel_filter_matrix[(row, col)] = coefficient;
        }

        let spectrogram = spectrogram.into_iter().map(f64::from);
        let power_spectrum_matrix_unflipped: DMatrix<f64> =
            DMatrix::from_iterator(49, power_spectrum_size, spectrogram);
        let power_spectrum_matrix_transposed =
            power_spectrum_matrix_unflipped.transpose();
        let mut power_spectrum_vec: Vec<_> =
            power_spectrum_matrix_transposed.row_iter().collect();
        &power_spectrum_vec.reverse();
        let power_spectrum_matrix: DMatrix<f64> =
            DMatrix::from_rows(&power_spectrum_vec);
        let mel_spectrum_matrix = &mel_filter_matrix * &power_spectrum_matrix;

        let min_value = mel_spectrum_matrix
            .data
            .as_vec()
            .iter()
            .fold(f64::INFINITY, |a, &b| a.min(b));
        let max_value = mel_spectrum_matrix
            .data
            .as_vec()
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let res: Vec<i8> = mel_spectrum_matrix
            .data
            .as_vec()
            .iter()
            .map(|freq| {
                (255.0 * (freq - min_value) / (max_value - min_value)) - 128.0
            })
            .map(|freq| freq as i8)
            .collect();
        let mut out = [0; 1960];

        for i in 0..1960 {
            out[i] = res[i];
        }

        return out;
    }
}

impl Default for ShortTimeFourierTransform {
    fn default() -> Self { ShortTimeFourierTransform::new() }
}

impl<const N: usize> Transform<[i16; N]> for ShortTimeFourierTransform {
    type Output = [i8; 1960];

    fn transform(&mut self, input: [i16; N]) -> Self::Output {
        self.transform_inner(input.to_vec())
    }
}

impl Transform<Tensor<i16>> for ShortTimeFourierTransform {
    type Output = Tensor<i8>;

    fn transform(&mut self, input: Tensor<i16>) -> Self::Output {
        let input = input.elements().to_vec();
        let stft = self.transform_inner(input);
        Tensor::new_vector(stft.iter().copied())
    }
}

impl<'a> Transform<&'a [i16]> for ShortTimeFourierTransform {
    type Output = [i8; 1960];

    fn transform(&mut self, input: &'a [i16]) -> Self::Output {
        self.transform_inner(input.to_vec())
    }
}

impl HasOutputs for ShortTimeFourierTransform {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut fft_pb = ShortTimeFourierTransform::new().with_sample_rate(16000);
        let input = [0; 16000];

        let res = fft_pb.transform(input);
        assert_eq!(res.len(), 1960);
    }
}

