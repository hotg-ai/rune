use std::collections::HashSet;

use legion::{Entity, systems::CommandBuffer, world::SubWorld, IntoQuery};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{ToTokens, quote};
use heck::{CamelCase, SnakeCase};
use crate::{
    codegen::{CustomSection, File},
    lowering::{
        Inputs, Model, ModelFile, Name, Outputs, ProcBlock, Resource,
        ResourceData, Sink, SinkKind, Source, SourceKind, Tensor,
    },
    parse::{ResourceOrString, ResourceType, Value},
};

#[legion::system]
#[read_component(CustomSection)]
#[read_component(Inputs)]
#[read_component(Model)]
#[read_component(Name)]
#[read_component(Outputs)]
#[read_component(ProcBlock)]
#[read_component(Resource)]
#[read_component(ResourceData)]
#[read_component(Sink)]
#[read_component(Source)]
#[read_component(Tensor)]
pub(crate) fn run(cmd: &mut CommandBuffer, world: &SubWorld) {
    let mut sections = <&CustomSection>::query();
    let mut models = <(&Name, &Model, &Inputs, &Outputs)>::query();
    let mut names = <&Name>::query();
    let mut tensors = <&Tensor>::query();
    let mut resources = <(&Name, &Resource, Option<&ResourceData>)>::query();
    let mut capabilities = <(&Name, &Source)>::query();
    let mut proc_blocks = <(&Name, &ProcBlock)>::query();
    let mut outputs = <(&Name, &Sink)>::query();
    let mut pipeline_nodes =
        <(Entity, &Name, Option<&Inputs>, Option<&Outputs>)>::query();

    let models: Vec<_> = models.iter(world).collect();
    let sections: Vec<_> = sections.iter(world).collect();
    let resources: Vec<_> = resources.iter(world).collect();
    let capabilities: Vec<_> = capabilities.iter(world).collect();
    let proc_blocks: Vec<_> = proc_blocks.iter(world).collect();
    let outputs: Vec<_> = outputs.iter(world).collect();
    let pipeline_nodes: Vec<_> = pipeline_nodes.iter(world).collect();

    let lib_rs = generate_lib_rs(
        &sections,
        &models,
        &resources,
        &capabilities,
        &proc_blocks,
        &outputs,
        &pipeline_nodes,
        |ent| names.get(world, ent).ok(),
        |ent| tensors.get(world, ent).ok(),
    );
    let file = File::new("lib.rs", lib_rs.to_string().into_bytes());

    cmd.push((file,));
}

fn generate_lib_rs<'world>(
    sections: &[&CustomSection],
    models: &'world [(
        &'world Name,
        &'world Model,
        &'world Inputs,
        &'world Outputs,
    )],
    resources: &[(&Name, &Resource, Option<&ResourceData>)],
    capabilities: &[(&Name, &Source)],
    proc_blocks: &[(&Name, &ProcBlock)],
    outputs: &[(&Name, &Sink)],
    pipeline_nodes: &[(&Entity, &Name, Option<&Inputs>, Option<&Outputs>)],
    mut get_name: impl FnMut(Entity) -> Option<&'world Name>,
    mut get_tensor: impl FnMut(Entity) -> Option<&'world Tensor>,
) -> TokenStream {
    let prelude = generate_prelude();
    let custom_sections = generate_custom_sections(sections);
    let resources_module = generate_resources_module(resources);
    let models_module = generate_models_module(
        models.iter().map(|(n, m, _, _)| (*n, *m)),
        &mut get_name,
    );
    let manifest = generate_manifest_function(
        models,
        capabilities,
        proc_blocks,
        outputs,
        pipeline_nodes,
        &mut get_name,
        &mut get_tensor,
    );
    let call = generate_call_function();

    quote! {
        #prelude
        #custom_sections
        #resources_module
        #models_module
        #manifest
        #call
    }
}

/// Generate a `manifest()` function that initializes the various nodes in
/// our pipeline then turns it into a closure that gets stored in the
/// `PIPELINE` static variable.
fn generate_manifest_function<'world, F, T>(
    models: &[(&Name, &Model, &Inputs, &Outputs)],
    capabilities: &[(&Name, &Source)],
    proc_blocks: &[(&Name, &ProcBlock)],
    outputs: &[(&Name, &Sink)],
    pipeline_nodes: &[(&Entity, &Name, Option<&Inputs>, Option<&Outputs>)],
    get_name: &mut F,
    get_tensor: &mut T,
) -> TokenStream
where
    F: FnMut(Entity) -> Option<&'world Name>,
    T: FnMut(Entity) -> Option<&'world Tensor>,
{
    let capabilities = initialize_capabilities(capabilities);
    let proc_blocks = initialize_proc_blocks(proc_blocks);
    let models: TokenStream = models
        .iter()
        .map(|(n, m, i, o)| initialize_model(n, m, i, o, get_name, get_tensor))
        .collect();
    let outputs = initialize_outputs(outputs);
    let pipeline = execute_pipeline(pipeline_nodes);

    quote! {
        #[no_mangle]
        pub extern "C" fn manifest() -> i32 {
            #capabilities
            #proc_blocks
            #models
            #outputs

            let pipeline = move || {
                let _guard = hotg_runicos_base_wasm::PipelineGuard::default();
                #pipeline
            };

            unsafe {
                PIPELINE = Some(Box::new(pipeline));
            }

            1
        }
    }
}

fn execute_pipeline(
    pipeline_nodes: &[(&Entity, &Name, Option<&Inputs>, Option<&Outputs>)],
) -> TokenStream {
    todo!()
}

fn initialize_outputs(outputs: &[(&Name, &Sink)]) -> TokenStream {
    outputs
        .iter()
        .map(|(name, sink)| initialize_output(name, sink))
        .collect()
}

fn initialize_output(name: &Name, sink: &Sink) -> TokenStream {
    let name = Ident::new(name, Span::call_site());
    let type_name: TokenStream = sink_type_name(&sink.kind);

    quote! {
        let mut #name = #type_name::default();
    }
}

fn sink_type_name(kind: &SinkKind) -> TokenStream {
    match kind {
        SinkKind::Serial => quote!(hotg_runicos_base_wasm::Serial),
        SinkKind::Other(other) => {
            unimplemented!("Unable to handle \"{}\" outputs", other)
        },
    }
}

fn initialize_model<'world, N, T>(
    name: &Name,
    model: &Model,
    inputs: &Inputs,
    outputs: &Outputs,
    get_name: &mut N,
    get_tensor: &mut T,
) -> TokenStream
where
    N: FnMut(Entity) -> Option<&'world Name>,
    T: FnMut(Entity) -> Option<&'world Tensor>,
{
    let name = Ident::new(name, Span::call_site());

    let path_to_model_bytes = match &model.model_file {
        ModelFile::FromDisk(_) => quote!(crate::models::#name),
        ModelFile::Resource(resource) => {
            let resource_name = get_name(*resource)
                .expect("We should always be able to get a resource's name");
            let resource_name = Ident::new(resource_name, Span::call_site());
            quote!(crate::resources::#resource_name)
        },
    };

    let input_descriptors: TokenStream =
        tensor_descriptors(&inputs.tensors, get_tensor);
    let output_descriptors: TokenStream =
        tensor_descriptors(&outputs.tensors, get_tensor);

    let mimetype = "application/tflite-model";

    quote! {
        let mut #name = hotg_runicos_base_wasm::Model::load(
            #mimetype,
            &#path_to_model_bytes,
            #input_descriptors,
            #output_descriptors,
        );
    }
}

fn tensor_descriptors<'world, T>(
    tensors: &[Entity],
    get_tensor: &mut T,
) -> TokenStream
where
    T: FnMut(Entity) -> Option<&'world Tensor>,
{
    let inputs = tensors
        .iter()
        .map(|&ent| {
            get_tensor(ent).expect("All tensors should have been allocated")
        })
        .map(|t| t.0.to_string());
    quote! { &[#(#inputs),*] }
}

fn initialize_proc_blocks(proc_blocks: &[(&Name, &ProcBlock)]) -> TokenStream {
    proc_blocks
        .iter()
        .copied()
        .map(|(name, proc_block)| initialize_proc_block(name, proc_block))
        .collect()
}

fn initialize_proc_block(name: &Name, proc_block: &ProcBlock) -> TokenStream {
    let ty = proc_block_type(proc_block);

    let name = Ident::new(name, Span::call_site());
    let setters = proc_block.parameters.iter().map(|(key, value)| {
        let value = value_to_tokens(value);
        let setter = format!("set_{}", key);
        let setter = Ident::new(&setter, Span::call_site());
        quote! {
            #name.#setter(#value);
        }
    });

    quote! {
        let mut #name = #ty::default();
        #( #setters )*
    }
}

fn proc_block_type(proc_block: &ProcBlock) -> TokenStream {
    let module_name = proc_block.name().to_snake_case();
    let type_name = module_name.to_camel_case();

    let module_name = Ident::new(&module_name, Span::call_site());
    let type_name = Ident::new(&type_name, Span::call_site());

    quote!(#module_name::#type_name)
}

fn initialize_capabilities(capabilities: &[(&Name, &Source)]) -> TokenStream {
    capabilities
        .iter()
        .copied()
        .map(|(name, source)| initialize_capability(name, source))
        .collect()
}

fn initialize_capability(name: &Name, source: &Source) -> TokenStream {
    let capability_type = match &source.kind {
        SourceKind::Random => Ident::new("Random", Span::call_site()),
        SourceKind::Accelerometer => {
            Ident::new("Accelerometer", Span::call_site())
        },
        SourceKind::Sound => Ident::new("Sound", Span::call_site()),
        SourceKind::Image => Ident::new("Image", Span::call_site()),
        SourceKind::Raw => Ident::new("Raw", Span::call_site()),
        SourceKind::Other(other) => unimplemented!(
            "Unable to generate code for the \"{}\" capability type",
            other
        ),
    };

    let name = Ident::new(name, Span::call_site());
    let setters = source.parameters.iter().map(|(key, value)| {
        let value = value_to_tokens(value);
        quote! {
            #name.set_parameter(#key, #value);
        }
    });

    quote! {
        let mut #name = hotg_runicos_base_wasm::#capability_type::default();
        #( #setters )*
    }
}

fn value_to_tokens(value: &Value) -> TokenStream {
    match value {
        Value::Int(i) => i.into_token_stream(),
        Value::Float(f) => f.into_token_stream(),
        Value::String(ResourceOrString::String(s)) => s.into_token_stream(),
        Value::String(ResourceOrString::Resource(r)) => {
            let resource_name = Ident::new(r, Span::call_site());
            quote!(&*crate::resources::#resource_name)
        },
        Value::List(list) => {
            let tokens = list.iter().map(value_to_tokens);
            quote! { &[ #(#tokens),* ] }
        },
    }
}

/// Imports and miscellaneous attributes added to the top of the file.
fn generate_prelude() -> TokenStream {
    quote! {
        //! Automatically generated by Rune. DO NOT EDIT!

        #![no_std]
        #![feature(alloc_error_handler)]
        #![allow(warnings)]

        extern crate alloc;

        #[macro_use]
        extern crate lazy_static;

        use alloc::boxed::Box;
        use hotg_rune_core::*;
        use hotg_rune_proc_blocks::*;

        static mut PIPELINE: Option<Box<dyn FnMut()>> = None;
    }
}

/// The `call()` function - a simple function which invokes the `PIPELINE`
/// constructed by [`generate_manifest_function()`].
fn generate_call_function() -> TokenStream {
    quote! {
        #[no_mangle]
        pub extern "C" fn _call(
            _capability_type: i32,
            _input_type: i32,
            _capability_idx: i32,
        ) -> i32 {
            unsafe {
                let pipeline = PIPELINE.as_mut()
                    .expect("The rune hasn't been initialized");
                pipeline();

                0
            }
        }
    }
}

/// Generate WebAssembly custom sections which are used to embed metadata in
/// the compiled Rune.
fn generate_custom_sections(sections: &[&CustomSection]) -> TokenStream {
    let sections = sections
        .iter()
        .enumerate()
        .map(|(i, section)| generate_custom_section(i, *section));

    quote! {
        /// Custom sections embedded in the Rune that can be inspected later.
        ///
        /// # Note
        ///
        /// These sections need to be at the top level to make sure the linker
        /// won't remove them during its "gc sections" pass, but we also don't
        /// want to pollute the top-level namespace so we put it inside an
        /// unnamed constant.
        const _: () = {
            #( #sections )*
        };
    }
}

/// Generate the declaration for a [`CustomSection`], appending a unique number
/// to help avoid duplicates when you've got multiple [`CustomSection`]s with
/// the same name.
fn generate_custom_section(
    section_number: usize,
    s: &CustomSection,
) -> TokenStream {
    let unique_ident = format!("{}_{}", s.identifier(), section_number);
    let ident = Ident::new(&unique_ident, Span::call_site());
    let section_name = &s.section_name;
    let data = Literal::byte_string(&s.value);
    let len = s.value.len();

    quote! {
        #[link_section = #section_name]
        static #ident: [u8; #len] = *#data;
    }
}

fn generate_resources_module(
    resources: &[(&Name, &Resource, Option<&ResourceData>)],
) -> TokenStream {
    let initializers = resources
        .iter()
        .copied()
        .map(|(name, res, data)| resource_initializer(name, res, data));

    quote! {
        /// Lazily loaded accessors for all resources used by this Rune.
        mod resources {
            lazy_static::lazy_static! {
                #(#initializers)*
            }
        }
    }
}

fn resource_initializer(
    name: &Name,
    res: &Resource,
    data: Option<&ResourceData>,
) -> TokenStream {
    let name = name.as_str();

    // First we try to read the resource using the runtime, returning a
    // Result<Vec<u8>, _>
    let maybe_bytes = quote! {
        hotg_runicos_base_wasm::Resource::read_to_end(name)
    };

    // We then take the Result and unwrap it, either falling back to a default
    // value (provided in the Runefile) or blowing up
    let bytes = match data {
        Some(default_value) => {
            let default_value = Literal::byte_string(default_value);
            quote!(#maybe_bytes.unwrap_or(#default_value))
        },
        None => {
            let error_message =
                format!("Unable to read the \"{}\" resource", name);
            quote!(#maybe_bytes.expect(#error_message))
        },
    };

    let ident = Ident::new(name, Span::call_site());

    // And now we can initialize our "static ref"
    match res.ty {
        ResourceType::String => {
            let error_message =
                format!("The \"{}\" resource isn't valid UTF-8", name);

            quote! {
                pub(crate) static ref #ident: alloc::string::String =
                    core::str::from_utf8(#bytes).expect(#error_message).into();
            }
        },
        ResourceType::Binary => {
            quote! {
                pub(crate) static ref #ident: alloc::vec::Vec<u8> = #bytes.to_vec();
            }
        },
    }
}

fn generate_models_module<'world, N, M>(
    models: M,
    get_name: &mut N,
) -> TokenStream
where
    M: Iterator<Item = (&'world Name, &'world Model)>,
    N: FnMut(Entity) -> Option<&'world Name>,
{
    let initializers =
        models.map(|(name, model)| model_initializer(name, model, get_name));

    quote! {
        mod models {
            lazy_static::lazy_static! {
                #(#initializers)*
            }
        }
    }
}

fn model_initializer<'world, N>(
    name: &Name,
    model: &Model,
    get_name: &mut N,
) -> TokenStream
where
    N: FnMut(Entity) -> Option<&'world Name>,
{
    let name = Ident::new(name, Span::call_site());

    match &model.model_file {
        ModelFile::FromDisk(_) => {
            let path = format!("models/{}", name);

            quote! {
                pub(crate) static ref #name: &[u8] = include_str!(#path);
            }
        },
        ModelFile::Resource(resource) => {
            let resource_name = get_name(*resource).unwrap();
            let resource_name = Ident::new(resource_name, Span::call_site());

            quote! {
                pub(crate) static ref #name: &[u8] = crate::resources::#resource_name.as_ref();
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Write, Read},
        process::{Command, Stdio},
    };
    use super::*;

    fn rustfmt(tokens: TokenStream) -> String {
        let mut child = Command::new("rustfmt")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        // Note: We need to wrap the fragment in a function so it'll parse
        let mut stdin = child.stdin.take().unwrap();
        writeln!(stdin, "fn main() {{").unwrap();
        writeln!(stdin, "{}", tokens).unwrap();
        writeln!(stdin, "}}").unwrap();
        stdin.flush().unwrap();
        drop(stdin);

        let mut stdout = child.stdout.take().unwrap();
        let mut pretty = String::new();
        stdout.read_to_string(&mut pretty).unwrap();

        let opening_curly = pretty.find("{").unwrap();
        let closing_curly = pretty.rfind("}").unwrap();

        pretty[opening_curly + 1..closing_curly].trim().to_string()
    }

    macro_rules! assert_quote_eq {
        ($left:expr, $right:expr) => {{
            let left = $left.to_string();
            let right = $right.to_string();

            if left != right {
                let pretty_left = rustfmt($left);
                let pretty_right = rustfmt($right);
                assert_eq!(pretty_left, pretty_right);
                assert_eq!(left, right);
            }
        }};
    }

    #[test]
    fn custom_section() {
        let section = CustomSection::new(".name", b"hello world".as_ref());
        let should_be = quote! {
            #[link_section = ".name"]
            static name_42: [u8; 11usize] = *b"hello world";
        };

        let got = generate_custom_section(42, &section);

        assert_quote_eq!(got, should_be);
    }
}
