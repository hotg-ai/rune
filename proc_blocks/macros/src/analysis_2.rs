use proc_macro2::{Ident, Span};
use runic_types::reflect::Type;
use quote::{ToTokens, quote};
use syn::{
    Attribute, DeriveInput, Error, ExprLit, Lit, LitStr, Path, Token,
    TypeArray, TypePath, TypeReference,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};

use crate::{
    descriptor::{
        ProcBlockDescriptor, ParameterDescriptor, TransformDescriptor,
        Dimension, Dimensions, TensorDescriptor,
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
    let (setters, parameters, setter_assertions) = analyse_properties(&input)?;
    let assertions = Assertions {
        set: setter_assertions,
        transform: transform_assertions,
    };
    let descriptor = ProcBlockDescriptor {
        type_name: type_name.to_string().into(),
        description: description.into(),
        parameters: parameters.into(),
        available_transforms: available_transforms.into(),
    };
    let custom_section = make_custom_section(&input.ident, &descriptor)?;

    Ok(DeriveOutput {
        trait_impl: ProcBlockImpl {
            exports,
            type_name,
            descriptor,
        },
        custom_section,
        setters,
        assertions,
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
        let input = to_rust_tensor(&input.element_type);
        let output = to_rust_tensor(&output.element_type);

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

fn to_rust_tensor(ty: &Type) -> Option<syn::Type> {
    let rust_name = ty.rust_name()?;
    let rust_type = Ident::new(rust_name, Span::call_site());
    let tokens = quote!(Tensor<#rust_type>);

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
) -> Result<(Setters, Vec<ParameterDescriptor<'static>>, SetterAssertions), Error>
{
    let data = match &input.data {
        syn::Data::Struct(s) => s,
        _ => return Err(Error::new(input.span(), "")),
    };

    let mut setters = Vec::new();
    let mut descriptors = Vec::new();
    let mut assertions = Vec::new();

    for field in &data.fields {
        if let Some(parsed) = parse_parameter(field)? {
            let ParsedField {
                property,
                descriptor,
                property_type,
                possible_types,
            } = parsed;

            descriptors.push(descriptor);

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

    Ok((Setters(setters), descriptors, SetterAssertions(assertions)))
}

struct ParsedField {
    property: Ident,
    descriptor: ParameterDescriptor<'static>,
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

    let description = doc_comments(&field.attrs)?;
    let parameter_type = syn_type_to_reflect(&field.ty)?;

    let descriptor = ParameterDescriptor {
        name: property.to_string().into(),
        description: description.into(),
        parameter_type,
    };

    let property_type = field.ty.clone();
    // TODO: Let people specify which other types can be used to set this
    // propert (e.g. because there is a From impl)
    let possible_types = vec![field.ty.clone()];

    Ok(Some(ParsedField {
        descriptor,
        property,
        property_type,
        possible_types,
    }))
}

fn syn_type_to_reflect(t: &syn::Type) -> Result<Type, Error> {
    match t {
        syn::Type::Array(_) => unimplemented!("Array Parameters"),
        syn::Type::Reference(TypeReference { ref elem, .. }) => {
            if let syn::Type::Path(TypePath {
                qself: None,
                ref path,
            }) = **elem
            {
                if let Some(ident) = path.get_ident() {
                    if ident == "str" {
                        return Ok(Type::String);
                    }
                }
            }
        },
        syn::Type::Path(TypePath {
            qself: None,
            ref path,
        }) => {
            if let Some(ident) = path.get_ident() {
                let single_word = ident.to_string();

                match Type::from_rust_name(&single_word) {
                    Some(t) => return Ok(t),
                    None => todo!("Handle \"{}\"", path.to_token_stream()),
                }
            }
        },
        syn::Type::Slice(_) => unimplemented!("Slice Parameters"),
        _ => {},
    }

    return Err(Error::new(
        t.span(),
        "Unable to encode this as a parameter type",
    ));
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
    let default = syn::parse_str("rune_pb_core::internal")
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
            struct Proc {}
        };
        let input: DeriveInput = syn::parse2(tokens).unwrap();
        let expected_description = "One-line summary.\n\nDetailed description.";
        let expected_transforms = &[TransformDescriptor {
            input: TensorDescriptor {
                element_type: Type::f32,
                dimensions: vec![Dimension::Value(1)].into(),
            },
            output: TensorDescriptor {
                element_type: Type::u8,
                dimensions: vec![Dimension::Any; 3].into(),
            },
        }];
        let exports: Path = syn::parse_str("rune_pb_core::internal").unwrap();
        let expected_assertions = TransformAssertions {
            proc_block_type: syn::parse_str("Proc").unwrap(),
            exports: exports.clone(),
            assertions: vec![TransformAssertion {
                input: syn::parse_str("Tensor<f32>").unwrap(),
                output: syn::parse_str("Tensor<u8>").unwrap(),
            }],
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
        let expected_setters = Setters(vec![Setter {
            property: syn::parse_str("first").unwrap(),
            property_type: syn::parse_str("u32").unwrap(),
        }]);
        let expected_parameters = vec![ParameterDescriptor {
            name: "first".into(),
            description: "The first item.".into(),
            parameter_type: Type::u32,
        }];
        let expected_assertions = SetterAssertions(vec![SetterAssertion {
            proc_block_type: syn::parse_str("Proc").unwrap(),
            property: syn::parse_str("first").unwrap(),
            setter_argument: syn::parse_str("u32").unwrap(),
        }]);

        let (setters, parameters, assertions) =
            analyse_properties(&input).unwrap();

        assert_eq!(setters, expected_setters);
        assert_eq!(parameters, expected_parameters);
        assert_eq!(assertions, expected_assertions);
    }
}

mod tensor_parse_tests {
    use super::*;

    macro_rules! parse_tensor_type {
        ($name:ident, $ty:tt => $should_be:expr) => {
            #[test]
            fn $name() {
                let tokens = quote!($ty);

                let got: TensorDescriptor<'static> =
                    syn::parse2(tokens).unwrap();

                assert_eq!(got, $should_be);
            }
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
}
