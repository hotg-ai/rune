//! Common abstractions used by all Rune proc blocks.
//!
//! # Feature Flags
//!
//! This crate has the following cargo feature flags:
//!
//! - `derive` - re-export the `#[derive(ProcBlock)]` from the
//!   `hotg-rune-proc-block-macros` crate

#![no_std]
#![cfg_attr(feature = "unstable_doc_cfg", feature(doc_cfg))]

extern crate alloc;

mod descriptor;

pub use descriptor::*;
pub use hotg_rune_core::Tensor;
#[cfg(feature = "derive")]
pub use hotg_rune_proc_block_macros::ProcBlock;

/// This crate's version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Process some data, transforming it from one form to another.
pub trait Transform<Input>: ProcBlock {
    type Output;

    fn transform(&mut self, input: Input) -> Self::Output;
}

/// The base trait that all proc blocks must implement.
///
/// This trait shouldn't be implemented manually, instead you should prefer the
/// `#[derive(ProcBlock)]` custom derive.
///
/// # Struct Attributes
///
/// Use the `#[transform(...)]` attribute to specify which transformations are
/// valid for a particular proc block. A plain primitive will be treated as a
/// 1D `Tensor<T>`.
///
/// ```rust
/// use hotg_rune_core::Tensor;
/// use hotg_rune_proc_blocks::{ProcBlock, Transform};
///
/// #[derive(Default, hotg_rune_proc_block_macros::ProcBlock)]
/// #[transform(inputs = f32, outputs = f32)]
/// struct Foo {}
///
/// impl Transform<Tensor<f32>> for Foo {
///     type Output = Tensor<f32>;
///
///     fn transform(&mut self, _input: Tensor<f32>) -> Self::Output {
///         unimplemented!()
///     }
/// }
/// ```
///
/// Forgetting to write the correct `Transform` implementation will fail to
/// compile.
///
/// ```rust,compile_fail
/// use hotg_rune_proc_blocks::{ProcBlock, Transform};
/// use hotg_rune_core::Tensor;
///
/// #[derive(Default, hotg_rune_proc_block_macros::ProcBlock)]  // Error: the trait bound `Foo: hotg_rune_proc_blocks::Transform<Tensor<f32>>` is not satisfied
/// #[transform(inputs = f32, outputs = f32)]
/// struct Foo { }
///
/// // impl Transform<Tensor<f32>> for Foo {
/// //     type Output = Tensor<f32>;
/// //     fn transform(&mut self, _input: Tensor<f32>) -> Self::Output { todo!() }
/// // }
/// ```
///
/// You can also specify the number of dimensions in an input or output. Using
/// `_` indicates the transformation works with *any* number of dimensions.
///
/// ```rust
/// use std::borrow::Cow;
///
/// use hotg_rune_core::Tensor;
/// use hotg_rune_proc_blocks::{ProcBlock, Transform};
///
/// #[derive(Default, hotg_rune_proc_block_macros::ProcBlock)]
/// #[transform(inputs = [f32; _], outputs = [u8; 1920])]
/// #[transform(inputs = utf8, outputs = [i16; 2])]
/// struct Foo {}
///
/// impl Transform<Tensor<f32>> for Foo {
///     type Output = Tensor<u8>;
///
///     fn transform(&mut self, _input: Tensor<f32>) -> Self::Output {
///         unimplemented!()
///     }
/// }
/// impl Transform<Tensor<Cow<'static, str>>> for Foo {
///     type Output = Tensor<i16>;
///
///     fn transform(
///         &mut self,
///         _input: Tensor<Cow<'static, str>>,
///     ) -> Self::Output {
///         unimplemented!()
///     }
/// }
/// ```
///
/// ## Field Attributes
///
/// By default, all fields in a proc block struct will be registered as
/// "properties" and will get some generated setters. These setters accept a
/// string and use [`core::str::FromStr`] to parse it into the correct type.
///
/// ```rust
/// use hotg_rune_proc_blocks::ProcBlock;
///
/// #[derive(Default, hotg_rune_proc_block_macros::ProcBlock)]
/// struct Foo {
///     property: f64,
/// }
///
/// let descriptor = Foo::DESCRIPTOR;
///
/// let mut foo = Foo::default();
///
/// foo.set_property("42").unwrap();
/// assert_eq!(foo.property, 42.0);
/// ```
///
/// A parameter can opt-out of this with the `#[proc_block(skip)]` attribute.
///
/// ```rust,compile_fail
/// use hotg_rune_proc_blocks::ProcBlock;
///
/// #[derive(Default, hotg_rune_proc_block_macros::ProcBlock)]
/// struct Foo {
///     #[proc_block(skip)]
///     skip_me: String,
///     include_me: u32,
/// }
///
/// let mut foo = Foo::default();
///
/// foo.set_skip_me("..."); // Error: no method named `set_skip_me` found for struct `Foo` in the current scope
/// ```
pub trait ProcBlock: Default + 'static {
    /// A description of the proc block.
    const DESCRIPTOR: ProcBlockDescriptor<'static>;
}

/// An internal module used by the `hotg_rune_proc_block_macros` crate
/// so it has access to all the types it will need.
#[doc(hidden)]
pub mod internal {
    pub use alloc::borrow::Cow;

    pub use hotg_rune_core::{ElementType, Tensor};

    pub use crate::{descriptor::*, ProcBlock, Transform};
}
