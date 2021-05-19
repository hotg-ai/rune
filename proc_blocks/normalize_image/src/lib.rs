#![no_std]

extern crate alloc;
use runic_types::{HasOutputs, Transform};

#[derive(Debug, Clone, PartialEq)]
pub struct Normalize<const N: usize> {
    values_as_float: [f32; N],
}

impl<const N: usize> Normalize<N> {
    pub fn new() -> Self {
        Normalize {
            values_as_float: [0.0; N],
        }
    }

    fn one_to_one_normalizer(&mut self, input: [u8; N]) {
        let scale: f32 = 2.0 / 255.0;
        let bias: f32 = -1.0;

        for (i, value) in &mut input.iter().enumerate() {
            self.values_as_float[i] = (*value as f32 * scale) + bias;
        }
    }
}

impl<const N: usize> Transform<[u8; N]> for Normalize<N> {
    type Output = [f32; N];

    fn transform(&mut self, input: [u8; N]) -> Self::Output {
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
        let mut pb: Normalize<3> = Normalize::new();
        let input = [30, 56, 133];

        let out = pb.transform(input);

        assert_eq!(out, [-0.7647059, -0.5607843, 0.043137312]);
    }
}
