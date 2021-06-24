#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use alloc::vec::Vec;
use runic_types::{HasOutputs, Tensor};
use rune_pb_core::{ProcBlock, Transform};

// TODO: Add Generics

#[derive(Debug, Clone, PartialEq, ProcBlock)]
#[transform(input = [i16; _], output = [f32; _])]
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

    fn transform_inner(&mut self, input: Vec<i16>) -> [f32; 5] {
        let mut recorded_vec: [f32; 5] = [0.0; 5];

        // TODO: Need to fix i16::MIN being normalized to -1.0000305
        // TODO: [96*64] should be [96,64]

        for (i,i16_input) in input.iter().enumerate() {
            recorded_vec[i] = *i16_input as f32 / self.i16_max_as_float;
        }

        recorded_vec
    }

}

impl Default for AudioFloatConversion {
    fn default() -> Self { AudioFloatConversion::new() }
}

impl Transform<Tensor<i16>> for AudioFloatConversion {
    type Output = Tensor<f32>;

    fn transform(&mut self, input: Tensor<i16>) -> Self::Output {
        input.map(|_dims, &value| (value as f32 / i16::MAX as f32).clamp(-1.0, 1.0)))
    }
}

impl HasOutputs for AudioFloatConversion {
    fn set_output_dimensions(&mut self, dimensions: &[usize]) {
        assert_eq!(
            dimensions.len(),
            1,
            "This proc block only supports 1D outputs (requested output: {:?})",
            dimensions
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_empty() {
        let mut pb = AudioFloatConversion::new();
        let input = Tensor::new_vector(vec![0; 5]);

        let got = pb.transform(input);

        assert_eq!(got.dimensions(), &[5]);
    }

    #[test]
    fn does_it_match() {
        let max = i16::MAX;
        let min = i16::MIN;

        let mut pb = AudioFloatConversion::new();
        let input = Tensor::new_vector(vec![0, max, min, min+1]);

        let got = pb.transform(input);

        assert_eq!(got.elements()[0..4], [0.0, 1.0, -1.0000305, -1.0]);
    }
}
