use syn::{
    Attribute, DeriveInput, Error, ExprLit, Ident, Lit, LitStr, Token,
    TypeArray, Path,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};
use runic_types::reflect::Type;
use crate::descriptor::{
    ProcBlockDescriptor, ParameterDescriptor, TransformDescriptor, Dimension,
    TensorDescriptor,
};

#[derive(Debug)]
pub struct Analysis {
    pub descriptor: ProcBlockDescriptor<'static>,
    pub exports: Path,
    pub name: Ident,
}

pub fn parse(input: &DeriveInput) -> Result<Analysis, Error> {
    let description = doc_comments(&input.attrs)?;
    let transforms = transforms(&input.attrs)?;
    let exports = exports(&input.attrs)?;
    let parameters = parse_parameters(input);

    Ok(Analysis {
        exports,
        name: input.ident.clone(),
        descriptor: ProcBlockDescriptor {
            type_name: "<invalid>".into(),
            description: description.into(),
            parameters: parameters.into(),
            available_transforms: transforms.into(),
        },
    })
}

fn exports(_attrs: &Vec<Attribute>) -> Result<Path, Error> {
    // In the future we may want to let people specify a custom path to
    // `rune_pb_core::internal` (e.g. because they imported `rune_pb_core` with
    // a different name).
    // This would be done by scanning for an #[export = "..."] attribute and
    // override the default.

    let exports: Path = syn::parse_str("rune_pb_core::internal").unwrap();

    Ok(exports)
}

fn parse_parameters(_input: &DeriveInput) -> Vec<ParameterDescriptor<'static>> {
    Vec::new()
}

fn transforms(
    attrs: &[Attribute],
) -> Result<Vec<TransformDescriptor<'static>>, Error> {
    let mut transforms = Vec::new();

    return Ok(transforms);

    for attr in attrs {
        if let Some(name) = attr.path.get_ident() {
            if name == "transform" {
                let transform: TransformDescriptor =
                    syn::parse2(attr.tokens.clone())?;
                transforms.push(transform);
            }
        }
    }

    Ok(transforms)
}

fn doc_comments(attrs: &[Attribute]) -> Result<String, Error> {
    let mut docs: Vec<String> = Vec::new();

    for attr in attrs {
        if let Some(name) = attr.path.get_ident() {
            if name == "doc" {
                let doc: DocAttr = syn::parse2(attr.tokens.clone())?;
                docs.push(doc.0);
            }
        }
    }

    Ok(concatenate_doc_comments(&docs))
}

fn concatenate_doc_comments(docs: &[String]) -> String {
    docs.iter().map(|s| remove_leading_space(s)).fold(
        String::new(),
        |mut buffer, line| {
            if !buffer.is_empty() {
                buffer.push('\n');
            }
            buffer.push_str(line);
            buffer
        },
    )
}

fn remove_leading_space(s: &str) -> &str {
    if s.starts_with(" ") {
        &s[1..]
    } else {
        s
    }
}

impl Parse for TransformDescriptor<'static> {
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

        Ok(TransformDescriptor { input, output })
    }
}

impl Parse for TensorDescriptor<'static> {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        if input.peek(syn::token::Bracket) {
            let TypeArray { elem, len, .. } = input.parse()?;

            let element_type = match &*elem {
                syn::Type::Path(p) => match p.path.get_ident() {
                    Some(id) => known_type_from_ident(id)?,
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

            Ok(TensorDescriptor {
                element_type,
                dimensions: vec![Dimension::Any; dimensions].into(),
            })
        } else {
            let element = input.parse()?;
            Ok(TensorDescriptor {
                element_type: known_type_from_ident(&element)?,
                dimensions: vec![Dimension::Value(1)].into(),
            })
        }
    }
}

fn known_type_from_ident(ident: &Ident) -> Result<Type, Error> {
    let s = ident.to_string();

    Type::from_rust_name(&s)
        .ok_or_else(|| Error::new(ident.span(), "Unknown type"))
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
        let should_be = TensorDescriptor {
            element_type: Type::f32,
            dimensions: vec![Dimension::Any; 3].into(),
        };

        let got: TensorDescriptor = syn::parse_str(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn single_element_tensor() {
        let src = "i8";
        let should_be = TensorDescriptor {
            element_type: Type::i8,
            dimensions: vec![Dimension::Value(1)].into(),
        };

        let got: TensorDescriptor = syn::parse_str(src).unwrap();

        assert_eq!(got, should_be);
    }
}
