use proc_macro2::{Ident, Span};
use runic_types::reflect::Type;
use quote::quote;
use syn::{
    Attribute, DeriveInput, Error, ExprLit, Lit, LitStr, Path, Token,
    TypeArray, TypePath, TypeReference,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};

use crate::{
    descriptor::{
        ProcBlockDescriptor, TransformDescriptor, Dimension, Dimensions,
        TensorDescriptor,
    },
    types::{
        Assertions, CustomSection, DeriveOutput, ProcBlockImpl, Setter,
        SetterAssertion, SetterAssertions, Setters, TransformAssertion,
        TransformAssertions,
    },
};

pub(crate) fn analyse(input: &DeriveInput) -> Result<DeriveOutput, Error> {
    let type_name = input.ident.clone();
    let exports = export_path(&input.attrs)?;

    let (description, available_transforms, transform_assertions) =
        analyse_struct_attributes(&input.ident, &exports, &input.attrs)?;

    let (setters, setter_assertions) = analyse_properties(&input)?;

    let descriptor = ProcBlockDescriptor {
        type_name: type_name.to_string().into(),
        description: description.into(),
        available_transforms: available_transforms.into(),
    };

    Ok(DeriveOutput {
        setters,
        custom_section: make_custom_section(&input.ident, &descriptor)?,
        trait_impl: ProcBlockImpl {
            exports,
            type_name,
            descriptor,
        },
        assertions: Assertions {
            set: setter_assertions,
            transform: transform_assertions,
        },
    })
}

fn analyse_struct_attributes(
    proc_block_type: &Ident,
    exports: &Path,
    attrs: &[syn::Attribute],
) -> Result<
    (
        String,
        Vec<TransformDescriptor<'static>>,
        TransformAssertions,
    ),
    Error,
> {
    let description = doc_comments(attrs)?;
    let transforms = transforms(attrs)?;

    let mut assertions = Vec::new();

    for transform in &transforms {
        let TransformDescriptor { input, output } = transform;
        let input = to_rust_tensor(exports, &input.element_type);
        let output = to_rust_tensor(exports, &output.element_type);

        if let Some((input, output)) = input.zip(output) {
            let assert = TransformAssertion { input, output };
            assertions.push(assert);
        }
    }

    Ok((
        description,
        transforms,
        TransformAssertions {
            proc_block_type: proc_block_type.clone(),
            exports: exports.clone(),
            assertions,
        },
    ))
}

fn to_rust_tensor(exports: &Path, ty: &Type) -> Option<syn::Type> {
    // Note: the rust type name for a string isn't defined unambiguously (you
    // could use str, &str, &'static str, String, etc.) so we handle it
    // specially.
    let tokens = match ty {
        Type::String => quote!(#exports::Tensor<&'static str>),
        everything_else => {
            let rust_type =
                Ident::new(everything_else.rust_name()?, Span::call_site());
            quote!(#exports::Tensor<#rust_type>)
        },
    };

    Some(
        syn::parse2(tokens)
            .expect("We should always be able to parse a tensor"),
    )
}

fn transforms(
    attrs: &[Attribute],
) -> Result<Vec<TransformDescriptor<'static>>, Error> {
    let mut transforms = Vec::new();

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

            let element_type = known_type_from_syn_type(&elem)?;
            let dimensions = match &len {
                syn::Expr::Lit(ExprLit {
                    lit: Lit::Int(int), ..
                }) => {
                    let rank: usize = int.base10_parse()?;
                    Dimensions::from(vec![Dimension::Any; rank])
                },
                syn::Expr::Verbatim(v) => {
                    let _: Token![_] = syn::parse2(v.clone())?;
                    Dimensions::Arbitrary
                },
                _ => {
                    return Err(Error::new(
                        len.span(),
                        "Expected a constant length",
                    ))
                },
            };

            Ok(TensorDescriptor {
                element_type,
                dimensions,
            })
        } else {
            let element = input.parse()?;
            Ok(TensorDescriptor {
                element_type: known_type_from_syn_type(&element)?,
                dimensions: vec![Dimension::Value(1)].into(),
            })
        }
    }
}

fn known_type_from_syn_type(ty: &syn::Type) -> Result<Type, Error> {
    match ty {
        syn::Type::Path(p) => {
            if let Some(id) = p.path.get_ident() {
                let name = id.to_string();

                return Type::from_rust_name(&name)
                    .ok_or_else(|| Error::new(ty.span(), "Unknown type"));
            }
        },
        syn::Type::Reference(TypeReference { elem, .. }) => match &**elem {
            syn::Type::Path(TypePath { path, .. }) if path.is_ident("str") => {
                return Ok(Type::String);
            },
            _ => {},
        },
        _ => {},
    }

    Err(Error::new(ty.span(), "Unknown tensor element type"))
}

fn doc_comments(attrs: &[Attribute]) -> Result<String, Error> {
    let mut docs: Vec<String> = Vec::new();

    for attr in attrs {
        if let Some(name) = attr.path.get_ident() {
            if name == "doc" {
                let DocAttr(doc) = syn::parse2(attr.tokens.clone())?;
                docs.push(doc);
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

fn remove_leading_space(s: &str) -> &str {
    if s.starts_with(" ") {
        &s[1..]
    } else {
        s
    }
}

fn analyse_properties(
    input: &DeriveInput,
) -> Result<(Setters, SetterAssertions), Error> {
    let data = match &input.data {
        syn::Data::Struct(s) => s,
        _ => return Err(Error::new(input.span(), "")),
    };

    let mut setters = Vec::new();
    let mut assertions = Vec::new();

    for field in &data.fields {
        if let Some(parsed) = parse_parameter(field)? {
            let ParsedField {
                property,
                property_type,
                possible_types,
            } = parsed;

            let new_assertions =
                possible_types.into_iter().map(|ty| SetterAssertion {
                    proc_block_type: input.ident.clone(),
                    property: property.clone(),
                    setter_argument: ty,
                });
            assertions.extend(new_assertions);

            setters.push(Setter {
                property,
                property_type,
            });
        }
    }

    let type_name = input.ident.clone();

    Ok((Setters { type_name, setters }, SetterAssertions(assertions)))
}

struct ParsedField {
    property: Ident,
    property_type: syn::Type,
    possible_types: Vec<syn::Type>,
}

fn parse_parameter(field: &syn::Field) -> Result<Option<ParsedField>, Error> {
    let attrs = field_attributes(&field.attrs)?;

    let is_skipped =
        attrs.iter().any(|f| matches!(f, &FieldAttribute::Skipped));
    if is_skipped {
        return Ok(None);
    }

    let property = match &field.ident {
        Some(id) => id.clone(),
        None => {
            return Err(Error::new(field.span(), "All fields must be named"))
        },
    };

    let property_type = field.ty.clone();
    // TODO: Let people specify which other types can be used to set this
    // propert (e.g. because there is a From impl)
    let possible_types = vec![field.ty.clone()];

    Ok(Some(ParsedField {
        property,
        property_type,
        possible_types,
    }))
}

fn field_attributes(attrs: &[Attribute]) -> Result<Vec<FieldAttribute>, Error> {
    let mut pairs = Vec::new();

    for attr in attrs {
        if attr.path.is_ident("proc_block") {
            let NameValuePairs(got) = attr.parse_args()?;
            pairs.extend(got);
        }
    }

    Ok(pairs)
}

enum FieldAttribute {
    Skipped,
}

impl Parse for FieldAttribute {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let ident: Ident = input.parse()?;

        if input.peek(Token![=]) {
            todo!("Handle key=value attributes: {:?}", input);
        }

        if ident == "skip" {
            return Ok(FieldAttribute::Skipped);
        } else {
            todo!("Handle #[proc_block({})]", ident)
        }
    }
}

struct NameValuePairs(Vec<FieldAttribute>);

impl Parse for NameValuePairs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pairs: Punctuated<FieldAttribute, Token![,]> =
            Punctuated::parse_terminated(input)?;

        Ok(NameValuePairs(pairs.into_iter().collect()))
    }
}

fn make_custom_section(
    proc_block_type: &Ident,
    descriptor: &ProcBlockDescriptor<'_>,
) -> Result<CustomSection, Error> {
    let payload =
        serde_json::to_vec(descriptor).expect("Serializing should never fail");

    Ok(CustomSection {
        type_name: proc_block_type.clone(),
        payload,
    })
}

fn export_path(_attrs: &[syn::Attribute]) -> Result<Path, Error> {
    let default = syn::parse_str("rune_proc_blocks::internal")
        .expect("Hard-coded values should always parse");

    Ok(default)
}

#[cfg(test)]
mod tests {
    use crate::types::{Setter, SetterAssertion, TransformAssertion};
    use super::*;

    #[test]
    fn struct_attributes() {
        let tokens = quote! {
            /// One-line summary.
            ///
            /// Detailed description.
            #[derive(ProcBlock)]
            #[transform(input = f32, output = [u8; 3])]
            #[transform(input = f32, output = [u8; 2])]
            struct Proc {}
        };
        let input: DeriveInput = syn::parse2(tokens).unwrap();
        let expected_description = "One-line summary.\n\nDetailed description.";
        let expected_transforms = &[
            TransformDescriptor {
                input: TensorDescriptor {
                    element_type: Type::f32,
                    dimensions: vec![Dimension::Value(1)].into(),
                },
                output: TensorDescriptor {
                    element_type: Type::u8,
                    dimensions: vec![Dimension::Any; 3].into(),
                },
            },
            TransformDescriptor {
                input: TensorDescriptor {
                    element_type: Type::f32,
                    dimensions: vec![Dimension::Value(1)].into(),
                },
                output: TensorDescriptor {
                    element_type: Type::u8,
                    dimensions: vec![Dimension::Any; 2].into(),
                },
            },
        ];
        let exports: Path =
            syn::parse_str("rune_proc_blocks::internal").unwrap();
        let expected_assertions = TransformAssertions {
            proc_block_type: syn::parse_str("Proc").unwrap(),
            exports: exports.clone(),
            assertions: vec![
                TransformAssertion {
                    input: syn::parse_str(
                        "rune_proc_blocks::internal::Tensor<f32>",
                    )
                    .unwrap(),
                    output: syn::parse_str(
                        "rune_proc_blocks::internal::Tensor<u8>",
                    )
                    .unwrap(),
                },
                TransformAssertion {
                    input: syn::parse_str(
                        "rune_proc_blocks::internal::Tensor<f32>",
                    )
                    .unwrap(),
                    output: syn::parse_str(
                        "rune_proc_blocks::internal::Tensor<u8>",
                    )
                    .unwrap(),
                },
            ],
        };

        let (description, available_transforms, transform_assertions) =
            analyse_struct_attributes(&input.ident, &exports, &input.attrs)
                .unwrap();

        assert_eq!(description, expected_description);
        assert_eq!(available_transforms, expected_transforms);
        assert_eq!(transform_assertions, expected_assertions);
    }

    #[test]
    fn properties() {
        let tokens = quote! {
            struct Proc {
                /// The first item.
                first: u32,
                #[proc_block(skip)]
                second: Vec<String>,
            }
        };
        let input: DeriveInput = syn::parse2(tokens).unwrap();
        let expected_setters = Setters {
            type_name: syn::parse_str("Proc").unwrap(),
            setters: vec![Setter {
                property: syn::parse_str("first").unwrap(),
                property_type: syn::parse_str("u32").unwrap(),
            }],
        };
        let expected_assertions = SetterAssertions(vec![SetterAssertion {
            proc_block_type: syn::parse_str("Proc").unwrap(),
            property: syn::parse_str("first").unwrap(),
            setter_argument: syn::parse_str("u32").unwrap(),
        }]);

        let (setters, assertions) = analyse_properties(&input).unwrap();

        assert_eq!(setters, expected_setters);
        assert_eq!(assertions, expected_assertions);
    }

    #[test]
    fn transform_assertion_automatically_wraps_in_tensor() {
        let inputs = vec![
            (Type::u8, quote!(exports::Tensor<u8>)),
            (Type::String, quote!(exports::Tensor<&'static str>)),
        ];
        let exports: Path = syn::parse_str("exports").unwrap();

        for (input, should_be) in inputs {
            let should_be: syn::Type = syn::parse2(should_be).unwrap();
            let got = to_rust_tensor(&exports, &input).unwrap();
            assert_eq!(got, should_be);
        }
    }
}

#[cfg(test)]
mod type_tests {
    use super::*;

    macro_rules! parse_tensor_type {
        ($name:ident, $($rest:tt)*) => {
            parse_tensor_type!(@collect $name, [] $($rest)*);
        };
        (@collect $name:ident, [$($ty:tt)*] => $should_be:expr) => {
            #[test]
            fn $name() {
                let tokens = quote!($($ty)*);

                let got: TensorDescriptor<'static> = match syn::parse2(tokens.clone()) {
                    Ok(d) => d,
                    Err(e) => panic!("Unable to parse \"{}\": {}", tokens.to_string(), e),
                };

                assert_eq!(got, $should_be);
            }
        };
        (@collect $name:ident, [$($ty:tt)*] $next:tt $($rest:tt)*) => {
            parse_tensor_type!(@collect $name, [$($ty)* $next] $($rest)*);
        };
    }

    parse_tensor_type!(one_dimension_tensor, i16 => TensorDescriptor {
       element_type: Type::i16,
       dimensions: vec![Dimension::Value(1)].into(),
    });
    parse_tensor_type!(parse_f32_rank_3, [f32; 3] => TensorDescriptor {
       element_type: Type::f32,
       dimensions: vec![Dimension::Any, Dimension::Any, Dimension::Any].into(),
    });
    parse_tensor_type!(parse_arbitrary_length, [u8; _] => TensorDescriptor {
       element_type: Type::u8,
       dimensions: Dimensions::Arbitrary,
    });
    parse_tensor_type!(parse_str_type, str => TensorDescriptor {
       element_type: Type::String,
       dimensions: vec![Dimension::Value(1)].into(),
    });
}
