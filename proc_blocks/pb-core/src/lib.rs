#![no_std]

pub use runic_types::{Tensor, TensorView, TensorViewMut};
pub use rune_pb_macros::ProcBlock;

/// Process some data, transforming it from one form to another.
pub trait Transform<Input>: ProcBlock {
    type Output;

    fn transform(&mut self, input: Input) -> Self::Output;
}

pub trait ProcBlock: Default + 'static {}

/// A really interesting type.
#[derive(ProcBlock, Default, PartialEq)]
#[transform(input = f32[1], output = u8)]
struct Foo {}
