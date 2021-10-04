#![no_std]

#[cfg(test)]
#[macro_use]
extern crate alloc;

use num_traits::{Bounded, ToPrimitive};
use hotg_rune_proc_blocks::{ProcBlock, Transform, Tensor};

/// A normalization routine which takes some tensor of integers and fits their
/// values to the range `[0, 1]` as `f32`'s.
#[derive(Debug, Default, Clone, PartialEq, ProcBlock)]
#[non_exhaustive]
#[transform(inputs = [u8; _], outputs = [f32; _])]
#[transform(inputs = [i8; _], outputs = [f32; _])]
#[transform(inputs = [u16; _], outputs = [f32; _])]
#[transform(inputs = [i16; _], outputs = [f32; _])]
#[transform(inputs = [u32; _], outputs = [f32; _])]
#[transform(inputs = [i32; _], outputs = [f32; _])]
pub struct ImageNormalization {}

impl ImageNormalization {
    fn check_input_dimensions(&self, dimensions: &[usize]) {
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

impl<T> Transform<Tensor<T>> for ImageNormalization
where
    T: Bounded + ToPrimitive + Copy,
{
    type Output = Tensor<f32>;

    fn transform(&mut self, input: Tensor<T>) -> Self::Output {
        self.check_input_dimensions(input.dimensions());
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
        let dims = vec![1, 1, 1, 3];
        let input: Tensor<u8> =
            Tensor::new_row_major(vec![0, 127, 255].into(), dims.clone());
        let mut norm = ImageNormalization::default();
        let should_be: Tensor<f32> =
            Tensor::new_row_major(vec![0.0, 127.0 / 255.0, 1.0].into(), dims);

        let got = norm.transform(input);

        assert_eq!(got, should_be);
    }
}
