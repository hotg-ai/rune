use core::{convert::TryInto, ops::Index};
use alloc::{vec::Vec, sync::Arc};

/// A multidimensional array with copy-on-write semantics.
///
/// # Examples
#[derive(Debug, Clone, PartialEq)]
pub struct Tensor<T> {
    elements: Arc<[T]>,
    dimensions: Vec<usize>,
}

impl<T> Tensor<T> {
    pub fn new_row_major(elements: Arc<[T]>, dimensions: Vec<usize>) -> Self {
        assert_eq!(
            dimensions.iter().product::<usize>(),
            elements.len(),
            "A {:?}-dimension tensor can't be created from {} elements",
            dimensions,
            elements.len()
        );
        assert!(
            !dimensions.is_empty(),
            "It doesn't make sense to create a 0-dimension tensor"
        );

        Tensor {
            elements,
            dimensions,
        }
    }

    pub fn new_vector(iter: impl Iterator<Item = T>) -> Self {
        let elements: Arc<[T]> = iter.collect();
        let len = elements.len();
        Tensor::new_row_major(elements, alloc::vec![len])
    }

    pub fn zeroed(dimensions: Vec<usize>) -> Self
    where
        T: Default + Copy,
    {
        let len = dimensions.iter().product::<usize>();
        let elements = alloc::vec![T::default(); len];

        Tensor::new_row_major(elements.into(), dimensions)
    }

    pub fn filled_with<F>(dimensions: Vec<usize>, f: F) -> Self
    where
        F: FnMut() -> T,
    {
        let mut elements = Vec::new();
        let len = dimensions.iter().product();
        elements.resize_with(len, f);

        Tensor::new_row_major(elements.into(), dimensions)
    }

    /// Get the [`Tensor`]'s dimensions.
    pub fn dimensions(&self) -> &[usize] { &self.dimensions }

    /// Get an immutable reference to the underlying elements in this
    /// [`Tensor`].
    pub fn elements(&self) -> &[T] { &self.elements }

    /// Get a mutable reference to the underlying elements in this [`Tensor`].
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
        Tensor::new_row_major(Arc::from(array), alloc::vec![N])
    }
}

impl<'a, T: Clone> From<&'a [T]> for Tensor<T> {
    fn from(array: &'a [T]) -> Self {
        let dims = alloc::vec![array.len()];
        Tensor::new_row_major(array.iter().cloned().collect(), dims)
    }
}

/// An immutable view into a [`Tensor`] with a particular rank (number of
/// dimensions).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TensorView<'t, T, const RANK: usize> {
    elements: &'t [T],
    dimensions: &'t [usize; RANK],
}

impl<'t, T, const RANK: usize> TensorView<'t, T, RANK> {
    pub fn elements(&self) -> &'t [T] { self.elements }

    pub fn dimensions(&self) -> &'t [usize] { self.dimensions }

    pub fn get(&self, indices: [usize; RANK]) -> Option<&T> {
        let ix = self.index_of(indices)?;
        Some(&self.elements[ix])
    }

    fn index_of(&self, indices: [usize; RANK]) -> Option<usize> {
        if indices
            .iter()
            .zip(self.dimensions)
            .any(|(ix, max)| ix >= max)
        {
            return None;
        }

        let mut index = *indices.last().unwrap();

        for k in 0..RANK {
            for l in (k + 1)..RANK {
                index += self.dimensions[l] * indices[k];
            }
        }

        Some(index)
    }
}

impl<'t, T, const RANK: usize> Index<[usize; RANK]>
    for TensorView<'t, T, RANK>
{
    type Output = T;

    #[track_caller]
    fn index(&self, index: [usize; RANK]) -> &Self::Output {
        match self.get(index) {
            Some(value) => value,
            None => panic!("index out of bounds: the index was {:?} but the tensor has dimensions of {:?}", index, self.dimensions)
        }
    }
}

impl<'t, T> Index<usize> for TensorView<'t, T, 1> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output { &self[[index]] }
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

    #[test]
    fn incorrect_view_dimensions() {
        let tensor: Tensor<u32> = Tensor::zeroed(alloc::vec![2, 3]);

        assert!(tensor.view::<1>().is_none());
        assert!(tensor.view::<1>().is_none());
    }

    #[test]
    #[should_panic(
        expected = "index out of bounds: the index was [2, 0] but the tensor has dimensions of [2, 3]"
    )]
    fn index_out_of_bounds() {
        let tensor: Tensor<u32> = Tensor::zeroed(alloc::vec![2, 3]);
        let view = tensor.view::<2>().unwrap();

        let _ = view[[2, 0]];
    }
}
