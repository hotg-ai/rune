extern crate proc_macro;
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod analysis;
mod expand;
mod types;

#[allow(dead_code)]
mod descriptor;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

/// Derive the `ProcBlock` trait for a particular type.
///
/// # Struct Attributes
///
/// Use the `#[transform(...)]` attribute to specify which transformations are
/// valid for a particular proc block. A plain primitive will be treated as a
/// 1D `Tensor<T>`.
///
/// ```rust
/// use hotg_rune_proc_blocks::{ProcBlock, Transform};
/// use hotg_rune_core::Tensor;
///
/// #[derive(Default, hotg_rune_proc_block_macros::ProcBlock)]
/// #[transform(input = f32, output = f32)]
/// struct Foo { }
///
/// impl Transform<Tensor<f32>> for Foo {
///     type Output = Tensor<f32>;
///     fn transform(&mut self, _input: Tensor<f32>) -> Self::Output { todo!() }
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
/// #[transform(input = f32, output = f32)]
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
/// use hotg_rune_proc_blocks::{ProcBlock, Transform};
/// use hotg_rune_core::Tensor;
///
/// #[derive(Default, hotg_rune_proc_block_macros::ProcBlock)]
/// #[transform(input = [f32; _], output = [u8; 1920])]
/// #[transform(input = str, output = [i16; 2])]
/// struct Foo { }
///
/// impl Transform<Tensor<f32>> for Foo {
///     type Output = Tensor<u8>;
///     fn transform(&mut self, _input: Tensor<f32>) -> Self::Output { todo!() }
/// }
/// impl Transform<Tensor<&'static str>> for Foo {
///     type Output = Tensor<i16>;
///     fn transform(&mut self, _input: Tensor<&'static str>) -> Self::Output { todo!() }
/// }
/// ```
///
/// ## Field Attributes
///
/// By default, all fields in a proc block struct will be registered as
/// "properties" and will get some generated setters.
///
/// ```rust
/// use hotg_rune_proc_blocks::ProcBlock;
///
/// #[derive(Default, hotg_rune_proc_block_macros::ProcBlock)]
/// struct Foo {
///     first: &'static str,
///     second: u32,
/// }
///
/// let descriptor = Foo::DESCRIPTOR;
///
/// let mut foo = Foo::default();
///
/// foo.set_first("Hello, World!").set_second(42_u32);
/// assert_eq!(foo.first, "Hello, World!");
/// assert_eq!(foo.second, 42);
/// ```
///
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
#[proc_macro_derive(ProcBlock, attributes(transform, proc_block))]
pub fn proc_block(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let tokens = analysis::analyse(&input)
        .map(ToTokens::into_token_stream)
        .unwrap_or_else(|e| e.into_compile_error().into());

    TokenStream::from(tokens)
}
