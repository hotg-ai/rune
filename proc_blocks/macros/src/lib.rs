extern crate proc_macro;

use quote::quote;
use syn::{
    Attribute, DeriveInput, Error, ExprLit, Ident, Lit, LitStr, Token,
    TypeArray,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

#[proc_macro_derive(ProcBlock, attributes(transform))]
pub fn proc_block(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let Attributes { docs, transforms } = match parse_attrs(&input.attrs) {
        Ok(a) => a,
        Err(e) => return e.into_compile_error().into(),
    };

    let tokens = quote! {
        impl ProcBlock for #name {}
    };

    tokens.into()
}

fn parse_attrs(attrs: &[Attribute]) -> Result<Attributes, Error> {
    let mut docs = String::new();
    let mut transforms = Vec::new();

    for attr in attrs {
        if let Some(name) = attr.path.get_ident() {
            if name == "doc" {
                let doc: DocAttr = syn::parse2(attr.tokens.clone())?;
                docs.push_str(&doc.0);
            } else if name == "transform" {
                let transform: Transform = syn::parse2(attr.tokens.clone())?;
                transforms.push(transform);
            }
        }
    }

    Ok(Attributes { docs, transforms })
}

#[derive(Debug, Default, Clone)]
struct Attributes {
    docs: String,
    transforms: Vec<Transform>,
}

#[derive(Debug, Clone)]
struct Transform {
    input: TensorType,
    output: TensorType,
}

impl Parse for Transform {
    fn parse(tokens: ParseStream) -> Result<Self, Error> {
        let inner;
        let _ = syn::parenthesized!(inner in tokens);
        let tokens = inner;

        let ident: Ident = tokens.parse()?;
        if ident != "input" {
            return Err(Error::new(ident.span(), "Expected \"input\""));
        }
        let _: Token![=] = tokens.parse()?;
        let input = tokens.parse()?;
        let _: Token![,] = tokens.parse()?;

        let ident: Ident = tokens.parse()?;
        if ident != "output" {
            return Err(Error::new(ident.span(), "Expected \"output\""));
        }
        let _: Token![=] = tokens.parse()?;
        let output = tokens.parse()?;

        Ok(Transform { input, output })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TensorType {
    element: Ident,
    rank: Option<usize>,
}

impl Parse for TensorType {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        if input.peek(syn::token::Bracket) {
            let TypeArray { elem, len, .. } = input.parse()?;

            let element = match &*elem {
                syn::Type::Path(p) => match p.path.get_ident() {
                    Some(id) => id.clone(),
                    None => {
                        return Err(Error::new(
                            elem.span(),
                            "Expected a type name",
                        ))
                    },
                },
                _ => {
                    return Err(Error::new(elem.span(), "Expected a type name"))
                },
            };
            let dimensions = match len {
                syn::Expr::Lit(ExprLit {
                    lit: Lit::Int(int), ..
                }) => int.base10_parse()?,
                _ => {
                    return Err(Error::new(
                        len.span(),
                        "Expected a constant length",
                    ))
                },
            };

            Ok(TensorType {
                element,
                rank: Some(dimensions),
            })
        } else {
            let element: Ident = input.parse()?;
            Ok(TensorType {
                element,
                rank: None,
            })
        }
    }
}

struct DocAttr(String);

impl Parse for DocAttr {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let span = input.span();
        let _: syn::Token![=] = input.parse()?;
        let docs: LitStr = input.parse()?;

        if input.is_empty() {
            Ok(DocAttr(docs.value()))
        } else {
            Err(Error::new(span, "Malformed doc attribute"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_tensor_type() {
        let src = "[f32; 3]";
        let should_be = TensorType {
            element: syn::parse_str("f32").unwrap(),
            rank: Some(3),
        };

        let got: TensorType = syn::parse_str(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn single_element_tensor() {
        let src = "f32";
        let should_be = TensorType {
            element: syn::parse_str("f32").unwrap(),
            rank: None,
        };

        let got: TensorType = syn::parse_str(src).unwrap();

        assert_eq!(got, should_be);
    }
}
