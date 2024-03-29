use alloc::{sync::Arc, vec::Vec};
use core::{
    convert::TryInto,
    fmt::{self, Display, Formatter},
    iter::FromIterator,
    ops::{Index, IndexMut},
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{element_type::AsElementType, Shape};

/// A multidimensional array with copy-on-write semantics.
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

    pub fn single(value: T) -> Self {
        Tensor::new_vector(core::iter::once(value))
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

    pub fn shape(&self) -> Shape<'_>
    where
        T: AsElementType,
    {
        Shape::new(T::TYPE, self.dimensions())
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

    /// Try to get a contiguous sub-slice of this tensor.
    ///
    /// # Note
    ///
    /// Due to the way tensors are laid out in memory, you can only slice off
    /// the leading dimensions.
    ///
    /// In order to be well-formed, this requires that
    /// `RANK + leading_indices.len()` equals [`Tensor::rank()`], and that each
    /// of the indices are within bounds.
    ///
    /// # Examples
    ///
    /// Say you have a `[3, 4, 2]` tensor, passing in `[0, 1]` would
    /// give you a 1D [`TensorView`] that views the 3 elements at
    /// `[0, 1, ..]`.
    ///
    /// ```rust
    /// # use hotg_rune_core::Tensor;
    /// let input = [
    ///     [[0, 1], [2, 3], [4, 5], [6, 7]],
    ///     [[8, 9], [10, 11], [12, 13], [14, 15]],
    ///     [[16, 17], [18, 19], [20, 21], [22, 23]],
    /// ];
    /// let tensor: Tensor<i32> = input.into();
    ///
    /// let got = tensor.slice::<1>(&[0, 1]).unwrap();
    ///
    /// assert_eq!(got.dimensions(), [2]);
    /// assert_eq!(got.elements(), &input[0][1]);
    /// ```
    pub fn slice<const RANK: usize>(
        &self,
        leading_indices: &[usize],
    ) -> Option<TensorView<'_, T, RANK>> {
        let (dimensions, range) =
            slice_indices::<RANK>(self.dimensions(), leading_indices)?;

        let elements = &self.elements[range];

        Some(TensorView {
            elements,
            dimensions,
        })
    }

    /// A mutable version of [`Tensor::slice()`].
    pub fn slice_mut<const RANK: usize>(
        &mut self,
        leading_indices: &[usize],
    ) -> Option<TensorViewMut<'_, T, RANK>>
    where
        T: Clone,
    {
        let (dimensions, range) =
            slice_indices::<RANK>(self.dimensions(), leading_indices)?;

        let elements = self.make_elements_mut();
        let elements = &mut elements[range];

        Some(TensorViewMut {
            elements,
            dimensions,
        })
    }

    /// Try to reinterpret this tensor as a `RANK`-D tensor.
    pub fn view<const RANK: usize>(&self) -> Option<TensorView<'_, T, RANK>> {
        self.slice::<RANK>(&[])
    }

    /// The mutable version of [`Tensor::view()`].
    pub fn view_mut<'a, 'this: 'a, const RANK: usize>(
        &'this mut self,
    ) -> Option<TensorViewMut<'a, T, RANK>>
    where
        T: Clone,
    {
        self.slice_mut::<RANK>(&[])
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
    {
        let mut counter = Counter::new(self.dimensions());

        // Note: this implicitly requires our counter and iteration to both
        // follow row-major order.
        let elements = self.elements().iter().map(|item| {
            let index = counter
                .next()
                .expect("The counter should be in sync with iteration");
            map(index, item)
        });

        Tensor::new_row_major(elements.collect(), self.dimensions.clone())
    }

    /// Get a reference to the element with this particular index.
    pub fn get(&self, indices: &[usize]) -> Option<&T> {
        let element_index = index_of(self.dimensions(), indices).ok()?;
        #[cfg(test)]
        println!(
            "{:?} from {:?} => {}",
            indices,
            self.dimensions(),
            element_index
        );
        self.elements.get(element_index)
    }

    /// Get a mutable reference to the element with this particular index.
    pub fn get_mut(&mut self, indices: &[usize]) -> Option<&mut T>
    where
        T: Clone,
    {
        let element_index = index_of(self.dimensions(), indices).ok()?;
        self.make_elements_mut().get_mut(element_index)
    }
}

/// The index math that powers [`Tensor::slice()`] and
/// [`Tensor::slice_mut()`].
///
/// Given a set of leading indices, we need to figure out which section of
/// [`Tensor::elements()`] is being referred and the shape of that section.
fn slice_indices<const RANK: usize>(
    dimensions: &[usize],
    leading_indices: &[usize],
) -> Option<([usize; RANK], core::ops::Range<usize>)> {
    if leading_indices.len() >= dimensions.len() {
        // The user is trying to slice off (for example) the first 5
        // dimensions from a 3D tensor.
        return None;
    }

    if RANK + leading_indices.len() != dimensions.len() {
        // The total number of dimensions must equal the number of
        // dimensions we are slicing off (leading_indices) plus the rank of
        // the resulting TensorView.
        return None;
    }

    let (front, rest) = dimensions.split_at(leading_indices.len());

    // The leading indices must all be within bounds. So if our tensor's
    // dimensions() returned [1, 5, 6, 3], you could pass in &[0, 0] to
    // get all elements [0, 0, .., ..], but passing in &[5] would be out of
    // bounds because the first index must be < 1.
    if front
        .iter()
        .zip(leading_indices)
        .any(|(max, value)| *value > *max)
    {
        return None;
    }

    let number_of_elements: usize = rest.iter().copied().product();
    let start_index = if leading_indices.is_empty() {
        0
    } else {
        let index_without_stride = index_of(front, leading_indices)
            .expect("Should have already done bounds checks");
        let stride: usize = rest.iter().product();

        index_without_stride * stride
    };

    let range = start_index..start_index + number_of_elements;

    let dimensions = rest
        .try_into()
        .expect("We've already checked that the ranks add up");

    Some((dimensions, range))
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

impl<T> FromIterator<T> for Tensor<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Tensor::new_vector(iter)
    }
}

impl<T, Ix> Index<Ix> for Tensor<T>
where
    Ix: AsRef<[usize]>,
{
    type Output = T;

    fn index(&self, index: Ix) -> &Self::Output {
        let index = index.as_ref();

        match self.get(index) {
            Some(value) => value,
            None => panic!(
                "Tried to get item {:?} from a tensor with dimensions {:?}",
                index,
                self.dimensions()
            ),
        }
    }
}

impl<T, Ix> IndexMut<Ix> for Tensor<T>
where
    Ix: AsRef<[usize]>,
    T: Clone,
{
    fn index_mut(&mut self, index: Ix) -> &mut Self::Output {
        let index = index.as_ref();

        match self.get_mut(index) {
            Some(value) => value,
            None => panic!(
                "Tried to get item {:?} from a tensor, but it is out of bounds",
                index,
            ),
        }
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

impl<T: Clone, const WIDTH: usize, const HEIGHT: usize>
    From<[[T; WIDTH]; HEIGHT]> for Tensor<T>
{
    fn from(array: [[T; WIDTH]; HEIGHT]) -> Self {
        let elements =
            array.iter().flat_map(|row| row.iter()).cloned().collect();
        Tensor::new_row_major(elements, alloc::vec![HEIGHT, WIDTH])
    }
}

impl<T: Clone, const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize>
    From<[[[T; WIDTH]; HEIGHT]; DEPTH]> for Tensor<T>
{
    fn from(array: [[[T; WIDTH]; HEIGHT]; DEPTH]) -> Self {
        let elements = array
            .iter()
            .flat_map(|row| row.iter())
            .flat_map(|column| column.iter())
            .cloned()
            .collect();
        Tensor::new_row_major(elements, alloc::vec![DEPTH, HEIGHT, WIDTH])
    }
}

macro_rules! array_type {
    ($element:ident, [$dim:ident $(,)?]) => { [$element; $dim] };
    ($element:ident, [$dim:ident, $($rest:ident),*]) => {
        [array_type!($element, [$($rest),*]); $dim]
    };
}

macro_rules! from_array {
    ($($name:ident : $letter:ident),*) => {
        impl<Element: Clone, $(const $letter: usize),*> From<array_type!(Element, [$($letter),*])> for Tensor<Element> {
            fn from(array: array_type!(Element, [$($letter),*])) -> Self {
                let elements = core::iter::once(&array)
                    $(
                        .flat_map(|$name| $name.iter())
                    )*
                    .cloned()
                    .collect();
                Tensor::new_row_major(elements, alloc::vec![$($letter),*])
            }
        }
    };
}

from_array!(a: A, b: B, c: C, d: D);
from_array!(a: A, b: B, c: C, d: D, e: E);
from_array!(a: A, b: B, c: C, d: D, e: E, f: F);
from_array!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
from_array!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);

impl<'a, T: Clone> From<&'a [T]> for Tensor<T> {
    fn from(array: &'a [T]) -> Self {
        let dims = alloc::vec![array.len()];
        Tensor::new_row_major(array.iter().cloned().collect(), dims)
    }
}

fn index_of(
    dimensions: &[usize],
    indices: &[usize],
) -> Result<usize, IndexError> {
    if dimensions.len() != indices.len() {
        return Err(IndexError::MismatchedRank {
            dimension_length: dimensions.len(),
            indices_length: indices.len(),
        });
    }
    if dimensions.is_empty() {
        return Err(IndexError::ZeroVector);
    }

    let rank = dimensions.len();
    let mut index = 0;

    for i in 0..rank {
        let ix = indices[i];
        let dim = dimensions[i];

        if ix >= dim {
            return Err(IndexError::IndexTooLarge {
                dimension: i,
                max_value: dim,
                found: ix,
            });
        }

        let stride: usize = dimensions[i + 1..].iter().product();

        index += ix * stride;
    }

    Ok(index)
}

#[cold]
#[track_caller]
fn on_index_error(
    e: IndexError,
    indices: impl AsRef<[usize]>,
    dimensions: impl AsRef<[usize]>,
) -> ! {
    panic!(
        "{} (index: {:?}, dimensions: {:?})",
        e,
        indices.as_ref(),
        dimensions.as_ref(),
    );
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum IndexError {
    MismatchedRank {
        dimension_length: usize,
        indices_length: usize,
    },
    IndexTooLarge {
        dimension: usize,
        max_value: usize,
        found: usize,
    },
    ZeroVector,
}

impl Display for IndexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IndexError::MismatchedRank {
                dimension_length,
                indices_length,
            } => write!(
                f,
                "Unable to index into a {}-dimension tensor with a \
                 {}-dimension index",
                dimension_length, indices_length
            ),
            IndexError::IndexTooLarge {
                dimension,
                max_value,
                found,
            } => write!(
                f,
                "Index {} should be less than {}, but found {}",
                dimension, max_value, found
            ),
            IndexError::ZeroVector => {
                write!(f, "Unable to index into the zero vector")
            },
        }
    }
}

/// An immutable view into a [`Tensor`] with a particular rank (number of
/// dimensions).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TensorView<'t, T, const RANK: usize> {
    elements: &'t [T],
    dimensions: [usize; RANK],
}

impl<'t, T, const RANK: usize> TensorView<'t, T, RANK> {
    pub fn elements(&self) -> &'t [T] { self.elements }

    pub fn dimensions(&self) -> [usize; RANK] { self.dimensions }

    pub fn get(&self, indices: [usize; RANK]) -> Option<&T> {
        let ix = self.index_of(indices).ok()?;
        Some(&self.elements[ix])
    }

    fn index_of(&self, indices: [usize; RANK]) -> Result<usize, IndexError> {
        index_of(&self.dimensions, &indices)
    }

    /// The [`TensorView`] version of [`Tensor::slice()`].
    pub fn slice<const NEW_RANK: usize>(
        &self,
        leading_indices: &[usize],
    ) -> Option<TensorView<'_, T, NEW_RANK>> {
        let (dimensions, range) =
            slice_indices::<NEW_RANK>(&self.dimensions(), leading_indices)?;

        let elements = &self.elements[range];

        Some(TensorView {
            elements,
            dimensions,
        })
    }
}

impl<'t, T, const RANK: usize> Index<[usize; RANK]>
    for TensorView<'t, T, RANK>
{
    type Output = T;

    #[track_caller]
    fn index(&self, index: [usize; RANK]) -> &Self::Output {
        match self.index_of(index) {
            Ok(value) => &self.elements[value],
            Err(e) => on_index_error(e, index, self.dimensions),
        }
    }
}

impl<'t, T> Index<usize> for TensorView<'t, T, 1> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output { &self[[index]] }
}

/// A mutable view into a [`Tensor`] with a particular rank (number of
/// dimensions).
#[derive(Debug, PartialEq)]
pub struct TensorViewMut<'t, T, const RANK: usize> {
    elements: &'t mut [T],
    dimensions: [usize; RANK],
}

impl<'t, T, const RANK: usize> TensorViewMut<'t, T, RANK> {
    pub fn elements(&mut self) -> &mut [T] { self.elements }

    pub fn dimensions(&self) -> [usize; RANK] { self.dimensions }

    pub fn get(&self, indices: [usize; RANK]) -> Option<&T> {
        let ix = self.index_of(indices).ok()?;
        Some(&self.elements[ix])
    }

    pub fn get_mut(&mut self, indices: [usize; RANK]) -> Option<&mut T> {
        let ix = self.index_of(indices).ok()?;
        Some(&mut self.elements[ix])
    }

    fn index_of(&self, indices: [usize; RANK]) -> Result<usize, IndexError> {
        index_of(&self.dimensions, &indices)
    }

    pub fn slice<const NEW_RANK: usize>(
        &self,
        leading_indices: &[usize],
    ) -> Option<TensorView<'_, T, NEW_RANK>> {
        let (dimensions, range) =
            slice_indices::<NEW_RANK>(&self.dimensions(), leading_indices)?;

        let elements = &self.elements[range];

        Some(TensorView {
            elements,
            dimensions,
        })
    }

    /// The [`TensorViewMut`] version of  [`Tensor::slice_mut()`].
    pub fn slice_mut<const NEW_RANK: usize>(
        &mut self,
        leading_indices: &[usize],
    ) -> Option<TensorViewMut<'_, T, RANK>>
    where
        T: Clone,
    {
        let (dimensions, range) =
            slice_indices::<RANK>(&self.dimensions(), leading_indices)?;

        let elements = &mut self.elements[range];

        Some(TensorViewMut {
            elements,
            dimensions,
        })
    }
}

impl<'t, T, const RANK: usize> Index<[usize; RANK]>
    for TensorViewMut<'t, T, RANK>
{
    type Output = T;

    #[track_caller]
    fn index(&self, index: [usize; RANK]) -> &Self::Output {
        match self.index_of(index) {
            Ok(value) => &self.elements[value],
            Err(e) => on_index_error(e, index, self.dimensions),
        }
    }
}
impl<'t, T, const RANK: usize> IndexMut<[usize; RANK]>
    for TensorViewMut<'t, T, RANK>
{
    #[track_caller]
    fn index_mut(&mut self, index: [usize; RANK]) -> &mut Self::Output {
        match self.index_of(index) {
            Ok(ix) => &mut self.elements[ix],
            Err(e) => on_index_error(e, index, self.dimensions),
        }
    }
}

impl<'t, T> Index<usize> for TensorViewMut<'t, T, 1> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output { &self[[index]] }
}

impl<'t, T> IndexMut<usize> for TensorViewMut<'t, T, 1> {
    #[track_caller]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self[[index]]
    }
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
    use std::prelude::v1::*;

    use super::*;

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
    #[should_panic(expected = "Index 0 should be less than 2, but found 2 \
                               (index: [2, 0], dimensions: [2, 3])")]
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

    #[test]
    fn convert_from_1d_array() {
        let input = [1.0, 2.0, 3.0];

        let got = Tensor::from(input);

        assert_eq!(got.dimensions(), &[3]);
        assert_eq!(got.elements(), input);
    }

    #[test]
    fn convert_from_2d_array() {
        let input = [[0, 1, 2], [3, 4, 5]];

        let got: Tensor<i32> = input.into();

        let view = got.view::<2>().unwrap();
        let coordinates_and_indices = vec![
            ([0, 0], 0),
            ([0, 1], 1),
            ([0, 2], 2),
            ([1, 0], 3),
            ([1, 1], 4),
            ([1, 2], 5),
        ];
        for (index, should_be) in coordinates_and_indices {
            assert_eq!(view[index], should_be);
        }
    }

    #[test]
    fn convert_from_3d_array() {
        // double-checked using numpy
        let input = [
            [[0, 1], [2, 3], [4, 5], [6, 7]],
            [[8, 9], [10, 11], [12, 13], [14, 15]],
            [[16, 17], [18, 19], [20, 21], [22, 23]],
        ];
        let elements_should_be = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            19, 20, 21, 22, 23,
        ];

        let got: Tensor<i32> = input.into();

        assert_eq!(got.dimensions(), &[3, 4, 2]);
        assert_eq!(got.elements(), elements_should_be);
    }

    #[test]
    fn slice_off_the_first_dimension_from_a_3d_tensor() {
        let input = [
            [[0, 1], [2, 3], [4, 5], [6, 7]],
            [[8, 9], [10, 11], [12, 13], [14, 15]],
            [[16, 17], [18, 19], [20, 21], [22, 23]],
        ];
        let tensor: Tensor<i32> = input.into();

        let got = tensor.slice::<2>(&[0]).unwrap();

        assert_eq!(got.dimensions(), [4, 2]);
        assert_eq!(got.elements(), &[0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn slice_off_the_first_two_dimensions_from_a_3d_tensor() {
        let input = [
            [[0, 1], [2, 3], [4, 5], [6, 7]],
            [[8, 9], [10, 11], [12, 13], [14, 15]],
            [[16, 17], [18, 19], [20, 21], [22, 23]],
        ];
        let tensor: Tensor<i32> = input.into();

        let got = tensor.slice::<1>(&[1, 3]).unwrap();

        assert_eq!(got.dimensions(), [2]);
        assert_eq!(got.elements(), &[14, 15]);
    }

    #[test]
    fn you_can_index_into_tensors() {
        let tensor: Tensor<i32> = [
            [[0, 1], [2, 3], [4, 5], [6, 7]],
            [[8, 9], [10, 11], [12, 13], [14, 15]],
            [[16, 17], [18, 19], [20, 21], [22, 23]],
        ]
        .into();

        let inputs = vec![
            ([0, 0, 0], 0),
            ([0, 0, 1], 1),
            ([0, 1, 0], 2),
            ([1, 0, 0], 8),
            ([1, 1, 1], 11),
            ([1, 2, 0], 12),
            ([2, 3, 1], 23),
        ];

        for (index, should_be) in inputs {
            let got = tensor[index];
            assert_eq!(got, should_be);
        }
    }
}
