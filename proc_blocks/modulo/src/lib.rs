#![no_std]

use num_traits::{FromPrimitive, ToPrimitive};
use rune_pb_core::{HasOutputs, Tensor, Transform, ProcBlock};

pub fn modulo<T>(modulus: f32, values: &mut [T])
where
    T: ToPrimitive + FromPrimitive,
{
    for item in values {
        let float = item.to_f32().unwrap();
        *item = T::from_f32(float % modulus).unwrap();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ProcBlock)]
pub struct Modulo {
    modulus: f32,
}

impl Modulo {
    pub fn new() -> Self { Modulo { modulus: 1.0 } }
}

impl Default for Modulo {
    fn default() -> Self { Modulo::new() }
}

impl<'a, T> Transform<Tensor<T>> for Modulo
where
    T: ToPrimitive + FromPrimitive,
{
    type Output = Tensor<T>;

    fn transform(&mut self, input: Tensor<T>) -> Tensor<T> {
        let modulus = self.modulus;

        input.map(|_, item| {
            let float = item.to_f32().unwrap();
            T::from_f32(float % modulus).unwrap()
        })
    }
}

impl HasOutputs for Modulo {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_360() {
        let number = 42 + 360;
        let mut m = Modulo::new();
        m.set_modulus(360.0);
        let input = Tensor::single(number);

        let got = m.transform(input);

        assert_eq!(got, Tensor::single(42_i64));
    }
}
