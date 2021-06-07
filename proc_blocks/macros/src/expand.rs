use proc_macro2::{Ident, TokenStream, Span};
use quote::{ToTokens, quote};
use crate::{
    descriptor::{
        ProcBlockDescriptor, ParameterDescriptor, TransformDescriptor,
    },
    analysis::Analysis,
};
use syn::{Path, LitByteStr};

pub(crate) fn implement_proc_block_trait(analysis: Analysis) -> TokenStream {
    let Analysis {
        name,
        exports,
        descriptor,
    } = analysis;
    let custom_section = expand_custom_section(&name, &descriptor);
    let descriptor = expand_descriptor(&name, &exports, &descriptor);

    quote! {
        impl ProcBlock for #name {
            const DESCRIPTOR: ProcBlockDescriptor<'static> = #descriptor;
        }

        #custom_section
    }
}

fn expand_custom_section(
    name: &Ident,
    descriptor: &ProcBlockDescriptor<'_>,
) -> TokenStream {
    let name = format!("PROC_BLOCK_DESCRIPTOR_FOR_{}", name).to_uppercase();
    let name = Ident::new(&name, Span::call_site());

    let serialized = serde_json::to_string(&descriptor)
        .expect("Unable to serialize the descriptor as JSON");
    let len = serialized.len();
    let serialized = LitByteStr::new(serialized.as_bytes(), Span::call_site());

    quote! {
        #[link_name = ".rune_proc_block"]
        pub static #name: [u8; #len] = *#serialized;
    }
}

fn expand_descriptor(
    name: &Ident,
    exports: &Path,
    descriptor: &ProcBlockDescriptor<'_>,
) -> TokenStream {
    let name = name.to_string();
    let ProcBlockDescriptor {
        type_name: _,
        description,
        available_transforms,
        parameters,
    } = descriptor;

    quote! {
        ProcBlockDescriptor {
            type_name: #exports::Cow::Borrowed(concat!(module_path!(), "::", #name)),
            description: #exports::Cow::Borrowed(#description),
            parameters: #exports::Cow::Borrowed(&[
                #( #parameters ),*
            ]),
            available_transforms: #exports::Cow::Borrowed(&[
                #( #available_transforms ),*
            ]),
        }
    }
}

impl<'a> ToTokens for ParameterDescriptor<'a> {
    fn to_tokens(&self, _tokens: &mut TokenStream) { todo!("{:?}", self) }
}

impl<'a> ToTokens for TransformDescriptor<'a> {
    fn to_tokens(&self, _tokens: &mut TokenStream) { todo!("{:?}", self) }
}
