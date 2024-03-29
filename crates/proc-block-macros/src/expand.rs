use hotg_rune_core::ElementType;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Generics, LitByteStr, Path, Type};

use crate::{
    descriptor::{
        Dimension, Dimensions, ProcBlockDescriptor, TensorDescriptor,
        TransformDescriptor,
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

fn generic_parameters(g: &Generics) -> (TokenStream, TokenStream) {
    if g.params.is_empty() {
        return (TokenStream::new(), TokenStream::new());
    }

    let params = g.params.iter();
    let impl_generics = quote!(<#(#params),*>);

    let params = g.type_params().map(|p| &p.ident);
    let type_generics = quote!(<#(#params),*>);

    (impl_generics, type_generics)
}

impl ToTokens for Setters {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Setters {
            type_name,
            setters,
            generics,
        } = self;

        let (impl_generics, type_generics) = generic_parameters(generics);

        let t = quote! {
            impl #impl_generics #type_name #type_generics  {
                #( #setters )*
            }
        };
        tokens.extend(t);
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
            ..
        } = self;

        let assertion_name = format!("_assert_{}_is_settable", property);
        let assertion_name = Ident::new(&assertion_name, property.span());
        let setter_name = format!("set_{}", property);
        let setter_name = Ident::new(&setter_name, property.span());

        let t = quote! {
            const _: () = {
                fn #assertion_name(proc_block: &mut #proc_block_type, #property: &str) {
                    fn assert_return_is_result_debug(_: Result<(), impl core::fmt::Debug>) {}

                    let result = proc_block.#setter_name(#property);
                    assert_return_is_result_debug(result);
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

        // Note: We'll use `#[cfg]` to make sure the custom section is only
        // included when compiling to WebAssembly. Apparently mach-o object
        // files don't support section names starting with a "." and this custom
        // section doesn't make sense for non-WebAssembly use cases anyway
        //
        // LLVM ERROR: Global variable 'PROC_BLOCK_DESCRIPTOR_FOR_Normalize' has
        // an invalid section specifier '.rune_proc_block': mach-o section
        // specifier requires a segment and section separated by a comma.

        let t = quote! {
            #[doc(hidden)]
            #[no_mangle]
            #[cfg(target_arch = "wasm32")]
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
            pub fn #property(&self) -> &#property_type { &self.#property }

            pub fn #method(&mut self, #property: &str) -> Result<(), impl core::fmt::Debug>
            {
                #property.parse().map(|value| { self.#property = value; })
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

        let assertions = assertions
            .iter()
            .map(|TransformAssertion { inputs, outputs }| {
                let inputs = transform_assertion_type(inputs);
                let outputs = transform_assertion_type(outputs);
                quote! {
                    assert_implements_transform::<#proc_block_type, #inputs, #outputs>();
                }
            });

        let t = quote! {
            const _: () = {
                fn assert_implements_transform<T, Inputs, Outputs>()
                where
                    T: #exports::Transform<Inputs, Output=Outputs>
                { }

                fn transform_assertions() {
                    #( #assertions )*
                }
            };
        };
        tokens.extend(t);
    }
}

fn transform_assertion_type(types: &[Type]) -> TokenStream {
    match types {
        [ty] => ty.to_token_stream(),
        _ => quote!((#(#types),*)),
    }
}

impl ToTokens for ProcBlockImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ProcBlockImpl {
            type_name,
            exports,
            descriptor,
            generics,
        } = self;

        let descriptor = descriptor_to_tokens(exports, descriptor);

        let (impl_generics, type_generics) = generic_parameters(generics);

        let t = quote! {
            impl #impl_generics #exports::ProcBlock for #type_name #type_generics  {
                const DESCRIPTOR: #exports::ProcBlockDescriptor<'static> = #descriptor;
            }
        };
        tokens.extend(t);
    }
}

fn descriptor_to_tokens<'a, 'b: 'a>(
    exports: &'a Path,
    d: &'a ProcBlockDescriptor<'b>,
) -> TokenStream {
    let ProcBlockDescriptor {
        type_name,
        description,
        available_transforms,
    } = d;

    let available_transforms = available_transforms
        .iter()
        .map(|transform| transform_to_tokens(exports, transform));

    quote! {
        #exports::ProcBlockDescriptor {
            type_name: #exports::Cow::Borrowed(#type_name),
            description: #exports::Cow::Borrowed(#description),
            available_transforms: #exports::Cow::Borrowed(&[
                #( #available_transforms ),*
            ]),
        }
    }
}

fn transform_to_tokens(
    exports: &Path,
    transform: &TransformDescriptor<'_>,
) -> TokenStream {
    let TransformDescriptor { inputs, outputs } = transform;

    let inputs = tensor_descriptors_to_tokens(exports, inputs);
    let outputs = tensor_descriptors_to_tokens(exports, outputs);

    quote! {
        #exports::TransformDescriptor {
            inputs: #exports::TensorDescriptors(#exports::Cow::Borrowed(#inputs)),
            outputs: #exports::TensorDescriptors(#exports::Cow::Borrowed(#outputs)),
        }
    }
}

fn tensor_descriptors_to_tokens(
    exports: &Path,
    tensors: &[TensorDescriptor<'_>],
) -> TokenStream {
    let descriptors = tensors.iter().map(
        |TensorDescriptor {
             element_type,
             dimensions,
         }| {
            let element_type = element_type_to_tokens(exports, *element_type);
            let dimensions = dimensions_to_tokens(exports, dimensions);

            quote! {
                #exports::TensorDescriptor {
                    element_type: #element_type,
                    dimensions: #dimensions,
                }
            }
        },
    );

    quote! {
        &[
            #( #descriptors ),*
        ]
    }
}

fn element_type_to_tokens(exports: &Path, ty: ElementType) -> TokenStream {
    let name = match ty {
        ElementType::U8 => "U8",
        ElementType::I8 => "I8",
        ElementType::U16 => "U16",
        ElementType::I16 => "I16",
        ElementType::U32 => "U32",
        ElementType::F32 => "F32",
        ElementType::I32 => "I32",
        ElementType::U64 => "U64",
        ElementType::F64 => "F64",
        ElementType::I64 => "I64",
        ElementType::String => "String",
    };
    let ident = Ident::new(name, Span::call_site());
    quote!(#exports::ElementType::#ident)
}

fn dimensions_to_tokens(
    exports: &Path,
    dimensions: &Dimensions<'_>,
) -> TokenStream {
    match dimensions {
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
        io::Write,
        process::{Command, Output, Stdio},
    };

    use syn::Generics;

    use super::*;
    use crate::types::{
        ProcBlockImpl, Setter, TransformAssertion, TransformAssertions,
    };

    fn rustfmt(tokens: TokenStream) -> String {
        let mut child = Command::new("rustfmt")
            .arg("--emit=stdout")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = child.stdin.take().unwrap();

        let input = quote! {
            const _: () = { #tokens };
        };

        writeln!(stdin, "{}", input).unwrap();
        stdin.flush().unwrap();
        drop(stdin);

        let Output { stdout, status, .. } = child.wait_with_output().unwrap();

        assert!(status.success(), "Unable to format the input\n\n{}", input);

        let mut pretty = String::from_utf8(stdout).unwrap();

        let start = pretty.find('{').unwrap();
        drop(pretty.drain(..=start));
        let end = pretty.rfind('}').unwrap();
        drop(pretty.drain(end..));

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
                fn _assert_first_is_settable(proc_block: &mut Proc, first: &str) {
                    fn assert_return_is_result_debug(
                        _: Result<(), impl core::fmt::Debug>,
                    ) { }
                    let result = proc_block.set_first(first);
                    assert_return_is_result_debug(result);
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
            #[cfg(target_arch = "wasm32")]
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
            pub fn first(&self) -> &f32 { &self.first }
            pub fn set_first(
                &mut self,
                first: &str,
            ) -> Result<(), impl core::fmt::Debug> {
                first.parse().map(|value| { self.first = value; })
            }
        };

        let got = setter.to_token_stream();

        assert_eq_tok!(got, should_be);
    }

    #[test]
    fn transform_assertion_with_string_input() {
        let assertions = TransformAssertions {
            proc_block_type: syn::parse_str("Proc").unwrap(),
            exports: syn::parse_str("exports").unwrap(),
            assertions: vec![TransformAssertion {
                inputs: vec![syn::parse_str(
                    "exports::Tensor<Cow<'static, str>>",
                )
                .unwrap()],
                outputs: vec![syn::parse_str(
                    "exports::Tensor<Cow<'static, str>>",
                )
                .unwrap()],
            }],
        };
        let should_be = quote! {
            const _: () = {
                fn assert_implements_transform<T, Inputs, Outputs>()
                where
                    T: exports::Transform<Inputs, Output=Outputs>
                { }

                fn transform_assertions() {
                    assert_implements_transform::<Proc, exports::Tensor<Cow<'static, str>>, exports::Tensor<Cow<'static, str>>>();
                }
            };
        };

        let got = assertions.to_token_stream();

        assert_eq_tok!(got, should_be);
    }

    #[test]
    fn transform_assertions() {
        let assertions = TransformAssertions {
            proc_block_type: syn::parse_str("Proc").unwrap(),
            exports: syn::parse_str("exports").unwrap(),
            assertions: vec![TransformAssertion {
                inputs: vec![syn::parse_str("exports::Tensor<f32>").unwrap()],
                outputs: vec![syn::parse_str("exports::Tensor<u8>").unwrap()],
            }],
        };
        let should_be = quote! {
            const _: () = {
                fn assert_implements_transform<T, Inputs, Outputs>()
                where
                    T: exports::Transform<Inputs, Output=Outputs>
                { }

                fn transform_assertions() {
                    assert_implements_transform::<Proc, exports::Tensor<f32>, exports::Tensor<u8>>();
                }
            };
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
                available_transforms: Cow::default(),
            },
            generics: Generics::default(),
        };
        let should_be = quote! {
            impl exports::ProcBlock for Proc {
                const DESCRIPTOR: exports::ProcBlockDescriptor<'static> = exports::ProcBlockDescriptor {
                    type_name: exports::Cow::Borrowed("Proc"),
                    description: exports::Cow::Borrowed("Hello, World!"),
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
        let transform = TransformDescriptor {
            inputs: TensorDescriptor {
                element_type: ElementType::F32,
                dimensions: Dimensions::Arbitrary,
            }
            .into(),
            outputs: TensorDescriptor {
                element_type: ElementType::U8,
                dimensions: Dimensions::Finite(
                    vec![Dimension::Value(1980)].into(),
                ),
            }
            .into(),
        };
        let should_be = quote! {
            exports::TransformDescriptor {
                inputs: exports::TensorDescriptors(exports::Cow::Borrowed(&[
                    exports::TensorDescriptor {
                        element_type: exports::ElementType::F32,
                        dimensions: exports::Dimensions::Arbitrary,
                    },
                ])),
                outputs: exports::TensorDescriptors(exports::Cow::Borrowed(&[
                    exports::TensorDescriptor {
                        element_type: exports::ElementType::U8,
                        dimensions: exports::Dimensions::Finite(
                            exports::Cow::Borrowed(&[
                                exports::Dimension::Value(1980usize),
                            ]),
                        ),
                    },
                ])),
            }
        };

        let got = transform_to_tokens(&exports, &transform);

        assert_eq_tok!(got, should_be);
    }
}
