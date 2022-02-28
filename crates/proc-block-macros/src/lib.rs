extern crate alloc;
extern crate proc_macro;

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
#[proc_macro_derive(ProcBlock, attributes(transform, proc_block))]
pub fn proc_block(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let tokens = analysis::analyse(&input)
        .map(ToTokens::into_token_stream)
        .unwrap_or_else(|e| e.into_compile_error());

    TokenStream::from(tokens)
}
