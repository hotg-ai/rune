use proc_macro2::{Ident, TokenStream, Span};
use quote::quote;
use runic_types::reflect::Type;
use crate::{
    descriptor::{
        ProcBlockDescriptor, ParameterDescriptor, TransformDescriptor,
        TensorDescriptor, Dimension, Dimensions,
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
    let assertions = expand_assertions(&name, &exports, &descriptor);
    let setters = expand_setters(&name, &descriptor.parameters);
    let descriptor = expand_descriptor(&name, &exports, &descriptor);

    quote! {
        impl #exports::ProcBlock for #name {
            const DESCRIPTOR: #exports::ProcBlockDescriptor<'static> = #descriptor;
        }

        #setters
        #custom_section
        #assertions
    }
}

fn expand_setters(
    name: &Ident,
    parameters: &[ParameterDescriptor<'_>],
) -> TokenStream {
    if parameters.is_empty() {
        return TokenStream::new();
    }

    let setters = parameters.iter().map(|p| {
        let ParameterDescriptor {
            name,
            parameter_type,
            ..
        } = p;
        let method_name = format!("set_{}", name);
        let method_name = Ident::new(&method_name, Span::call_site());
        let name = Ident::new(&name, Span::call_site());
        let ty: syn::Type =
            syn::parse_str(parameter_type.rust_name().unwrap()).unwrap();

        quote!(
            pub fn #method_name(&mut self, #name: #ty) -> &mut Self {
                self.#name = #name;
                self
            }
        )
    });

    quote! {
        impl #name {
            #( #setters )*
        }
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

    let section_name = ProcBlockDescriptor::CUSTOM_SECTION_NAME;

    quote! {
        #[link_section = #section_name]
        #[no_mangle]
        pub static #name: [u8; #len] = *#serialized;
    }
}

/// Add static assertions to ensure the proc block's implementation matches
/// what they have declared.
fn expand_assertions(
    name: &Ident,
    exports: &Path,
    descriptor: &ProcBlockDescriptor<'_>,
) -> TokenStream {
    let transform_assertions = expand_transform_assertions(
        name,
        exports,
        &descriptor.available_transforms,
    );

    quote! {
        const _: () = {
            #transform_assertions
        };
    }
}

/// Adds a bunch of static assertions to make sure you have the appropriate
/// `Transform` impl when you say `#[transform(input = f32, output = [u8; 3])]`.
fn expand_transform_assertions(
    name: &Ident,
    exports: &Path,
    available_transforms: &[TransformDescriptor<'_>],
) -> TokenStream {
    if available_transforms.is_empty() {
        return TokenStream::new();
    }

    let assertions = available_transforms.iter().map(
        |TransformDescriptor {
             input:
                 TensorDescriptor {
                     element_type: input,
                     ..
                 },
             output:
                 TensorDescriptor {
                     element_type: output,
                     ..
                 },
         }| {
            let input = expand_tensor_type(exports, input);
            let output = expand_tensor_type(exports, output);

            quote! {
                assert_implements_transform::<#name, #input, #output>();
            }
        },
    );

    quote! {
        fn assert_implements_transform<T, Inputs, Outputs>()
        where
            T: #exports::Transform<Inputs, Output=Outputs>
        { }

        fn transform_assertions() {
            #( #assertions )*
        }
    }
}

fn expand_tensor_type(exports: &Path, t: &Type) -> TokenStream {
    match t
        .rust_name()
        .and_then(|name| syn::parse_str::<syn::Type>(name).ok())
    {
        Some(ref name) => quote!(#exports::Tensor<#name>),
        None => panic!("Unable to get the tensor representation of {:?}", t),
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

    let available_transforms = available_transforms
        .iter()
        .map(|d| expand_transform_descriptor(exports, d));
    let parameters = parameters
        .iter()
        .map(|p| expand_parameter_descriptor(exports, p));

    quote! {
        #exports::ProcBlockDescriptor {
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

fn expand_transform_descriptor(
    exports: &Path,
    d: &TransformDescriptor<'_>,
) -> TokenStream {
    let TransformDescriptor { input, output } = d;
    let input = expand_tensor_descriptor(exports, input);
    let output = expand_tensor_descriptor(exports, output);

    quote! {
       #exports::TransformDescriptor {
           input: #input,
           output: #output,
       }
    }
}

fn expand_tensor_descriptor(
    exports: &Path,
    d: &TensorDescriptor<'_>,
) -> TokenStream {
    let TensorDescriptor {
        element_type,
        dimensions,
    } = d;

    let element_type = expand_type(exports, element_type);
    let dimensions = expand_dimensions(exports, dimensions);

    quote! {
        #exports::TensorDescriptor {
            element_type: #element_type,
            dimensions: #dimensions,
        }
    }
}

fn expand_dimensions(
    exports: &Path,
    dimensions: &Dimensions<'_>,
) -> TokenStream {
    match dimensions {
        Dimensions::Finite(finite) => {
            let dimensions =
                finite.iter().copied().map(|d| expand_dimension(exports, d));

            quote! {
                #exports::Dimensions::Finite(#exports::Cow::Borrowed(&[
                    #( #dimensions ),*
                ]))
            }
        },
        Dimensions::Arbitrary => quote!(#exports::Dimensions::Arbitrary),
    }
}

fn expand_dimension(exports: &Path, dimension: Dimension) -> TokenStream {
    match dimension {
        Dimension::Any => quote!(#exports::Dimension::Any),
        Dimension::Value(v) => quote!(#exports::Dimension::Value(#v)),
    }
}

fn expand_type(exports: &Path, t: &Type) -> TokenStream {
    match *t {
        Type::Integer { signed, bit_width } => quote!(#exports::Type::Integer {
            signed: #signed,
            bit_width: #bit_width,
        }),
        Type::Float { bit_width } => {
            quote!(#exports::Type::Float { bit_width: #bit_width })
        },
        Type::String => quote!(#exports::Type::String),
        Type::Opaque { ref type_name } => {
            let type_name = &*type_name;
            quote!(#exports::Type::Opaque {
               type_name: #exports::Cow::Borrowed(#type_name),
            })
        },
    }
}

fn expand_parameter_descriptor(
    exports: &Path,
    d: &ParameterDescriptor<'_>,
) -> TokenStream {
    let ParameterDescriptor {
        name,
        description,
        parameter_type,
    } = d;
    let parameter_type = expand_type(exports, parameter_type);

    quote! {
       #exports::ParameterDescriptor {
           name: #exports::Cow::Borrowed(#name),
           description: #exports::Cow::Borrowed(#description),
           parameter_type: #parameter_type,
       }
    }
}
