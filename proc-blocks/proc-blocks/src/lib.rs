#![no_std]

extern crate alloc;

mod descriptor;

pub use hotg_rune_core::Tensor;
pub use descriptor::*;

/// This crate's version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "derive")]
pub use hotg_rune_proc_block_macros::ProcBlock;

/// Process some data, transforming it from one form to another.
pub trait Transform<Input>: ProcBlock {
    type Output;

    fn transform(&mut self, input: Input) -> Self::Output;
}

/// The base trait that all proc blocks must implement.
///
/// This trait shouldn't be implemented manually, instead you should prefer the
/// `#[derive(ProcBlock)]` custom derive.
pub trait ProcBlock: Default + 'static {
    /// A description of the proc block.
    const DESCRIPTOR: ProcBlockDescriptor<'static>;
}

/// An internal module used by the `hotg_rune_proc_block_macros` crate
/// so it has access to all the types it will need.
#[doc(hidden)]
pub mod internal {
    pub use crate::{ProcBlock, Transform, descriptor::*};
    pub use alloc::borrow::Cow;
    pub use hotg_rune_core::{ElementType, Tensor};
}
