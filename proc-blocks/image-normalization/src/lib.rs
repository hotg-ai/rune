#![no_std]

use num_traits::{Bounded, ToPrimitive};
use rune_core::{HasOutputs, Tensor};
use rune_proc_blocks::{ProcBlock, Transform};

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

    Some((max - value) / (max - min))
}

impl HasOutputs for ImageNormalization {
    fn set_output_dimensions(&mut self, dimensions: &[usize]) {
        match *dimensions {
            [_, _, _, 3] => {},
            [_, _, _, channels] => panic!(
                "The number of channels should be either 1 or 3, found {}",
                channels
            ),
            _ => panic!("The image normalization proc block only supports outputs of the form [frames, rows, columns, channels], found {:?}", dimensions),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizing_with_default_distribution_is_noop() {
        let pixel_11 = [1.0, 2.0, 3.0];
        let pixel_12 = [4.0, 5.0, 6.0];
        let first_row = [pixel_11, pixel_12];
        let pixel_21 = [7.0, 8.0, 9.0];
        let pixel_22 = [10.0, 11.0, 12.0];
        let second_row = [pixel_21, pixel_22];
        let frame = [first_row, second_row];
        let image: Tensor<f32> = Tensor::from([frame]);
        let mut norm = ImageNormalization::default();

        let got = norm.transform(image.clone());

        assert_eq!(got, image);
    }
}
