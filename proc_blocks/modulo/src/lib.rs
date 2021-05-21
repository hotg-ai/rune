#![no_std]

use core::ops::Rem;
use num_traits::One;
use runic_types::{HasOutputs, Tensor, Transform};

pub fn modulo<T>(modulus: T, values: &mut [T])
where
    T: Rem<Output = T> + Clone,
{
    for item in values {
        *item = item.clone() % modulus.clone();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Modulo<T> {
    modulus: T,
}

impl<T: One> Modulo<T> {
    pub fn new() -> Self { Modulo { modulus: T::one() } }
}

impl<T> Modulo<T> {
    pub fn set_modulus(&mut self, modulus: T) -> &mut Self {
        self.modulus = modulus;
        self
    }
}

impl<T: One> Default for Modulo<T> {
    fn default() -> Self { Modulo::new() }
}

impl<T> Transform<T> for Modulo<T>
where
    T: Rem<Output = T> + Clone,
{
    type Output = T;

    fn transform(&mut self, input: T) -> T { input % self.modulus.clone() }
}

impl<T, const N: usize> Transform<[T; N]> for Modulo<T>
where
    T: Rem<Output = T> + Clone,
{
    type Output = [T; N];

    fn transform(&mut self, mut input: [T; N]) -> [T; N] {
        modulo(self.modulus.clone(), &mut input);
        input
    }
}

impl<'a, T> Transform<Tensor<T>> for Modulo<T>
where
    T: Rem<Output = T> + Clone,
{
    type Output = Tensor<T>;

    fn transform(&mut self, mut input: Tensor<T>) -> Tensor<T> {
        modulo(self.modulus.clone(), input.make_elements_mut());
        input
    }
}

impl<T> HasOutputs for Modulo<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_360() {
        let number = 42 + 360;
        let mut m = Modulo::new();
        m.set_modulus(360);

        let got = m.transform(number);

        assert_eq!(got, 42_i64);
    }
}
