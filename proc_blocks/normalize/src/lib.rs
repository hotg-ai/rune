#![no_std]

#[cfg(test)]
extern crate std;

use core::{
    fmt::{self, Formatter, Debug},
    marker::PhantomData,
    ops::{Div, Sub},
};
use runic_types::{Transform, Buffer};

/// Normalize the input to the range `[0, 1]`.
pub struct Normalize<B> {
    _type: PhantomData<fn(B) -> B>,
}

impl<B> Transform<B> for Normalize<B>
where
    B: Buffer,
    B::Item: PartialOrd + Div<Output = B::Item> + Sub<Output = B::Item> + Copy,
{
    type Output = B;

    fn transform(&mut self, mut input: B) -> B {
        if let Some((min, max)) = min_max(input.as_slice()) {
            if min != max {
                let range = max - min;

                for item in input.as_mut_slice() {
                    *item = (*item - min) / range;
                }
            }
        }

        input
    }
}

impl<B> Debug for Normalize<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Normalize").finish()
    }
}

impl<B> Default for Normalize<B> {
    fn default() -> Self { Normalize { _type: PhantomData } }
}

impl<B> PartialEq for Normalize<B> {
    fn eq(&self, other: &Self) -> bool {
        let Normalize { _type: type_a } = self;
        let Normalize { _type: type_b } = other;

        type_a == type_b
    }
}

impl<B> Copy for Normalize<B> {}

impl<B> Clone for Normalize<B> {
    fn clone(&self) -> Self { *self }
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
