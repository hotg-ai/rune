#![no_std]

#[cfg(test)]
extern crate std;

use core::{
    fmt::Debug,
    ops::{Div, Sub},
};
use hotg_rune_proc_blocks::{Transform, Tensor};

pub fn normalize<T>(input: &mut [T])
where
    T: PartialOrd + Div<Output = T> + Sub<Output = T> + Copy,
{
    if let Some((min, max)) = min_max(input.iter()) {
        if min != max {
            let range = max - min;

            for item in input {
                *item = (*item - min) / range;
            }
        }
    }
}

/// Normalize the input to the range `[0, 1]`.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, hotg_rune_proc_blocks::ProcBlock,
)]
#[non_exhaustive]
#[transform(inputs = [f32; 1], outputs = [f32; 1])]
#[transform(inputs = [f32; 2], outputs = [f32; 2])]
#[transform(inputs = [f32; 3], outputs = [f32; 3])]
pub struct Normalize {
    unused: &'static str,
}

impl<T> Transform<Tensor<T>> for Normalize
where
    T: PartialOrd + Div<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Tensor<T>;

    fn transform(&mut self, mut input: Tensor<T>) -> Tensor<T> {
        normalize(input.make_elements_mut());
        input
    }
}

impl<T, const N: usize> Transform<[T; N]> for Normalize
where
    T: PartialOrd + Div<Output = T> + Sub<Output = T> + Copy,
{
    type Output = [T; N];

    fn transform(&mut self, mut input: [T; N]) -> [T; N] {
        normalize(&mut input);
        input
    }
}

fn min_max<'a, I, T>(items: I) -> Option<(T, T)>
where
    I: IntoIterator<Item = &'a T> + 'a,
    T: PartialOrd + Copy + 'a,
{
    items.into_iter().fold(None, |bounds, &item| match bounds {
        Some((min, max)) => {
            let min = if item < min { item } else { min };
            let max = if max < item { item } else { max };
            Some((min, max))
        },
        None => Some((item, item)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = Tensor::from([0.0, 1.0, 2.0]);
        let mut pb = Normalize::default();

        let output = pb.transform(input);

        assert_eq!(output, [0.0, 0.5, 1.0]);
    }

    #[test]
    fn it_accepts_vectors() {
        let input = [0.0, 1.0, 2.0];
        let mut pb = Normalize::default();

        let _ = pb.transform(input);
    }

    #[test]
    fn handle_empty() {
        let input: [f32; 384] = [0.0; 384];
        let mut pb = Normalize::default();

        let output = pb.transform(input);

        assert_eq!(output, input);
        assert_eq!(output.len(), 384);
    }
}
