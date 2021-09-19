#![no_std]

use num_traits::{Bounded, ToPrimitive};
use hotg_rune_core::{Tensor};
use hotg_rune_proc_blocks::{ProcBlock, Transform};

/// A normalization routine which takes some tensor of integers and fits their
/// values to the range `[0, 1]` as `f32`'s.
#[derive(Debug, Default, Clone, PartialEq, ProcBlock)]
#[non_exhaustive]
#[transform(input = [u8; _], output = [f32; _])]
#[transform(input = [i8; _], output = [f32; _])]
#[transform(input = [u16; _], output = [f32; _])]
#[transform(input = [i16; _], output = [f32; _])]
#[transform(input = [u32; _], output = [f32; _])]
#[transform(input = [i32; _], output = [f32; _])]
pub struct ImageNormalization {}

impl<T> Transform<Tensor<T>> for ImageNormalization
where
    T: Bounded + ToPrimitive + Copy,
{
    type Output = Tensor<f32>;

    fn transform(&mut self, input: Tensor<T>) -> Self::Output {
        input.map(|_, &value| normalize(value).expect("Cast should never fail"))
    }
}

fn normalize<T>(value: T) -> Option<f32>
where
    T: Bounded + ToPrimitive,
{
    let min = T::min_value().to_f32()?;
    let max = T::max_value().to_f32()?;
    let value = value.to_f32()?;
    debug_assert!(min <= value && value <= max);

    Some((value - min) / (max - min))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizing_with_default_distribution_is_noop() {
        let input: Tensor<u8> = Tensor::from([0, 127, 255]);
        let mut norm = ImageNormalization::default();
        let should_be: Tensor<f32> = Tensor::from([0.0, 127.0 / 255.0, 1.0]);

        let got = norm.transform(input);

        assert_eq!(got, should_be);
    }
}
