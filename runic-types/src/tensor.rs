use core::convert::TryInto;

use alloc::{vec::Vec, sync::Arc};

/// A multidimensional array with copy-on-write semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct Tensor<T> {
    elements: Arc<[T]>,
    dimensions: Vec<usize>,
}

impl<T> Tensor<T> {
    pub fn with_elements(elements: Arc<[T]>, dimensions: Vec<usize>) -> Self {
        assert_eq!(dimensions.iter().product::<usize>(), elements.len());

        Tensor {
            elements,
            dimensions,
        }
    }

    pub fn zeroed(dimensions: Vec<usize>) -> Self
    where
        T: Default + Copy,
    {
        let len = dimensions.iter().product::<usize>();
        let elements = alloc::vec![T::default(); len];

        Tensor::with_elements(elements.into(), dimensions)
    }

    pub fn dimensions(&self) -> &[usize] { &self.dimensions }

    pub fn elements(&self) -> &[T] { &self.elements }

    pub fn get_elements_mut(&mut self) -> Option<&mut [T]> {
        Arc::get_mut(&mut self.elements)
    }

    pub fn view<const RANK: usize>(&self) -> Option<TensorView<'_, T, RANK>> {
        let dimensions = self.dimensions.as_slice().try_into().ok()?;

        Some(TensorView {
            elements: &self.elements,
            dimensions,
        })
    }
}

impl<T: Clone> Tensor<T> {
    pub fn make_elements_mut(&mut self) -> &mut [T] {
        // Note: we can't use Arc::make_mut() because [T] is not Clone

        if Arc::strong_count(&self.elements) > 0
            || Arc::weak_count(&self.elements) > 0
        {
            self.elements = self.elements.iter().cloned().collect();
        }

        Arc::get_mut(&mut self.elements).expect("Guaranteed to be unique")
    }
}

impl<T, const N: usize> From<[T; N]> for Tensor<T> {
    fn from(array: [T; N]) -> Self {
        Tensor::with_elements(Arc::from(array), alloc::vec![N])
    }
}

impl<'a, T: Clone> From<&'a [T]> for Tensor<T> {
    fn from(array: &'a [T]) -> Self {
        let dims = alloc::vec![array.len()];
        Tensor::with_elements(array.iter().cloned().collect(), dims)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TensorView<'t, T, const RANK: usize> {
    elements: &'t [T],
    dimensions: &'t [usize; RANK],
}

impl<'t, T, const RANK: usize> TensorView<'t, T, RANK> {
    pub fn get(&self, indices: [usize; RANK]) -> Option<&T> {
        let ix = self.index_of(indices)?;
        Some(&self.elements[ix])
    }

    pub fn index_of(&self, indices: [usize; RANK]) -> Option<usize> {
        let mut index = *indices.last().unwrap();

        for k in 0..RANK {
            for l in (k + 1)..RANK {
                index += self.dimensions[l] * indices[k];
            }
        }

        Some(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indices_for_2d_view() {
        let tensor: Tensor<u32> = Tensor::zeroed(alloc::vec![2, 3]);
        let view = tensor.view::<2>().unwrap();

        let inputs = alloc::vec![
            ([0, 0], 0),
            ([0, 1], 1),
            ([0, 2], 2),
            ([1, 0], 3),
            ([1, 1], 4),
            ([1, 2], 5),
        ];

        for (ix, should_be) in inputs {
            let got = view.index_of(ix).unwrap();
            assert_eq!(got, should_be, "for {:?}", ix);
        }
    }

    #[test]
    fn copy_on_write_semantics() {
        let mut first: Tensor<u32> = Tensor::zeroed(alloc::vec![2]);
        let second = Tensor::clone(&first);

        assert!(
            Arc::ptr_eq(&first.elements, &second.elements),
            "They start off aliased"
        );

        // try to mutate one
        first.make_elements_mut().fill(42);

        assert_eq!(second.elements(), &[0, 0], "The copy is unchanged");
        assert_eq!(first.elements(), &[42, 42], "But our original was mutated");
    }
}
