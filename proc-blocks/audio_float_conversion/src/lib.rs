#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use hotg_rune_proc_blocks::{ProcBlock, Transform, Tensor};

// TODO: Add Generics

#[derive(Debug, Clone, PartialEq, ProcBlock)]
#[transform(inputs = [i16; _], outputs = [f32; _])]
pub struct AudioFloatConversion {
    i16_max_as_float: f32,
}

const I16_MAX_AS_FLOAT: f32 = i16::MAX as f32;

impl AudioFloatConversion {
    pub const fn new() -> Self {
        AudioFloatConversion {
            i16_max_as_float: I16_MAX_AS_FLOAT,
        }
    }

    fn check_input_dimensions(&self, dimensions: &[usize]) {
        assert_eq!(
            dimensions.len(),
            1,
            "This proc block only supports 1D outputs (requested output: {:?})",
            dimensions
        );
    }
}

impl Default for AudioFloatConversion {
    fn default() -> Self { AudioFloatConversion::new() }
}

impl Transform<Tensor<i16>> for AudioFloatConversion {
    type Output = Tensor<f32>;

    fn transform(&mut self, input: Tensor<i16>) -> Self::Output {
        self.check_input_dimensions(input.dimensions());
        input.map(|_dims, &value| {
            (value as f32 / I16_MAX_AS_FLOAT).clamp(-1.0, 1.0)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_empty() {
        let mut pb = AudioFloatConversion::new();
        let input = Tensor::new_vector(vec![0; 15]);

        let got = pb.transform(input);

        assert_eq!(got.dimensions(), &[15]);
    }

    #[test]
    fn does_it_match() {
        let max = i16::MAX;
        let min = i16::MIN;

        let mut pb = AudioFloatConversion::new();
        let input = Tensor::new_vector(vec![0, max / 2, min / 2]);

        let got = pb.transform(input);

        assert_eq!(got.elements()[0..3], [0.0, 0.49998474, -0.50001526]);
    }
    #[test]
    fn does_clutch_work() {
        let max = i16::MAX;
        let min = i16::MIN;

        let mut pb = AudioFloatConversion::new();
        let input = Tensor::new_vector(vec![max, min, min + 1]);

        let got = pb.transform(input);

        assert_eq!(got.elements()[0..3], [1.0, -1.0, -1.0]);
    }
}
