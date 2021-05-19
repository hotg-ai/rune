#![no_std]

extern crate alloc;
use runic_types::{HasOutputs, Transform};

const CHANNEL_SIZE: usize = 3;
pub struct Normalize<const N: usize> {
    values_as_float: [[[f32; CHANNEL_SIZE]; N]; N],
}

impl<const N: usize> Normalize<N> {
    pub fn new() -> Self {
        Normalize {
            values_as_float: [[[0.0; CHANNEL_SIZE]; N]; N],
        }
    }

    fn one_to_one_normalizer(&mut self, input: [[[u8; CHANNEL_SIZE]; N]; N]) {
        let scale: f32 = 2.0 / 255.0;
        let bias: f32 = -1.0;

        for (i, channel) in input.iter().enumerate() {
            for (j, row) in channel.iter().enumerate() {
                for (k, col) in row.iter().enumerate() {
                    self.values_as_float[i][j][k] =
                        (*col as f32 * scale) + bias;
                }
            }
        }
    }
}

impl<const N: usize> Transform<[[[u8; CHANNEL_SIZE]; N]; N]> for Normalize<N> {
    type Output = [[[f32; CHANNEL_SIZE]; N]; N];

    fn transform(
        &mut self,
        input: [[[u8; CHANNEL_SIZE]; N]; N],
    ) -> Self::Output {
        self.one_to_one_normalizer(input);
        return self.values_as_float;
    }
}

impl<const N: usize> Default for Normalize<N> {
    fn default() -> Self { Normalize::new() }
}

impl<const N: usize> HasOutputs for Normalize<N> {}

#[cfg(test)]
mod tests {
    use super::*;
    use runic_types::Transform;

    #[test]
    fn test_one_to_one() {
        let mut pb: Normalize<2> = Normalize::new();
        let input =
            [[[255, 0, 255], [0, 0, 0]], [[255, 255, 255], [30, 56, 133]]];

        let out = pb.transform(input);

        assert_eq!(
            out,
            [
                [[1.0, -1.0, 1.0], [-1.0, -1.0, -1.0]],
                [[1.0, 1.0, 1.0], [-0.7647059, -0.5607843, 0.043137312]]
            ]
        );
    }
}
