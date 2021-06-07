extern crate proc_macro;
extern crate alloc;

mod analysis;
mod expand;

// This is a bit hacky, but by using a #[path] attribute we can share the
// descriptor definitions without actually needing to move them to a common
// dependency.
#[path = "../../pb-core/src/descriptor.rs"]
mod descriptor;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(ProcBlock, attributes(transform))]
pub fn proc_block(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    analysis::parse(&input)
        .map(|analysis| {
            let tokens = crate::expand::implement_proc_block_trait(analysis);
            TokenStream::from(tokens)
        })
        .unwrap_or_else(|e| TokenStream::from(e.into_compile_error()))
}
