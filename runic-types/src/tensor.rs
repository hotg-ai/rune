use core::{convert::TryInto, ops::Index};
use alloc::{sync::Arc, vec::Vec};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

    pub fn new_vector(iter: impl IntoIterator<Item = T>) -> Self {
        let elements: Arc<[T]> = iter.into_iter().collect();
        let len = elements.len();
        Tensor::new_row_major(elements, alloc::vec![len])
    }

    pub fn zeroed(dimensions: Vec<usize>) -> Self
    where
        T: Default,
    {
        Tensor::filled_with(dimensions, Default::default)
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

    /// The number of dimensions this [`Tensor`] has.
    pub fn rank(&self) -> usize { self.dimensions().len() }

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

    pub fn as_ptr_and_byte_length(&self) -> (*const u8, usize) {
        let elements = self.elements();
        let byte_length = elements.len() * core::mem::size_of::<T>();
        (elements.as_ptr().cast(), byte_length)
    }

    /// Create a new [`Tensor`] by applying a function to every element in the
    /// current tensor.
    ///
    /// This is often an expensive operation.
    #[must_use]
    pub fn map<F, Out>(&self, mut map: F) -> Tensor<Out>
    where
        F: FnMut(&[usize], &T) -> Out,
        Out: Clone + Default,
    {
        let mut counter = Counter::new(self.dimensions());
        let mut output = Tensor::zeroed(self.dimensions().to_vec());

        let elements = output.make_elements_mut();

        while let Some(indices) = counter.next() {
            let index = index_of(self.dimensions(), indices).unwrap();
            let previous = &self.elements[index];
            elements[index] = map(indices, previous);
        }

        output
    }
}

impl<T: PartialEq> PartialEq<[T]> for Tensor<T> {
    fn eq(&self, other: &[T]) -> bool {
        self.dimensions.len() == 1 && self.elements() == other
    }
}

impl<T: PartialEq> PartialEq<Tensor<T>> for [T] {
    fn eq(&self, other: &Tensor<T>) -> bool { other == self }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for Tensor<T> {
    fn eq(&self, other: &[T; N]) -> bool { self == &other[..] }
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
        index_of(self.dimensions, &indices)
    }
}

fn index_of(dimensions: &[usize], indices: &[usize]) -> Option<usize> {
    if indices.iter().zip(dimensions).any(|(ix, max)| ix >= max) {
        return None;
    }
    if dimensions.len() != indices.len() {
        return None;
    }

    let mut index = *indices.last().unwrap();

    for k in 0..dimensions.len() {
        for l in (k + 1)..dimensions.len() {
            index += dimensions[l] * indices[k];
        }
    }

    Some(index)
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

impl<T: Serialize> Serialize for Tensor<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(serde::Serialize)]
        struct S<'a, Item> {
            elements: &'a [Item],
            dimensions: &'a [usize],
        }

        let s = S {
            elements: self.elements(),
            dimensions: self.dimensions(),
        };

        s.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Tensor<T> {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct D<Item> {
            elements: Vec<Item>,
            dimensions: Vec<usize>,
        }

        let D {
            elements,
            dimensions,
        } = D::deserialize(de)?;

        Ok(Tensor::<T>::new_row_major(elements.into(), dimensions))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Counter<'a> {
    current: Vec<usize>,
    max: &'a [usize],
    first: bool,
}

impl<'a> Counter<'a> {
    fn new(max: &'a [usize]) -> Self {
        Counter {
            current: alloc::vec![0; max.len()],
            max,
            first: true,
        }
    }

    fn next(&mut self) -> Option<&[usize]> {
        if self.first {
            self.first = false;
            return Some(&self.current);
        }

        for (i, index) in self.current.iter_mut().rev().enumerate() {
            let i = self.max.len() - i - 1;

            *index += 1;
            if *index >= self.max[i] {
                *index = 0;
                continue;
            }

            return Some(&self.current);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::*;

    #[test]
    fn indices_for_2d_view() {
        let tensor: Tensor<u32> = Tensor::zeroed(vec![2, 3]);
        let view = tensor.view::<2>().unwrap();

        let inputs = vec![
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
        let mut first: Tensor<u32> = Tensor::zeroed(vec![2]);
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
        let tensor: Tensor<u32> = Tensor::zeroed(vec![2, 3]);

        assert!(tensor.view::<1>().is_none());
        assert!(tensor.view::<1>().is_none());
    }

    #[test]
    #[should_panic(
        expected = "index out of bounds: the index was [2, 0] but the tensor has dimensions of [2, 3]"
    )]
    fn index_out_of_bounds() {
        let tensor: Tensor<u32> = Tensor::zeroed(vec![2_usize, 3]);
        let view = tensor.view::<2>().unwrap();

        let _ = view[[2, 0]];
    }

    #[test]
    fn map_the_elements() {
        let tensor: Tensor<u32> =
            Tensor::new_row_major(vec![1, 2, 3, 4, 5, 6].into(), vec![2, 3]);
        let mut indices = Vec::new();

        let got = tensor.map(|index, &element| {
            indices.push(index.to_vec());
            element * 2
        });

        let should_be = tensor.elements.iter().map(|&e| e * 2).collect();
        let should_be =
            Tensor::new_row_major(should_be, tensor.dimensions().to_vec());

        assert_eq!(got, should_be);

        let expected_order = vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![1, 1],
            vec![1, 2],
        ];
        assert_eq!(indices, expected_order);
    }

    fn collect_counter(mut counter: Counter<'_>) -> Vec<Vec<usize>> {
        let mut indices = Vec::new();

        while let Some(index) = counter.next() {
            indices.push(index.to_vec());
            println!("{:?}", counter);
        }

        indices
    }

    #[test]
    fn one_dimension_counter() {
        let counter = Counter::new(&[4]);
        let should_be = vec![vec![0], vec![1], vec![2], vec![3]];

        let got = collect_counter(counter);

        assert_eq!(got, should_be);
    }
}
