#[macro_use]
extern crate pretty_assertions;

use rune_proc_blocks::{
    Dimension, Dimensions, ProcBlock, ProcBlockDescriptor, TensorDescriptor,
    Transform, TransformDescriptor,
};
use runic_types::{Tensor, reflect::Type};

/// A dummy proc block.
///
/// Can it handle multiple lines of input?
#[derive(rune_proc_block_macros::ProcBlock, Default, PartialEq)]
#[transform(input = [f32; 3], output = u8)]
#[transform(input = [u8; _], output = [f32; 1])]
struct Foo {
    /// Some parameter.
    a: u32,
    #[proc_block(skip)]
    skipped: Vec<String>,
}

impl Transform<Tensor<f32>> for Foo {
    type Output = Tensor<u8>;

    fn transform(&mut self, _: Tensor<f32>) -> Self::Output { unimplemented!() }
}

impl Transform<Tensor<u8>> for Foo {
    type Output = Tensor<f32>;

    fn transform(&mut self, _: Tensor<u8>) -> Self::Output { unimplemented!() }
}

#[test]
fn generate_expected_descriptor() {
    let should_be = ProcBlockDescriptor {
        type_name: "Foo".into(),
        description:
            "A dummy proc block.\n\nCan it handle multiple lines of input?"
                .into(),
        available_transforms: vec![
            TransformDescriptor {
                input: TensorDescriptor {
                    element_type: Type::f32,
                    dimensions: vec![Dimension::Any; 3].into(),
                },
                output: TensorDescriptor {
                    element_type: Type::u8,
                    dimensions: vec![Dimension::Value(1)].into(),
                },
            },
            TransformDescriptor {
                input: TensorDescriptor {
                    element_type: Type::u8,
                    dimensions: Dimensions::Arbitrary,
                },
                output: TensorDescriptor {
                    element_type: Type::f32,
                    dimensions: vec![Dimension::Any].into(),
                },
            },
        ]
        .into(),
    };

    let got = <Foo as ProcBlock>::DESCRIPTOR;

    assert_eq!(got, should_be);
}
