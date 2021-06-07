#![no_std]

extern crate alloc;

mod descriptor;

pub use runic_types;
pub use rune_pb_macros::ProcBlock;
pub use descriptor::{
    Dimension, TensorDescriptor, ProcBlockDescriptor, ParameterDescriptor,
    TransformDescriptor,
};

/// Process some data, transforming it from one form to another.
pub trait Transform<Input>: ProcBlock {
    type Output;

    fn transform(&mut self, input: Input) -> Self::Output;
}

/// The base trait that all proc blocks must implement.
///
/// This trait must be implemented using the [`rune_pb_macros::ProcBlock`]
/// custom derive.
pub trait ProcBlock: Default + 'static {
    /// A description of the proc block.
    const DESCRIPTOR: ProcBlockDescriptor<'static>;
}

#[doc(hidden)]
pub mod internal {
    pub use crate::{ProcBlock, Transform, descriptor::*};
    pub use alloc::borrow::Cow;
}
