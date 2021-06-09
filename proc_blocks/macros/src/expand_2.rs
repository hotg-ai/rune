use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{LitByteStr, Path, Type, TypeArray};
use runic_types::reflect::Type as ReflectType;
use crate::{
    descriptor::{
        ProcBlockDescriptor, ParameterDescriptor, TransformDescriptor,
        TensorDescriptor, Dimensions, Dimension,
    },
    types::{
        Assertions, CustomSection, DeriveOutput, ProcBlockImpl, Setter,
        SetterAssertion, SetterAssertions, Setters, TransformAssertion,
        TransformAssertions,
    },
};

impl ToTokens for DeriveOutput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let DeriveOutput {
            trait_impl,
            custom_section,
            setters,
            assertions,
        } = self;

        trait_impl.to_tokens(tokens);
        custom_section.to_tokens(tokens);
        setters.to_tokens(tokens);
        assertions.to_tokens(tokens);
    }
}

impl ToTokens for Setters {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for setter in &self.0 {
            setter.to_tokens(tokens);
        }
    }
}

impl ToTokens for Assertions {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Assertions { set, transform } = self;

        set.to_tokens(tokens);
        transform.to_tokens(tokens);
    }
}

impl ToTokens for SetterAssertions {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for assertion in &self.0 {
            assertion.to_tokens(tokens);
        }
    }
}

impl ToTokens for SetterAssertion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SetterAssertion {
            proc_block_type,
            property,
            setter_argument: property_type,
        } = self;

        let assertion_name = format!("_assert_{}_is_settable", property);
        let assertion_name = Ident::new(&assertion_name, property.span());
        let setter_name = format!("set_{}", property);
        let setter_name = Ident::new(&setter_name, property.span());

        let t = quote! {
            const _: () = {
                fn #assertion_name(proc_block: &mut #proc_block_type, #property: #property_type) -> &mut #proc_block_type {
                    proc_block.#setter_name(#property)
                }
            };
        };
        tokens.extend(t);
    }
}

impl ToTokens for CustomSection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let CustomSection { type_name, payload } = self;
        let len = payload.len();
        let payload = LitByteStr::new(payload, Span::call_site());

        let format = format!("PROC_BLOCK_DESCRIPTOR_FOR_{}", type_name);
        let name = Ident::new(&format, type_name.span());
        let section_name =
            crate::descriptor::ProcBlockDescriptor::CUSTOM_SECTION_NAME;

        let t = quote! {
            #[doc(hidden)]
            #[no_mangle]
            #[link_section = #section_name]
            pub static #name: [u8; #len] = *#payload;
        };
        tokens.extend(t);
    }
}

impl ToTokens for Setter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Setter {
            property,
            property_type,
        } = self;

        let method = format!("set_{}", property);
        let method = Ident::new(&method, property.span());

        let t = quote! {
            pub fn #method(&mut self, #property: impl Into<#property_type>) -> &mut Self {
                self.#property = #property.into();
                self
            }
        };
        tokens.extend(t);
    }
}

impl ToTokens for TransformAssertions {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TransformAssertions {
            proc_block_type,
            exports,
            assertions,
        } = self;

        if assertions.is_empty() {
            return;
        }

        let assertions = assertions.iter()
            .map(|TransformAssertion { input, output }| {
                let input = equivalent_tensor(exports, input);
                let output = equivalent_tensor(exports, output);
                quote! {
                    assert_implements_transform::<#proc_block_type, #input, #output>();
                }
            });

        let t = quote! {
            fn assert_implements_transform<T, Inputs, Outputs>()
            where
                T: #exports::Transform<Inputs, Output=Outputs>
            { }

            fn transform_assertions() {
                #( #assertions )*
            }
        };
        tokens.extend(t);
    }
}

fn equivalent_tensor(exports: &Path, ty: &Type) -> TokenStream {
    match *ty {
        Type::Array(TypeArray { ref elem, .. }) => quote! {
            #exports::Tensor<#elem>
        },
        _ => ty.to_token_stream(),
    }
}

impl ToTokens for ProcBlockImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ProcBlockImpl {
            type_name,
            exports,
            descriptor,
        } = self;

        let descriptor = descriptor_to_tokens(exports, descriptor);

        let t = quote! {
            impl #exports::ProcBlock for #type_name {
                const DESCRIPTOR: #exports::ProcBlockDescriptor<'static> = #descriptor;
            }
        };
        tokens.extend(t);
    }
}

fn descriptor_to_tokens(
    exports: &Path,
    d: &ProcBlockDescriptor<'_>,
) -> TokenStream {
    let ProcBlockDescriptor {
        type_name,
        description,
        parameters,
        available_transforms,
    } = d;

    let parameters = parameters
        .iter()
        .map(|parameter| ParameterProxy { exports, parameter });
    let available_transforms = available_transforms
        .iter()
        .map(|transform| TransformProxy { exports, transform });

    quote! {
        #exports::ProcBlockDescriptor {
            type_name: #exports::Cow::Borrowed(#type_name),
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

#[derive(Debug, Copy, Clone)]
struct ParameterProxy<'a> {
    exports: &'a Path,
    parameter: &'a ParameterDescriptor<'a>,
}

impl<'a> ToTokens for ParameterProxy<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ParameterProxy {
            exports,
            parameter:
                ParameterDescriptor {
                    name,
                    description,
                    parameter_type,
                },
        } = *self;

        let parameter_type = TypeProxy {
            exports,
            ty: parameter_type,
        };

        let t = quote! {
            #exports::ParameterDescriptor {
                name: #exports::Cow::Borrowed(#name),
                description: #exports::Cow::Borrowed(#description),
                parameter_type: #parameter_type,
            }
        };
        tokens.extend(t);
    }
}

#[derive(Debug, Copy, Clone)]
struct TransformProxy<'a> {
    exports: &'a Path,
    transform: &'a TransformDescriptor<'a>,
}

impl<'a> ToTokens for TransformProxy<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TransformProxy {
            exports,
            transform: TransformDescriptor { input, output },
        } = *self;

        let input = TensorProxy {
            exports,
            tensor: input,
        };
        let output = TensorProxy {
            exports,
            tensor: output,
        };

        let t = quote! {
            #exports::TransformDescriptor {
                input: #input,
                output: #output,
            }
        };
        tokens.extend(t);
    }
}

#[derive(Debug, Copy, Clone)]
struct TensorProxy<'a> {
    exports: &'a Path,
    tensor: &'a TensorDescriptor<'a>,
}

impl<'a> ToTokens for TensorProxy<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TensorProxy {
            exports,
            tensor:
                TensorDescriptor {
                    element_type,
                    dimensions,
                },
        } = *self;

        let element_type = TypeProxy {
            exports,
            ty: element_type,
        };
        let dimensions = DimensionsProxy {
            exports,
            dimensions,
        };

        let t = quote! {
            #exports::TensorDescriptor {
                element_type: #element_type,
                dimensions: #dimensions,
            }
        };
        tokens.extend(t);
    }
}

#[derive(Debug, Copy, Clone)]
struct TypeProxy<'a> {
    exports: &'a Path,
    ty: &'a ReflectType,
}

impl<'a> ToTokens for TypeProxy<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TypeProxy { exports, ty } = *self;

        let t = match *ty {
            ReflectType::Integer { signed, bit_width } => {
                quote!(#exports::Type::Integer {
                    signed: #signed,
                    bit_width: #bit_width,
                })
            },
            ReflectType::Float { bit_width } => quote!(#exports::Type::Float {
                bit_width: #bit_width,
            }),
            ReflectType::String => quote!(#exports::Type::Stirng),
            ReflectType::Opaque { ref type_name } => {
                quote!(#exports::Type::Opaque {
                    type_name: $exports::Cow::Borrowed(#type_name),
                })
            },
        };
        tokens.extend(t);
    }
}

#[derive(Debug, Copy, Clone)]
struct DimensionsProxy<'a> {
    exports: &'a Path,
    dimensions: &'a Dimensions<'a>,
}

impl<'a> ToTokens for DimensionsProxy<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let DimensionsProxy {
            exports,
            dimensions,
        } = self;

        let t = match dimensions {
            Dimensions::Arbitrary => quote!(#exports::Dimensions::Arbitrary),
            Dimensions::Finite(fixed) => {
                let dimensions = fixed
                    .iter()
                    .copied()
                    .map(|dimension| DimensionProxy { exports, dimension });

                quote!(#exports::Dimensions::Finite(#exports::Cow::Borrowed(&[
                    #(#dimensions),*
                ])))
            },
        };
        tokens.extend(t);
    }
}

#[derive(Debug, Copy, Clone)]
struct DimensionProxy<'a> {
    exports: &'a Path,
    dimension: Dimension,
}

impl<'a> ToTokens for DimensionProxy<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let DimensionProxy { exports, dimension } = self;

        let t = match dimension {
            Dimension::Any => quote!(#exports::Dimension::Any),
            Dimension::Value(v) => quote!(#exports::Dimension::Value(#v)),
        };
        tokens.extend(t);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        borrow::Cow,
        io::{Read, Write},
        process::{Command, Stdio},
    };
    use crate::types::{
        ProcBlockImpl, Setter, TransformAssertion, TransformAssertions,
    };
    use super::*;

    fn rustfmt(tokens: TokenStream) -> String {
        let mut child = Command::new("rustfmt")
            .arg("--emit=stdout")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = child.stdin.take().unwrap();
        let mut stdout = child.stdout.take().unwrap();

        writeln!(stdin, "fn main() {{ {} }}", tokens).unwrap();
        stdin.flush().unwrap();
        drop(stdin);

        let mut pretty = String::new();
        stdout.read_to_string(&mut pretty).unwrap();
        drop(stdout);

        eprintln!("====\n{}\n====", pretty);

        let start = pretty.find("{").unwrap();
        drop(pretty.drain(..=start));
        let end = pretty.rfind("}").unwrap();
        drop(pretty.drain(end..));

        child.kill().unwrap();

        pretty
    }

    macro_rules! assert_eq_tok {
        ($left:expr, $right:expr) => {
            let left = $left;
            let right = $right;

            if left.to_string() != right.to_string() {
                let left = rustfmt(left);
                let right = rustfmt(right);
                difference::assert_diff!(&left, &right, "\n", 0);
            }
        };
    }

    #[test]
    fn setter_assertion() {
        let assertion = SetterAssertion {
            proc_block_type: syn::parse_str("Proc").unwrap(),
            property: syn::parse_str("first").unwrap(),
            setter_argument: syn::parse_str("f32").unwrap(),
        };
        let should_be = quote! {
            const _: () =  {
                fn _assert_first_is_settable(proc_block: &mut Proc, first: f32) -> &mut Proc {
                    proc_block.set_first(first)
                }
            };
        };

        let got = assertion.to_token_stream();

        assert_eq_tok!(got, should_be);
    }

    #[test]
    fn custom_section() {
        let section = CustomSection {
            type_name: syn::parse_str("Proc").unwrap(),
            payload: b"Hello, World!".to_vec(),
        };
        let should_be = quote! {
            #[doc(hidden)]
            #[no_mangle]
            #[link_section = ".rune_proc_block"]
            pub static PROC_BLOCK_DESCRIPTOR_FOR_Proc: [u8; 13usize] = *b"Hello, World!";
        };

        let got = section.to_token_stream();

        assert_eq_tok!(got, should_be);
    }

    #[test]
    fn setter_implementation() {
        let setter = Setter {
            property: syn::parse_str("first").unwrap(),
            property_type: syn::parse_str("f32").unwrap(),
        };
        let should_be = quote! {
            pub fn set_first(&mut self, first: impl Into<f32>) -> &mut Self {
                self.first = first.into();
                self
            }
        };

        let got = setter.to_token_stream();

        assert_eq_tok!(got, should_be);
    }

    #[test]
    fn transform_assertions() {
        let assertions = TransformAssertions {
            proc_block_type: syn::parse_str("Proc").unwrap(),
            exports: syn::parse_str("exports").unwrap(),
            assertions: vec![TransformAssertion {
                input: syn::parse_str("[f32; 3]").unwrap(),
                output: syn::parse_str("[u8; _]").unwrap(),
            }],
        };
        let should_be = quote! {
            fn assert_implements_transform<T, Inputs, Outputs>()
            where
                T: exports::Transform<Inputs, Output=Outputs>
            { }

            fn transform_assertions() {
                assert_implements_transform::<Proc, exports::Tensor<f32>, exports::Tensor<u8>>();
            }
        };

        let got = assertions.to_token_stream();

        assert_eq_tok!(got, should_be);
    }

    #[test]
    fn implement_proc_block_trait_with_no_params_or_transforms() {
        let input = ProcBlockImpl {
            type_name: syn::parse_str("Proc").unwrap(),
            exports: syn::parse_str("exports").unwrap(),
            descriptor: ProcBlockDescriptor {
                type_name: "Proc".into(),
                description: "Hello, World!".into(),
                parameters: Cow::default(),
                available_transforms: Cow::default(),
            },
        };
        let should_be = quote! {
            impl exports::ProcBlock for Proc {
                const DESCRIPTOR: exports::ProcBlockDescriptor<'static> = exports::ProcBlockDescriptor {
                    type_name: exports::Cow::Borrowed("Proc"),
                    description: exports::Cow::Borrowed("Hello, World!"),
                    parameters: exports::Cow::Borrowed(&[]),
                    available_transforms: exports::Cow::Borrowed(&[]),
                };
            }
        };

        let got = input.to_token_stream();

        assert_eq_tok!(got, should_be);
    }

    #[test]
    fn transform() {
        let exports = syn::parse_str("exports").unwrap();
        let transform = TransformProxy {
            exports: &exports,
            transform: &TransformDescriptor {
                input: TensorDescriptor {
                    element_type: ReflectType::f32,
                    dimensions: Dimensions::Arbitrary,
                },
                output: TensorDescriptor {
                    element_type: ReflectType::u8,
                    dimensions: Dimensions::Finite(
                        vec![Dimension::Value(1980)].into(),
                    ),
                },
            },
        };
        let should_be = quote! {
            exports::TransformDescriptor {
                input: exports::TensorDescriptor {
                    element_type: exports::Type::Float { bit_width: 32usize },
                    dimensions: exports::Dimensions::Arbitrary,
                },
                output: exports::TensorDescriptor {
                    element_type: exports::Type::Integer { signed: false, bit_width: 8usize },
                    dimensions: exports::Dimensions::Finite(
                        exports::Cow::Borrowed(&[
                            exports::Dimension::Value(1980usize),
                        ]),
                    ),
                },
            }
        };

        let got = transform.to_token_stream();

        assert_eq_tok!(got, should_be);
    }
}
