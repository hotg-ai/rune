//! Abstractions for dealing with tuples of tensors.
//!
//! The main source of complexity in this module comes from jumping between the
//! world of types (e.g. an input parameter of `(Tensor<f32>, Tensor<u8>)`) and
//! the world of values (`&[Shape]`) so we can pass type information back to
//! the runtime.
//!
//! I apologise in advance for all the generic gymnastics.

use crate::{Shape, Tensor, element_type::AsElementType};

/// A helper trait which lets us get the shape from a tuple of different
/// tensors.
///
/// Note that this is implemented for references (i.e. `&Tensor<i16>` and
/// `&(Tensor<f32>, Tensor<u8>)`) so we can carry the lifetime around. Having
/// access to the lifetime means we can borrow from the `Shape`'s dimension
/// array instead of cloning them.
pub trait TensorList<'a> {
    // Note: we could get rid of all these XXXBuffer types if const generics
    // were more fleshed out. You'd just need to define an associated
    // `const RANK: usize` then have each of the methods return something like
    // `[_; Self::Rank]`.
    //
    // It would also let us get rid of the lifetime because we can use
    // `fn shape_list(&self) -> [Shape<'_>; Self::Rank]`.

    type ShapeBuffer: AsRef<[Shape<'a>]>;
    type ConstElementPtrBuffer: AsRef<[*const u8]>;

    /// Get an array containing the [`Shape`] for each [`Tensor`] in this tuple.
    fn shape_list(&self) -> Self::ShapeBuffer;

    /// Get an array containing pointers to the elements of each [`Tensor`] in
    /// this tuple.
    fn element_ptr(&self) -> Self::ConstElementPtrBuffer;
}

/// A set of tensors that can be mutated.
pub trait TensorListMut {
    type MutElementPtrBuffer: AsMut<[*mut u8]>;

    /// Create a new set of empty tensors with the specified shape.
    ///
    /// # Panics
    ///
    /// This will panic if the shape doesn't match this [`TensorListMut`]
    /// because there are a different number of tensors or one of the tensors
    /// is the wrong type.
    fn new_tensors(shape: &[Shape<'_>]) -> Self;

    /// Get a set of mutable pointers into each tensor's backing buffer.
    fn element_ptr_mut(&mut self) -> Self::MutElementPtrBuffer;
}

impl<'a, T> TensorList<'a> for &'a Tensor<T>
where
    T: AsElementType,
{
    type ConstElementPtrBuffer = [*const u8; 1];
    type ShapeBuffer = [Shape<'a>; 1];

    fn shape_list(&self) -> Self::ShapeBuffer { [self.shape()] }

    fn element_ptr(&self) -> Self::ConstElementPtrBuffer {
        [self.elements().as_ptr() as *const u8]
    }
}

impl<T> TensorListMut for Tensor<T>
where
    T: AsElementType + Default + Clone,
{
    type MutElementPtrBuffer = [*mut u8; 1];

    fn new_tensors(shape: &[Shape<'_>]) -> Self {
        match shape {
            [s] => {
                assert_eq!(s.element_type(), T::TYPE, "Incorrect element type");
                Tensor::zeroed(s.dimensions().to_vec())
            },
            _ => panic!("Expected a shape with 1 element, found {:?}", shape),
        }
    }

    fn element_ptr_mut(&mut self) -> Self::MutElementPtrBuffer {
        [self.make_elements_mut().as_mut_ptr() as *mut u8]
    }
}

/// Count the number of identifiers that were passed in.
macro_rules! count {
    ($first:ident $(, $rest:ident)* $(,)*) => { 1 + count!($($rest),*) };
    () => { 0 };
}

/// Recursively implement our traits for tuples of any length up to a finite
/// number.
macro_rules! reflection_type_list {
    ($first:ident $(, $dim:ident)* $(,)*) => {
        #[allow(non_snake_case)]
        impl<'a, $first, $($dim),*> TensorList<'a> for &'a (Tensor<$first>, $(Tensor<$dim>),*)
        where
            $first: AsElementType,
            $($dim: AsElementType),*
        {
            type ShapeBuffer = [Shape<'a>; count!($first, $($dim),*)];
            type ConstElementPtrBuffer  = [*const u8; count!($first, $($dim),*)];

            fn shape_list(&self) -> Self::ShapeBuffer {
                let ($first, $($dim),* ) = self;

                [
                    $first.shape(),
                    $( $dim.shape()),*
                ]
            }


            fn element_ptr(&self) -> Self::ConstElementPtrBuffer {
                let ($first, $($dim),* ) = self;

                [
                    $first.elements().as_ptr() as *const u8,
                    $( $dim.elements().as_ptr() as *const u8),*
                ]
            }
        }

        #[allow(non_snake_case)]
        impl<$first, $($dim),*> TensorListMut for (Tensor<$first>, $(Tensor<$dim>),*)
        where
            $first: AsElementType + Default + Clone,
            $( $dim: AsElementType + Default + Clone ),*
        {
            type MutElementPtrBuffer  = [*mut u8; count!($first, $($dim),*)];

            fn new_tensors(shape: &[Shape<'_>]) -> Self {
                match shape {
                    [$first, $($dim),*] => {
                        assert_eq!($first.element_type(), <$first>::TYPE, "Incorrect element type");
                        let $first = Tensor::zeroed($first.dimensions().to_vec());
                        $(
                            assert_eq!($dim.element_type(), <$dim>::TYPE, "Incorrect element type");
                            let $dim = Tensor::zeroed($dim.dimensions().to_vec());
                        )*

                        ($first, $($dim),*)
                    },
                    _ => panic!(
                        "Expected a shape with {} elements, found {:?}",
                        count!($first, $($dim),*),
                        shape,
                    ),
                }
            }

            fn element_ptr_mut(&mut self) -> Self::MutElementPtrBuffer {
                let ($first, $($dim),* ) = self;

                [
                    $first.make_elements_mut().as_mut_ptr() as *mut u8,
                    $( $dim.make_elements_mut().as_mut_ptr() as *mut u8),*
                ]
            }
        }

        reflection_type_list!($( $dim ),*);
    };
    ($(,)*) => {};
}

reflection_type_list!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
    Z, AA, AB, AC, AD, AF, AG, AH
);

#[cfg(test)]
mod tests {
    use crate::element_type::ElementType;

    use super::*;
    use std::prelude::v1::*;

    #[test]
    fn count_idents() {
        let inputs = vec![
            (count!(), 0),
            (count!(A), 1),
            (count!(A, B), 2),
            (count!(A, B, C), 3),
        ];

        for (count, should_be) in inputs {
            assert_eq!(count, should_be);
        }
    }

    #[test]
    fn create_empty_tensors_from_shapes_list() {
        let shapes = [
            Shape::new(ElementType::F32, [1_usize].as_ref()),
            Shape::new(ElementType::U8, [3_usize, 256, 256].as_ref()),
            Shape::new(ElementType::I16, [1920_usize].as_ref()),
        ];

        let (a, b, c) =
            <(Tensor<f32>, Tensor<u8>, Tensor<i16>)>::new_tensors(&shapes);

        let got = [a.shape(), b.shape(), c.shape()];
        assert_eq!(got, shapes);
    }

    #[test]
    #[should_panic = "Expected a shape with 3 elements, found [Shape { element_type: F32, dimensions: [1] }]"]
    fn incorrect_shape_list_length() {
        let shapes = [Shape::new(ElementType::F32, [1_usize].as_ref())];

        let _ = <(Tensor<f32>, Tensor<u8>, Tensor<i16>)>::new_tensors(&shapes);
    }

    #[test]
    #[should_panic = "Incorrect element type"]
    fn incorrect_type_in_shape_list() {
        let shapes = [Shape::new(ElementType::F32, [1_usize].as_ref())];

        let _ = <Tensor<i16>>::new_tensors(&shapes);
    }
}
