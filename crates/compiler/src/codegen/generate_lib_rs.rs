use std::collections::{HashMap, HashSet};
use hotg_rune_core::{Shape, reflect::Type};
use legion::{Entity, Query, systems::CommandBuffer, world::SubWorld};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{ToTokens, quote};
use heck::{CamelCase, SnakeCase};
use crate::{
    codegen::{CustomSection, File},
    lowering::{
        Inputs, Model, ModelFile, Name, Outputs, PipelineNode, ProcBlock,
        Resource, ResourceData, Sink, SinkKind, Source, SourceKind, Tensor,
    },
    parse::{ResourceOrString, ResourceType, Value},
};

/// Generate the entire `lib.rs` file.
///
/// FIXME: This should be split up into different phases for generating each
/// part in the `lib.rs` file. Some low hanging fruit are things like the
/// resources and models modules or the initializers for each pipeline node
/// because they can be done using a `#[legion::system(for_each)]`.
#[legion::system]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    sections: &mut Query<&CustomSection>,
    models: &mut Query<(&Name, &Model, &Inputs, &Outputs)>,
    names: &mut Query<&Name>,
    tensors: &mut Query<(Entity, &Tensor, Option<&Inputs>, Option<&Outputs>)>,
    tensor_by_ent: &mut Query<&Tensor>,
    resources: &mut Query<(&Name, &Resource, Option<&ResourceData>)>,
    capabilities: &mut Query<(&Name, &Source)>,
    proc_blocks: &mut Query<(&Name, &ProcBlock)>,
    outputs: &mut Query<(&Name, &Sink)>,
    pipeline_nodes: &mut Query<(
        Entity,
        &Name,
        Option<&Inputs>,
        Option<&Outputs>,
        &PipelineNode,
    )>,
) {
    let models: Vec<_> = models.iter(world).collect();
    let sections: Vec<_> = sections.iter(world).collect();
    let resources: Vec<_> = resources.iter(world).collect();
    let capabilities: Vec<_> = capabilities.iter(world).collect();
    let proc_blocks: Vec<_> = proc_blocks.iter(world).collect();
    let outputs: Vec<_> = outputs.iter(world).collect();
    let pipeline_nodes: Vec<_> = pipeline_nodes.iter(world).collect();
    let tensors: Vec<_> = tensors.iter(world).collect();

    let lib_rs = generate_lib_rs(
        &sections,
        &models,
        &resources,
        &capabilities,
        &proc_blocks,
        &outputs,
        &pipeline_nodes,
        &tensors,
        |ent| names.get(world, ent).ok(),
        |ent| tensor_by_ent.get(world, ent).ok(),
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
    pipeline_nodes: &[(
        &Entity,
        &Name,
        Option<&Inputs>,
        Option<&Outputs>,
        &PipelineNode,
    )],
    tensors: &[(&Entity, &Tensor, Option<&Inputs>, Option<&Outputs>)],
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
        tensors,
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
    pipeline_nodes: &[(
        &Entity,
        &Name,
        Option<&Inputs>,
        Option<&Outputs>,
        &PipelineNode,
    )],
    tensors: &[(&Entity, &Tensor, Option<&Inputs>, Option<&Outputs>)],
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
    let pipeline = execute_pipeline(pipeline_nodes, tensors);

    quote! {
        #[no_mangle]
        pub extern "C" fn _manifest() -> i32 {
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
    pipeline_nodes: &[(
        &Entity,
        &Name,
        Option<&Inputs>,
        Option<&Outputs>,
        &PipelineNode,
    )],
    tensors: &[(&Entity, &Tensor, Option<&Inputs>, Option<&Outputs>)],
) -> TokenStream {
    let ExecutionOrder {
        order,
        tensor_names,
        pipeline_nodes,
        ..
    } = ExecutionOrder::calculate(pipeline_nodes, tensors);

    order
        .iter()
        .map(|entity| {
            execute_pipeline_node(
                entity,
                &pipeline_nodes,
                &tensor_names,
                tensors,
            )
        })
        .collect()
}

fn execute_pipeline_node(
    node: &Entity,
    pipeline_nodes: &HashMap<
        Entity,
        (&Name, Option<&Inputs>, Option<&Outputs>),
    >,
    tensor_names: &HashMap<Entity, Ident>,
    tensors: &[(&Entity, &Tensor, Option<&Inputs>, Option<&Outputs>)],
) -> TokenStream {
    let (name, inputs, outputs) = pipeline_nodes
        .get(node)
        .copied()
        .expect("This pipeline node always be present");

    match (inputs, outputs) {
        (Some(inputs), Some(outputs)) => execute_model_or_proc_block(
            name,
            inputs,
            outputs,
            tensor_names,
            tensors,
        ),
        (None, Some(outputs)) => {
            execute_capability(name, outputs, tensor_names, tensors)
        },
        (Some(inputs), None) => execute_output(name, inputs, tensor_names),
        (None, None) => {
            unreachable!(
                "The \"{}\" pipeline node should have inputs and/or outputs",
                name
            )
        },
    }
}

fn execute_output(
    name: &Name,
    inputs: &Inputs,
    tensor_names: &HashMap<Entity, Ident>,
) -> TokenStream {
    let name = Ident::new(name, Span::call_site());
    let inputs = input_bindings(&inputs.tensors, tensor_names);

    quote! {
        #name.consume(#inputs);
    }
}

fn execute_model_or_proc_block(
    name: &Name,
    inputs: &Inputs,
    outputs: &Outputs,
    tensor_names: &HashMap<Entity, Ident>,
    tensors: &[(&Entity, &Tensor, Option<&Inputs>, Option<&Outputs>)],
) -> TokenStream {
    let name = Ident::new(name, Span::call_site());
    let inputs = input_bindings(&inputs.tensors, tensor_names);
    let output_types = tensor_types(&outputs.tensors, tensors);
    let outputs = tensor_name_or_tuple(&outputs.tensors, tensor_names);

    quote! {
        let #outputs: #output_types = #name.transform(#inputs);
    }
}

fn input_bindings(
    tensors: &[Entity],
    tensor_names: &HashMap<Entity, Ident>,
) -> TokenStream {
    let names: Vec<_> = tensors.iter().map(|t| &tensor_names[t]).collect();

    match names.as_slice() {
        [] => unreachable!("Expected 1 or more tensors"),
        [tensor] => quote!(#tensor.clone()),
        names => quote!((#( #names.clone() ),*)),
    }
}

fn tensor_types<'world>(
    tensors: &[Entity],
    all_tensors: &[(&Entity, &Tensor, Option<&Inputs>, Option<&Outputs>)],
) -> TokenStream {
    let mut types = Vec::new();

    for ent in tensors {
        let (_, Tensor(shape), _, _) = all_tensors
            .iter()
            .copied()
            .find(|(e, _, _, _)| ent == *e)
            .unwrap();

        types.push(shape_to_tensor_type(shape));
    }

    match types.as_slice() {
        [single] => single.clone(),
        many => quote!((#(#many),*)),
    }
}

fn shape_to_tensor_type(shape: &Shape) -> TokenStream {
    let element_type = match shape.element_type() {
        &Type::u8 => quote!(u8),
        &Type::i8 => quote!(i8),
        &Type::u16 => quote!(u16),
        &Type::i16 => quote!(i16),
        &Type::u32 => quote!(u32),
        &Type::i32 => quote!(i32),
        &Type::f32 => quote!(f32),
        &Type::u64 => quote!(u64),
        &Type::i64 => quote!(i64),
        &Type::f64 => quote!(f64),
        &Type::str => quote!(&str),
        other => unreachable!("Unable to get the Rust name for {:?}", other),
    };
    quote!(Tensor<#element_type>)
}

fn tensor_name_or_tuple(
    tensors: &[Entity],
    tensor_names: &HashMap<Entity, Ident>,
) -> TokenStream {
    let names: Vec<_> = tensors.iter().map(|t| &tensor_names[t]).collect();

    match names.as_slice() {
        [] => unreachable!("Expected 1 or more tensors"),
        [tensor] => tensor.into_token_stream(),
        names => quote!((#(#names),*)),
    }
}

fn execute_capability(
    name: &Name,
    outputs: &Outputs,
    tensor_names: &HashMap<Entity, Ident>,
    tensors: &[(&Entity, &Tensor, Option<&Inputs>, Option<&Outputs>)],
) -> TokenStream {
    let name = Ident::new(name, Span::call_site());
    let output_types = tensor_types(&outputs.tensors, tensors);
    let outputs = tensor_name_or_tuple(&outputs.tensors, tensor_names);

    quote! { let #outputs: #output_types = #name.generate(); }
}

#[derive(Debug, Default)]
struct ExecutionOrder<'world> {
    order: Vec<Entity>,
    tensor_names: HashMap<Entity, Ident>,
    // internal bookkeeping
    visited_nodes: HashSet<Entity>,
    pipeline_nodes: HashMap<
        Entity,
        (
            &'world Name,
            Option<&'world Inputs>,
            Option<&'world Outputs>,
        ),
    >,
    tensors: &'world [(
        &'world Entity,
        &'world Tensor,
        Option<&'world Inputs>,
        Option<&'world Outputs>,
    )],
    tensor_inputs: HashMap<Entity, &'world [Entity]>,
}

impl<'world> ExecutionOrder<'world> {
    /// Given a set of pipeline nodes, determine the order they should be
    /// executed in and variable names for the various tensors involved.
    ///
    /// # Notes
    ///
    /// This assumes the pipeline nodes define a directed acyclic graph, and may
    /// not return if it contains cycles.
    ///
    /// This does [a topological sort][topo] using a modified depth-first
    /// search.
    ///
    /// [topo]: https://www.geeksforgeeks.org/topological-sorting/
    fn calculate(
        pipeline_nodes: &'world [(
            &'world Entity,
            &'world Name,
            Option<&'world Inputs>,
            Option<&'world Outputs>,
            &'world PipelineNode,
        )],
        tensors: &'world [(
            &'world Entity,
            &'world Tensor,
            Option<&'world Inputs>,
            Option<&'world Outputs>,
        )],
    ) -> Self {
        let mut order = ExecutionOrder {
            order: Vec::new(),
            tensor_names: HashMap::new(),
            visited_nodes: HashSet::new(),
            pipeline_nodes: pipeline_nodes
                .iter()
                .copied()
                .map(|(ent, name, inputs, outputs, _)| {
                    (*ent, (name, inputs, outputs))
                })
                .collect(),
            tensors,
            tensor_inputs: tensors
                .iter()
                .copied()
                .map(|(ent, _, inputs, _)| {
                    (
                        *ent,
                        inputs
                            .map(|i| i.tensors.as_slice())
                            .unwrap_or_default(),
                    )
                })
                .collect(),
        };

        for (entity, ..) in pipeline_nodes.iter().copied() {
            order.visit(*entity);
        }

        order
    }

    fn visit(&mut self, entity: Entity) {
        if self.visited_nodes.contains(&entity) {
            return;
        }

        self.visited_nodes.insert(entity);

        let (name, inputs, outputs) = self.pipeline_nodes[&entity];

        // We need to make sure all the inputs have been initialized first
        if let Some(inputs) = inputs {
            for input in &inputs.tensors {
                let previous_nodes =
                    self.tensor_inputs.get(input).copied().expect(
                        "All tensors must have a node that created them",
                    );
                for &previous_node in previous_nodes {
                    self.visit(previous_node);
                }
            }
        }

        // the pipeline node is executed
        self.order.push(entity);

        // and now it's been executed, we can mark each of its outputs as
        // available.
        if let Some(outputs) = outputs {
            for (i, tensor) in outputs.tensors.iter().enumerate() {
                let tensor_name = format!("{}_{}", name, i);
                self.tensor_names.insert(
                    *tensor,
                    Ident::new(&tensor_name, Span::call_site()),
                );
            }
        }
    }
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
        .map(|t| shape_to_tokens(&t.0));
    quote! { &[#(#inputs),*] }
}

fn shape_to_tokens(shape: &Shape<'_>) -> TokenStream {
    let rust_name = shape.element_type().rust_name().unwrap();
    let rust_name = Ident::new(rust_name, Span::call_site());
    let element_type = quote!(hotg_rune_core::reflect::Type::#rust_name);

    let dimensions = shape.dimensions();

    quote! {
        hotg_rune_core::Shape::new(
            #element_type,
            [ #(#dimensions),* ].as_ref(),
        )
    }
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
        let setter = format!("set_{}", key).replace("-", "_");
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
        let key = key.replace("-", "_");
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
        Value::String(ResourceOrString::String(s)) if s.starts_with("@") => {
            s[1..].parse().unwrap()
        },
        Value::String(ResourceOrString::String(s)) => s.into_token_stream(),
        Value::String(ResourceOrString::Resource(r)) => {
            let resource_name = Ident::new(r, Span::call_site());
            quote!(&*crate::resources::#resource_name)
        },
        Value::List(list) => {
            let tokens = list.iter().map(value_to_tokens);
            quote! { [ #(#tokens),* ].as_ref() }
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
        hotg_runicos_base_wasm::Resource::read_to_end(#name)
    };

    // We then take the Result and unwrap it, either falling back to a default
    // value (provided in the Runefile) or blowing up
    let bytes = match data {
        Some(default_value) => {
            let default_value = Literal::byte_string(default_value);
            quote!(#maybe_bytes.unwrap_or_else(|_| #default_value.as_ref().into()))
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
                pub(crate) static ref #ident: alloc::string::String = {
                    let bytes = #bytes;
                    core::str::from_utf8(&bytes).expect(#error_message).into()
                };
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
                pub(crate) static ref #name: &'static [u8] = include_bytes!(#path);
            }
        },
        ModelFile::Resource(resource) => {
            let resource_name = get_name(*resource).unwrap();
            let resource_name = Ident::new(resource_name, Span::call_site());

            quote! {
                pub(crate) static ref #name: &'static [u8] = crate::resources::#resource_name.as_ref();
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
    use legion::{Resources, World, IntoQuery};

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

    #[test]
    fn simple_linear_execution_order() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut cmd = CommandBuffer::new(&world);
        // manually add the first node
        let first_output = cmd.push((Tensor("f32[1]".parse().unwrap()),));
        let first = cmd.push((
            Name::from("first"),
            Outputs {
                tensors: vec![first_output],
            },
            PipelineNode,
        ));
        // add the second node
        let second_output = cmd.push((Tensor("f32[1]".parse().unwrap()),));
        let second = cmd.push((
            Name::from("second"),
            Inputs {
                tensors: vec![first_output],
            },
            Outputs {
                tensors: vec![second_output],
            },
            PipelineNode,
        ));
        // Add the third node
        let third = cmd.push((
            Name::from("third"),
            Inputs {
                tensors: vec![second_output],
            },
            PipelineNode,
        ));
        cmd.flush(&mut world, &mut resources);

        let pipeline_nodes: Vec<_> = <(
            Entity,
            &Name,
            Option<&Inputs>,
            Option<&Outputs>,
            &PipelineNode,
        )>::query()
        .iter(&world)
        .collect();
        let tensors: Vec<_> =
            <(Entity, &Tensor, Option<&Inputs>, Option<&Outputs>)>::query()
                .iter(&world)
                .collect();

        let ExecutionOrder {
            order,
            tensor_names,
            ..
        } = ExecutionOrder::calculate(&pipeline_nodes, &tensors);

        let order_should_be = vec![first, second, third];
        assert_eq!(order, order_should_be);
        let tensor_names_should_be: HashMap<_, _> = vec![
            (first_output, Ident::new("first_0", Span::call_site())),
            (second_output, Ident::new("second_0", Span::call_site())),
        ]
        .into_iter()
        .collect();
        assert_eq!(tensor_names, tensor_names_should_be);
    }

    #[test]
    fn execute_a_capability() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut cmd = CommandBuffer::new(&world);
        let first_output_tensor = Tensor("f32[1]".parse().unwrap());
        let first_output = cmd.push((first_output_tensor.clone(),));
        let name = Name::from("first");
        let outputs = Outputs {
            tensors: vec![first_output],
        };
        cmd.flush(&mut world, &mut resources);
        let tensor_names: HashMap<_, _> =
            vec![(first_output, Ident::new("first_0", Span::call_site()))]
                .into_iter()
                .collect();
        let tensors = &[(&first_output, &first_output_tensor, None, None)];

        let got = execute_capability(&name, &outputs, &tensor_names, tensors);

        let should_be = quote! {
            let first_0: Tensor<f32> = first.generate();
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn execute_model() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut cmd = CommandBuffer::new(&world);
        let model_output_tensor = Tensor("f32[1]".parse().unwrap());
        let model_output = cmd.push((model_output_tensor.clone(),));
        let model_input_tensor = Tensor("u8[1, 1, 1]".parse().unwrap());
        let model_input = cmd.push((model_input_tensor.clone(),));
        let name = Name::from("model");
        cmd.flush(&mut world, &mut resources);
        let inputs = Inputs {
            tensors: vec![model_input],
        };
        let outputs = Outputs {
            tensors: vec![model_output],
        };
        let tensor_names: HashMap<_, _> = vec![
            (model_output, Ident::new("model_output", Span::call_site())),
            (model_input, Ident::new("model_input", Span::call_site())),
        ]
        .into_iter()
        .collect();
        let tensors = &[
            (&model_output, &model_output_tensor, None, None),
            (&model_input, &model_input_tensor, None, None),
        ];

        let got = execute_model_or_proc_block(
            &name,
            &inputs,
            &outputs,
            &tensor_names,
            tensors,
        );

        let should_be = quote! {
            let model_output: Tensor<f32> = model.transform(model_input.clone());
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn consume_multiple_outputs() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut cmd = CommandBuffer::new(&world);
        let first_input_tensor =
            cmd.push((Tensor("u8[1, 1, 1]".parse().unwrap()),));
        let second_input_tensor =
            cmd.push((Tensor("f32[128]".parse().unwrap()),));
        let name = Name::from("serial");
        cmd.flush(&mut world, &mut resources);
        let inputs = Inputs {
            tensors: vec![first_input_tensor, second_input_tensor],
        };
        let tensor_names: HashMap<_, _> = vec![
            (
                first_input_tensor,
                Ident::new("first_input", Span::call_site()),
            ),
            (
                second_input_tensor,
                Ident::new("second_input", Span::call_site()),
            ),
        ]
        .into_iter()
        .collect();

        let got = execute_output(&name, &inputs, &tensor_names);

        let should_be = quote! {
            serial.consume((first_input.clone(), second_input.clone()));
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn tensor_shapes_as_rust_types() {
        let inputs = vec![
            ("f32[1]", quote!(Tensor<f32>)),
            ("u8[1, 2, 3, 4]", quote!(Tensor<u8>)),
            ("str[42]", quote!(Tensor<&str>)),
        ];

        for (shape, should_be) in inputs {
            let shape: Shape<'_> = shape.parse().unwrap();
            let got = shape_to_tensor_type(&shape);
            assert_eq!(
                got.to_string().replace(" ", ""),
                should_be.to_string().replace(" ", "")
            );
        }
    }
}
