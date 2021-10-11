#[macro_use]
extern crate pretty_assertions;

use hotg_rune_proc_blocks::{
    Dimension, Dimensions, ProcBlock, ProcBlockDescriptor, TensorDescriptor,
    Transform, TransformDescriptor,
};
use hotg_rune_core::{Tensor, ElementType};

/// A dummy proc block.
///
/// Can it handle multiple lines of input?
#[derive(hotg_rune_proc_block_macros::ProcBlock, Default, PartialEq)]
#[transform(inputs = [f32; 3], outputs = u8)]
#[transform(inputs = [u8; _], outputs = [f32; 1])]
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
                inputs: TensorDescriptor {
                    element_type: ElementType::F32,
                    dimensions: vec![Dimension::Any; 3].into(),
                }
                .into(),
                outputs: TensorDescriptor {
                    element_type: ElementType::U8,
                    dimensions: vec![Dimension::Value(1)].into(),
                }
                .into(),
            },
            TransformDescriptor {
                inputs: TensorDescriptor {
                    element_type: ElementType::U8,
                    dimensions: Dimensions::Arbitrary,
                }
                .into(),
                outputs: TensorDescriptor {
                    element_type: ElementType::F32,
                    dimensions: vec![Dimension::Any].into(),
                }
                .into(),
            },
        ]
        .into(),
    };

    let got = <Foo as ProcBlock>::DESCRIPTOR;

    assert_eq!(got, should_be);
}
