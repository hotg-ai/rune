use std::str::FromStr;

use hotg_rune_core::ElementType;
use proc_macro2::{Group, Ident};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, DeriveInput, Error, ExprLit, Lit, LitStr, Path, Token,
    TypeArray, TypePath, TypeReference,
};

use crate::{
    descriptor::{
        Dimension, Dimensions, ProcBlockDescriptor, TensorDescriptor,
        TensorDescriptors, TransformDescriptor,
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

    let (setters, setter_assertions) = analyse_properties(input)?;

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
            generics: input.generics.clone(),
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
        let TransformDescriptor { inputs, outputs } = transform;
        let inputs: Vec<_> = inputs
            .iter()
            .map(|t| to_rust_tensor(exports, &t.element_type))
            .collect();
        let outputs: Vec<_> = outputs
            .iter()
            .map(|t| to_rust_tensor(exports, &t.element_type))
            .collect();

        let assert = TransformAssertion { inputs, outputs };
        assertions.push(assert);
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

fn to_rust_tensor(exports: &Path, ty: &ElementType) -> syn::Type {
    let element_type = match ty {
        ElementType::U8 => quote!(u8),
        ElementType::I8 => quote!(i8),
        ElementType::U16 => quote!(u16),
        ElementType::I16 => quote!(i16),
        ElementType::U32 => quote!(u32),
        ElementType::F32 => quote!(f32),
        ElementType::I32 => quote!(i32),
        ElementType::U64 => quote!(u64),
        ElementType::F64 => quote!(f64),
        ElementType::I64 => quote!(i64),
        ElementType::String => quote!(#exports::Cow<'static, str>),
    };

    syn::parse2(quote!(#exports::Tensor<#element_type>))
        .expect("We should always be able to parse a type")
}

fn transforms(
    attrs: &[Attribute],
) -> Result<Vec<TransformDescriptor<'static>>, Error> {
    let mut transforms = Vec::new();

    for attr in attrs {
        if let Some(name) = attr.path.get_ident() {
            if name == "transform" {
                let parenthesized_key_values: Group =
                    syn::parse2(attr.tokens.clone())?;

                let transform = parse_transform_descriptor
                    .parse2(parenthesized_key_values.stream())?;
                transforms.push(transform);
            }
        }
    }

    Ok(transforms)
}

/// Parse the `inputs = ..., outputs = ...` from
/// `#[transform(inputs =..., outputs = ...)]`.
fn parse_transform_descriptor(
    tokens: ParseStream,
) -> Result<TransformDescriptor<'static>, Error> {
    let ident: Ident = tokens.parse()?;
    if ident != "inputs" {
        return Err(Error::new(ident.span(), "Expected \"inputs\""));
    }
    let _: Token![=] = tokens.parse()?;
    let inputs = parse_tensor_descriptors(tokens)?;
    let _: Token![,] = tokens.parse()?;

    let ident: Ident = tokens.parse()?;
    if ident != "outputs" {
        return Err(Error::new(ident.span(), "Expected \"outputs\""));
    }
    let _: Token![=] = tokens.parse()?;
    let outputs = parse_tensor_descriptors(tokens)?;

    Ok(TransformDescriptor { inputs, outputs })
}

/// Parse either a single tensor descriptor (`[f32; 1]`) or multiple tensor
/// descriptors inside parens (`([f32; 1], [u8; 1024])`).
fn parse_tensor_descriptors(
    tokens: ParseStream,
) -> Result<TensorDescriptors<'static>, Error> {
    if tokens.peek(syn::token::Paren) {
        // We've got multiple comma-separated tensor descriptors in a tuple.
        let inner;
        let _ = syn::parenthesized!(inner in tokens);
        let comma_separated_tensors: Punctuated<
            TensorDescriptor<'static>,
            Token![,],
        > = Punctuated::parse_separated_nonempty_with(
            &inner,
            parse_tensor_descriptor,
        )?;

        Ok(comma_separated_tensors.into_iter().collect())
    } else {
        // They've specified just one input
        parse_tensor_descriptor(tokens).map(Into::into)
    }
}

fn parse_tensor_descriptor(
    input: ParseStream,
) -> Result<TensorDescriptor<'static>, Error> {
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

fn known_type_from_syn_type(ty: &syn::Type) -> Result<ElementType, Error> {
    match ty {
        syn::Type::Path(p) => {
            if let Some(id) = p.path.get_ident() {
                let name = id.to_string();

                return ElementType::from_str(&name)
                    .map_err(|e| Error::new(ty.span(), e));
            }
        },
        syn::Type::Reference(TypeReference { elem, .. }) => match &**elem {
            syn::Type::Path(TypePath { path, .. }) if path.is_ident("str") => {
                return Ok(ElementType::String);
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

fn remove_leading_space(s: &str) -> &str { s.strip_prefix(' ').unwrap_or(s) }

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

    Ok((
        Setters {
            type_name,
            setters,
            generics: input.generics.clone(),
        },
        SetterAssertions(assertions),
    ))
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
            Ok(FieldAttribute::Skipped)
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
    let default = syn::parse_str("hotg_rune_proc_blocks::internal")
        .expect("Hard-coded values should always parse");

    Ok(default)
}

#[cfg(test)]
mod tests {
    use syn::Generics;

    use super::*;
    use crate::types::{Setter, SetterAssertion, TransformAssertion};

    #[test]
    fn struct_attributes() {
        let tokens = quote! {
            /// One-line summary.
            ///
            /// Detailed description.
            #[derive(ProcBlock)]
            #[transform(inputs = f32, outputs = [u8; 3])]
            #[transform(inputs = f32, outputs = [u8; 2])]
            struct Proc {}
        };
        let input: DeriveInput = syn::parse2(tokens).unwrap();
        let expected_description = "One-line summary.\n\nDetailed description.";
        let expected_transforms = &[
            TransformDescriptor {
                inputs: TensorDescriptor {
                    element_type: ElementType::F32,
                    dimensions: vec![Dimension::Value(1)].into(),
                }
                .into(),
                outputs: TensorDescriptor {
                    element_type: ElementType::U8,
                    dimensions: vec![Dimension::Any; 3].into(),
                }
                .into(),
            },
            TransformDescriptor {
                inputs: TensorDescriptor {
                    element_type: ElementType::F32,
                    dimensions: vec![Dimension::Value(1)].into(),
                }
                .into(),
                outputs: TensorDescriptor {
                    element_type: ElementType::U8,
                    dimensions: vec![Dimension::Any; 2].into(),
                }
                .into(),
            },
        ];
        let exports: Path =
            syn::parse_str("hotg_rune_proc_blocks::internal").unwrap();
        let expected_assertions = TransformAssertions {
            proc_block_type: syn::parse_str("Proc").unwrap(),
            exports: exports.clone(),
            assertions: vec![
                TransformAssertion {
                    inputs: vec![syn::parse_str(
                        "hotg_rune_proc_blocks::internal::Tensor<f32>",
                    )
                    .unwrap()],
                    outputs: vec![syn::parse_str(
                        "hotg_rune_proc_blocks::internal::Tensor<u8>",
                    )
                    .unwrap()],
                },
                TransformAssertion {
                    inputs: vec![syn::parse_str(
                        "hotg_rune_proc_blocks::internal::Tensor<f32>",
                    )
                    .unwrap()],
                    outputs: vec![syn::parse_str(
                        "hotg_rune_proc_blocks::internal::Tensor<u8>",
                    )
                    .unwrap()],
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
            generics: Generics::default(),
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
            (ElementType::U8, quote!(exports::Tensor<u8>)),
            (
                ElementType::String,
                quote!(exports::Tensor<exports::Cow<'static, str>>),
            ),
        ];
        let exports: Path = syn::parse_str("exports").unwrap();

        for (input, should_be) in inputs {
            let should_be: syn::Type = syn::parse2(should_be).unwrap();
            let got = to_rust_tensor(&exports, &input);
            assert_eq!(got, should_be);
        }
    }
}

#[cfg(test)]
mod type_tests {
    use std::borrow::Cow;

    use super::*;

    macro_rules! parse_tensor_type {
        ($name:ident, $($rest:tt)*) => {
            parse_tensor_type!(@collect $name, [] $($rest)*);
        };
        (@collect $name:ident, [$($ty:tt)*] => $should_be:expr) => {
            #[test]
            fn $name() {
                let tokens = quote!($($ty)*);

                let got: TensorDescriptor<'static> = match parse_tensor_descriptor.parse2(tokens.clone()) {
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
       element_type: ElementType::I16,
       dimensions: vec![Dimension::Value(1)].into(),
    });
    parse_tensor_type!(parse_f32_rank_3, [f32; 3] => TensorDescriptor {
       element_type: ElementType::F32,
       dimensions: vec![Dimension::Any, Dimension::Any, Dimension::Any].into(),
    });
    parse_tensor_type!(parse_arbitrary_length, [u8; _] => TensorDescriptor {
       element_type: ElementType::U8,
       dimensions: Dimensions::Arbitrary,
    });
    parse_tensor_type!(parse_str_type, utf8 => TensorDescriptor {
       element_type: ElementType::String,
       dimensions: vec![Dimension::Value(1)].into(),
    });

    macro_rules! parse_transform_attribute {
        ($name:ident, #[transform( $($attr:tt)* )] => $should_be:expr $(,)?) => {
            #[test]
            fn $name() {
                let tokens = quote!($($attr)*);

                let got: TransformDescriptor<'static> = match parse_transform_descriptor.parse2(tokens.clone()) {
                    Ok(d) => d,
                    Err(e) => panic!("Unable to parse \"{}\": {}", tokens.to_string(), e),
                };

                assert_eq!(got, $should_be);
            }
        };
    }

    parse_transform_attribute!(transform_single_to_single,
        #[transform(inputs = [f32; 1], outputs = [f32; 1])] =>
        TransformDescriptor {
            inputs: TensorDescriptors(Cow::Borrowed(&[
                TensorDescriptor {
                    element_type: ElementType::F32,
                    dimensions: Dimensions::Finite(Cow::Borrowed(&[
                        Dimension::Any,
                    ])),
                },
            ])),
            outputs: TensorDescriptors(Cow::Borrowed(&[
                TensorDescriptor {
                    element_type: ElementType::F32,
                    dimensions: Dimensions::Finite(Cow::Borrowed(&[
                        Dimension::Any,
                    ])),
                },
            ])),
        },
    );

    parse_transform_attribute!(transform_attribute_with_multiple_inputs,
        #[transform(inputs = ([f32; 1], [u8; 2]), outputs = [f32; 1])] =>
        TransformDescriptor {
            inputs: TensorDescriptors(Cow::Borrowed(&[
                TensorDescriptor {
                    element_type: ElementType::F32,
                    dimensions: Dimensions::Finite(Cow::Borrowed(&[
                        Dimension::Any,
                    ])),
                },
                TensorDescriptor {
                    element_type: ElementType::U8,
                    dimensions: Dimensions::Finite(Cow::Borrowed(&[
                        Dimension::Any,
                        Dimension::Any,
                    ])),
                },
            ])),
            outputs: TensorDescriptors(Cow::Borrowed(&[
                TensorDescriptor {
                    element_type: ElementType::F32,
                    dimensions: Dimensions::Finite(Cow::Borrowed(&[
                        Dimension::Any,
                    ])),
                },
            ])),
        },
    );
}
