#![no_std]

extern crate alloc;

mod descriptor;

pub use rune_core::{HasOutputs, Tensor};
pub use rune_proc_block_macros::ProcBlock;
pub use descriptor::*;

/// Process some data, transforming it from one form to another.
pub trait Transform<Input>: ProcBlock {
    type Output;

    fn transform(&mut self, input: Input) -> Self::Output;
}

/// The base trait that all proc blocks must implement.
///
/// This trait must be implemented using the
/// [`rune_proc_block_macros::ProcBlock`] custom derive.
pub trait ProcBlock: Default + 'static {
    /// A description of the proc block.
    const DESCRIPTOR: ProcBlockDescriptor<'static>;
}

/// An internal module used give the [`rune_proc_block_macros`] crate access to
/// all the types it will need.
#[doc(hidden)]
pub mod internal {
    pub use crate::{ProcBlock, Transform, descriptor::*};
    pub use alloc::borrow::Cow;
    pub use rune_core::{reflect::Type, Tensor};
}
