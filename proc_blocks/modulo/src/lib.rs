#![no_std]

use core::ops::Rem;
use num_traits::One;
use runic_types::Transform;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Modulo<T> {
    modulus: T,
}

impl<T: One> Modulo<T> {
    pub fn new() -> Self { Modulo { modulus: T::one() } }

    pub fn with_modulus(self, modulus: T) -> Self { Modulo { modulus } }
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
        for item in &mut input {
            *item = item.clone() % self.modulus.clone();
        }

        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_360() {
        let number = 42 + 360;
        let mut m = Modulo::new().with_modulus(360);

        let got = m.transform(number);

        assert_eq!(got, 42_i64);
    }
}
