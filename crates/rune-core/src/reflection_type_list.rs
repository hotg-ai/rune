//! Abstractions for dealing with tuples of tensors.
//!
//! We need to do a lot of generic gymnastics here to be able to go from the
//! world of types (e.g. an input parameter of `(Tensor<f32>, Tensor<u8>)`) to
//! the world of values (`&[Shape]`) so we can pass type information back to
//! the runtime.

use crate::{Shape, Tensor, reflect::ReflectionType};

/// A helper trait which lets us get the shape from a tuple of different
/// tensors.
pub trait ReflectionTypeList<'a> {
    // Note: we could get rid of all these XXXBuffer types if const generics
    // were more fleshed out. You'd just need to define an associated
    // `const RANK: usize` then have each of the methods return something like
    // `[_; Self::Rank]`.
    type ShapeBuffer: AsRef<[Shape<'a>]>;
    type ConstElementPtrBuffer: AsRef<[*const u8]>;

    fn shape_list(&self) -> Self::ShapeBuffer;
    fn element_ptr(&self) -> Self::ConstElementPtrBuffer;
}

pub trait ReflectionTypeListMut<'a> {
    type Tensors;
    type MutElementPtrBuffer: AsMut<[*mut u8]>;

    fn new_tensors(shape: &[Shape<'_>]) -> Self::Tensors;

    fn element_ptr_mut(
        tensors: &mut Self::Tensors,
    ) -> Self::MutElementPtrBuffer;
}

impl<'a, T> ReflectionTypeList<'a> for &'a Tensor<T>
where
    T: ReflectionType + Default,
{
    type ConstElementPtrBuffer = [*const u8; 1];
    type ShapeBuffer = [Shape<'a>; 1];

    fn shape_list(&self) -> Self::ShapeBuffer { [self.shape()] }

    fn element_ptr(&self) -> Self::ConstElementPtrBuffer {
        [self.elements().as_ptr() as *const u8]
    }
}

impl<'a, T> ReflectionTypeListMut<'a> for &'a mut Tensor<T>
where
    T: ReflectionType + Default + Clone,
{
    type MutElementPtrBuffer = [*mut u8; 1];
    type Tensors = (Tensor<T>,);

    fn new_tensors(shape: &[Shape<'_>]) -> Self::Tensors {
        match shape {
            [s] => {
                assert_eq!(
                    s.element_type(),
                    &T::TYPE,
                    "Incorrect element type"
                );
                (Tensor::filled_with(
                    s.dimensions().to_vec(),
                    Default::default,
                ),)
            },
            _ => panic!("Expected a shape with 1 element, found {:?}", shape),
        }
    }

    fn element_ptr_mut(
        tensors: &mut Self::Tensors,
    ) -> Self::MutElementPtrBuffer {
        let (tensor,) = tensors;
        [tensor.make_elements_mut().as_mut_ptr() as *mut u8]
    }
}

/// Count the number of identifiers that were passed in.
macro_rules! count {
    ($first:ident $(, $rest:ident)* $(,)*) => { 1 + count!($($rest),*) };
    () => { 0 };
}

/// Recursively implement [`ReflectionTypeList`] for tuples of any length.
macro_rules! reflection_type_list {
    ($first:ident $(, $dim:ident)* $(,)*) => {
        #[allow(non_snake_case)]
        impl<'a, $first, $($dim),*> ReflectionTypeList<'a> for &'a (Tensor<$first>, $(Tensor<$dim>),*)
        where
            $first: ReflectionType + Default + Clone,
            $( $dim: ReflectionType + Default + Clone ),*
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
        impl<'a, $first, $($dim),*> ReflectionTypeListMut<'a> for &'a mut (Tensor<$first>, $(Tensor<$dim>),*)
        where
            $first: ReflectionType + Default + Clone,
            $( $dim: ReflectionType + Default + Clone ),*
        {
            type Tensors = (Tensor<$first>, $(Tensor<$dim>),*);
            type MutElementPtrBuffer  = [*mut u8; count!($first, $($dim),*)];

            fn new_tensors(shape: &[Shape<'_>]) -> Self::Tensors {
                match shape {
                    [$first, $($dim),*] => {
                        assert_eq!($first.element_type(), &<$first>::TYPE, "Incorrect element type");
                        let $first = Tensor::filled_with(
                            $first.dimensions().to_vec(),
                            Default::default,
                        );
                        $(
                            assert_eq!($dim.element_type(), &<$dim>::TYPE, "Incorrect element type");
                            let $dim = Tensor::filled_with(
                                $dim.dimensions().to_vec(),
                                Default::default,
                            );
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

            fn element_ptr_mut(tensors: &mut Self::Tensors) -> Self::MutElementPtrBuffer {
                let ($first, $($dim),* ) = tensors;

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

reflection_type_list!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(test)]
mod tests {
    use crate::reflect::Type;

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
            Shape::new(Type::f32, [1_usize].as_ref()),
            Shape::new(Type::u8, [3_usize, 256, 256].as_ref()),
            Shape::new(Type::i16, [1920_usize].as_ref()),
        ];

        let (a, b, c) =
            <&mut (Tensor<f32>, Tensor<u8>, Tensor<i16>)>::new_tensors(&shapes);

        let got = [a.shape(), b.shape(), c.shape()];
        assert_eq!(got, shapes);
    }

    #[test]
    #[should_panic = "Expected a shape with 3 elements, found [Shape { element_type: Float { bit_width: 32 }, dimensions: [1] }]"]
    fn incorrect_shape_list_length() {
        let shapes = [Shape::new(Type::f32, [1_usize].as_ref())];

        let _ =
            <&mut (Tensor<f32>, Tensor<u8>, Tensor<i16>)>::new_tensors(&shapes);
    }

    #[test]
    #[should_panic = "Incorrect element type"]
    fn incorrect_type_in_shape_list() {
        let shapes = [Shape::new(Type::f32, [1_usize].as_ref())];

        let _ = <&mut Tensor<i16>>::new_tensors(&shapes);
    }
}
