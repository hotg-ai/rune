#![no_std]

#[cfg(test)]
extern crate std;

use core::{
    marker::PhantomData,
    ops::{Div, Sub},
};
use runic_types::Transform;

/// Normalize the input to the range `[0, 1]`.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Normalize<T> {
    _type: PhantomData<fn(T) -> T>,
}

impl<T, A> Transform<A> for Normalize<T>
where
    A: AsMut<[T]> + AsRef<[T]>,
    T: PartialOrd + Div<Output = T> + Sub<Output = T> + Copy,
{
    type Output = A;

    fn transform(&mut self, mut input: A) -> A {
        if let Some((min, max)) = min_max(input.as_ref()) {
            if min != max {
                let range = max - min;

                for item in input.as_mut() {
                    *item = (*item - min) / range;
                }
            }
        }

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
    use crate::Normalize;
    use runic_types::Transform;

    #[test]
    fn it_works() {
        let input = [0.0, 1.0, 2.0];
        let mut pb = Normalize::default();

        let output = pb.transform(input);

        assert_eq!(output, [0.0, 0.5, 1.0]);
    }

    #[test]
    fn it_accepts_vectors() {
        let input = std::vec![0.0, 1.0, 2.0];
        let mut pb = Normalize::default();

        let _ = pb.transform(input);
    }

    #[test]
    fn it_accepts_mutable_slices() {
        let mut input = [0.0, 1.0, 2.0];
        let mut pb = Normalize::default();

        let _ = pb.transform(&mut input[..]);
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
