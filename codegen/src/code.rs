use std::collections::HashMap;
use anyhow::{Error, Context};
use heck::{CamelCase, SnakeCase};
use quote::{ToTokens, TokenStreamExt, quote};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use rune_syntax::{
    hir::{
        HirId, Node, Primitive, ProcBlock, Rune, Sink, SinkKind, Slot, Source,
        SourceKind, Stage, Type,
    },
    yaml::Value,
};
use serde::Serialize;

/// Generate the Rune's `lib.rs` file.
pub fn generate(
    rune: &Rune,
    build_info: Option<serde_json::Value>,
) -> Result<String, Error> {
    let image_crate = quote!(runicos_base_wasm);
    let preamble = preamble(rune, build_info)?;
    let manifest = manifest_function(rune, &image_crate);
    let call = call();

    let tokens = quote! {
        #preamble
        #manifest
        #call
    };

    Ok(tokens.to_token_stream().to_string())
}

fn manifest_function(rune: &Rune, image_crate: &TokenStream) -> impl ToTokens {
    let sorted_pipeline: Vec<_> = rune.sorted_pipeline().collect();

    let initialized_node = sorted_pipeline
        .iter()
        .copied()
        .map(|(id, node)| initialize_node(rune, id, node, image_crate));

    let set_output_dimensions = sorted_pipeline
        .iter()
        .copied()
        .map(|(id, node)| set_output_dimensions(rune, id, node));

    let transform = sorted_pipeline
        .iter()
        .copied()
        .map(|(id, node)| evaluate_node(rune, id, node));

    quote! {
        #[no_mangle]
        pub extern "C" fn _manifest() -> u32 {
            let _setup = #image_crate::SetupGuard::default();

            #( #initialized_node )*

            #( #set_output_dimensions )*

            let pipeline = move || {
                let _guard = #image_crate::PipelineGuard::default();

                #( #transform )*
            };

            unsafe {
                PIPELINE = Some(Box::new(pipeline));
            }

            1
        }
    }
}

fn set_output_dimensions(
    rune: &Rune,
    node_id: HirId,
    node: &Node,
) -> Option<TokenStream> {
    let output_slot_id = match *node.output_slots {
        [] => return None,
        [output] => output,
        // TODO: Create a mechanism for notifying a pipeline stage about
        // multiple outputs. A proper solution will probably require a
        // more sophisticated "reflection" system.
        [..] => return None,
    };

    let name = Ident::new(&rune.names[node_id], Span::call_site());
    let slot = &rune.slots[&output_slot_id];

    if let Type::Buffer { dimensions, .. } = &rune.types[&slot.element_type] {
        Some(quote! {
            #name.set_output_dimensions(&[ #(#dimensions),* ]);
        })
    } else {
        None
    }
}

fn evaluate_node(rune: &Rune, id: HirId, node: &Node) -> TokenStream {
    match node.stage {
        Stage::Source(_) => evaluate_source_node(rune, id, &node.output_slots),
        Stage::Sink(_) => evaluate_sink_node(rune, id, &node.input_slots),
        Stage::Model(_) | Stage::ProcBlock(_) => evaluate_transform_node(
            rune,
            id,
            &node.input_slots,
            &node.output_slots,
        ),
    }
}

fn evaluate_transform_node(
    rune: &Rune,
    node_id: HirId,
    input_slots: &[HirId],
    output_slots: &[HirId],
) -> TokenStream {
    let name = &rune.names[node_id];
    let output_bindings = output_bindings(rune, name, output_slots);
    let input_bindings = input_bindings(rune, input_slots);
    let name = Ident::new(name, Span::call_site());

    quote! {
        let #output_bindings = #name.transform(#input_bindings);
    }
}

fn evaluate_sink_node(
    rune: &Rune,
    node_id: HirId,
    input_slots: &[HirId],
) -> TokenStream {
    let name = &rune.names[node_id];
    let input_bindings = input_bindings(rune, input_slots);
    let name = Ident::new(name, Span::call_site());

    quote! {
        #name.consume(#input_bindings);
    }
}

fn evaluate_source_node(
    rune: &Rune,
    node_id: HirId,
    output_slots: &[HirId],
) -> TokenStream {
    let name = &rune.names[node_id];
    let rets = output_bindings(rune, name, output_slots);
    let name = Ident::new(name, Span::call_site());

    quote! {
        let #rets = #name.generate();
    }
}

fn output_bindings(
    rune: &Rune,
    name: &str,
    output_slots: &[HirId],
) -> TokenStream {
    let return_values: Vec<_> = output_slots
        .iter()
        .enumerate()
        .map(|(i, id)| slot_type(rune, name, *id, i))
        .collect();

    match return_values.as_slice() {
        [(name, ty)] => quote!(#name : #ty),
        [] => unreachable!(),
        [..] => {
            let names = return_values.iter().map(|(name, _)| name);
            let types = return_values.iter().map(|(_, ty)| ty);
            quote! {
                (#(#names),*) : (#(#types),*)
            }
        },
    }
}

fn input_bindings(rune: &Rune, input_slots: &[HirId]) -> TokenStream {
    let input_names: Vec<_> = input_slots
        .iter()
        .map(|id| {
            let slot = &rune.slots[id];
            let input = slot.input_node;
            let input_node = &rune.stages[&input];
            let input_name = rune.names.get_name(input).unwrap();
            let index = input_node
                .output_slots
                .iter()
                .position(|s| s == id)
                .unwrap();

            let name = format!("{}_out_{}", input_name, index);
            let ident = Ident::new(&name, Span::call_site());

            // TODO: be smart and only add a clone() call when this slot is used
            // multiple times

            quote!(#ident.clone())
        })
        .collect();

    match input_names.as_slice() {
        [name] => quote!(#name),
        [] => unreachable!(),
        [..] => {
            quote! {
                (#(#input_names),*)
            }
        },
    }
}

fn slot_type(
    rune: &Rune,
    node_name: &str,
    slot: HirId,
    index: usize,
) -> (Ident, TokenStream) {
    let name = format!("{}_out_{}", node_name, index);
    let name = Ident::new(&name, Span::call_site());

    let Slot { element_type, .. } = &rune.slots[&slot];

    (name, rust_type(element_type, rune))
}

fn rust_type(element_type: &HirId, rune: &Rune) -> TokenStream {
    match &rune.types[element_type] {
        Type::Primitive(Primitive::U8) => quote!(u8),
        Type::Primitive(Primitive::I8) => quote!(i8),
        Type::Primitive(Primitive::U16) => quote!(u16),
        Type::Primitive(Primitive::I16) => quote!(i16),
        Type::Primitive(Primitive::U32) => quote!(u32),
        Type::Primitive(Primitive::I32) => quote!(i32),
        Type::Primitive(Primitive::F32) => quote!(f32),
        Type::Primitive(Primitive::U64) => quote!(u64),
        Type::Primitive(Primitive::I64) => quote!(i64),
        Type::Primitive(Primitive::F64) => quote!(f64),
        Type::Primitive(Primitive::String) => quote!(&str),
        Type::Buffer { underlying_type, .. } => {
            let underlying_type = rust_type(underlying_type, rune);
            quote!(Tensor<#underlying_type>)
        },
        Type::Unknown |
        Type::Any => unreachable!("The parsing and type checking phase should have resolved all types"),
    }
}

fn initialize_node(
    rune: &Rune,
    id: HirId,
    node: &Node,
    image_crate: &TokenStream,
) -> TokenStream {
    let name = rune.names[id].replace("-", "_");
    let name = Ident::new(&name, Span::call_site());

    match &node.stage {
        Stage::Source(Source { kind, parameters }) => {
            initialize_source(name, kind, parameters, image_crate)
        },
        Stage::Sink(Sink { kind }) => {
            let type_name = sink_type_name(kind, image_crate);

            quote! {
                let mut #name = #type_name::default();
            }
        },
        Stage::Model(_) => {
            let model_file = format!("{}.tflite", name);
            quote! {
                let mut #name = #image_crate::Model::load(include_bytes!(#model_file));
            }
        },
        Stage::ProcBlock(proc_block) => initialize_proc_block(name, proc_block),
    }
}

fn initialize_proc_block(name: Ident, proc_block: &ProcBlock) -> TokenStream {
    let type_name = proc_block_type_name(proc_block);

    let setters = proc_block.parameters.iter().map(|(key, value)| {
        let arg = quote_value(value);
        let setter = format!("set_{}", key);
        let setter = Ident::new(&setter, Span::call_site());
        quote! { #name.#setter(#arg); }
    });

    quote! {
        let mut #name = #type_name::default();

        #( #setters )*
    }
}

fn proc_block_type_name(proc_block: &ProcBlock) -> TokenStream {
    let module_name = proc_block.name().to_snake_case();
    let type_name = module_name.to_camel_case();

    let module_name = Ident::new(&module_name, Span::call_site());
    let type_name = Ident::new(&type_name, Span::call_site());

    quote!(#module_name::#type_name)
}

fn sink_type_name(kind: &SinkKind, image_crate: &TokenStream) -> TokenStream {
    match kind {
        SinkKind::Serial => quote!(#image_crate::Serial),
        SinkKind::Other(other) => ident(other),
    }
}

fn initialize_source(
    name: Ident,
    kind: &SourceKind,
    parameters: &HashMap<String, Value>,
    image_crate: &TokenStream,
) -> TokenStream {
    let type_name = source_type_name(kind, image_crate);
    let setters = parameters.iter().map(|(key, value)| {
        let arg = quote_value(value);
        quote! { #name.set_parameter(#key, #arg); }
    });

    quote! {
        let mut #name = #type_name::default();
        #( #setters )*
    }
}

fn quote_value(value: &Value) -> TokenStream {
    match value {
        Value::Int(i) => quote!(#i),
        Value::Float(f) => quote!(#f),
        Value::String(s) if s.starts_with("@") => {
            let rust_code = &s[1..];
            match rust_code.parse() {
                Ok(tokens) => tokens,
                // TODO: validate this as part of rune-syntax so users can't
                // make `rune build` blow up
                Err(e) => panic!(
                    "Unable to parse \"{}\" as valid Rust: {}",
                    rust_code, e
                ),
            }
        },
        Value::String(s) => quote!(#s),
        Value::List(list) => {
            let values = list.iter().map(quote_value);
            quote!([#(#values),*])
        },
    }
}

fn source_type_name(
    kind: &SourceKind,
    image_crate: &TokenStream,
) -> TokenStream {
    match kind {
        SourceKind::Random => quote!(#image_crate::Random),
        SourceKind::Accelerometer => quote!(#image_crate::Accelerometer),
        SourceKind::Sound => quote!(#image_crate::Sound),
        SourceKind::Image => quote!(#image_crate::Image),
        SourceKind::Raw => quote!(#image_crate::Raw),
        SourceKind::Other(other) => ident(other),
    }
}

fn ident(fully_qualified_path: &str) -> TokenStream {
    let segments = fully_qualified_path
        .split("::")
        .map(|segment| Ident::new(segment, Span::call_site()));

    let mut tokens = TokenStream::new();
    tokens.append_separated(segments, quote! {::});
    tokens
}

fn custom_section(
    section_name: &str,
    global: &str,
    value: &impl Serialize,
) -> Result<TokenStream, Error> {
    let jsonified = serde_json::to_string(value)
        .context("Unable to serialize the Rune to JSON")?;
    let global = Ident::new(global, Span::call_site());
    let len = jsonified.len();
    let data = Literal::byte_string(jsonified.as_bytes());

    Ok(quote! {
        #[link_section = #section_name]
        static #global: [u8; #len] = *#data;
    })
}

fn preamble(
    rune: &Rune,
    build_info: Option<serde_json::Value>,
) -> Result<TokenStream, Error> {
    let rune_graph =
        custom_section(crate::GRAPH_CUSTOM_SECTION, "RUNE_GRAPH", rune)?;
    let build_info = match build_info {
        Some(v) => {
            custom_section(crate::VERSION_CUSTOM_SECTION, "RUNE_VERSION", &v)?
        },
        None => quote!(),
    };

    Ok(quote! {
        //! Automatically generated by rune. DO NOT EDIT!

        #![no_std]
        #![feature(alloc_error_handler)]
        #![allow(unused_imports, dead_code)]

        extern crate alloc;

        use alloc::boxed::Box;
        use runic_types::*;

        #rune_graph
        #build_info

        static mut PIPELINE: Option<Box<dyn FnMut()>> = None;
    })
}

fn call() -> impl ToTokens {
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

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        process::{Command, Stdio},
    };
    use rune_syntax::{Diagnostics, yaml::Document};
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

    fn rune(doc: &str) -> Rune {
        let doc = Document::parse(doc).unwrap();
        let mut diags = Diagnostics::new();
        let rune = rune_syntax::analyse_yaml_runefile(&doc, &mut diags);
        assert!(diags.is_empty(), "{:#?}", diags);

        rune
    }

    #[test]
    fn single_item_pipeline() {
        let rune = rune(
            r#"
        image: runicos/base
        version: 1

        pipeline:
          audio:
            capability: SOUND
            args:
              hz: 16000
            outputs:
            - type: u8
              dimensions: [18000]
        "#,
        );
        let image_crate = quote!(runicos_base_wasm);

        let got = manifest_function(&rune, &image_crate).to_token_stream();

        let should_be = quote! {
            #[no_mangle]
            pub extern "C" fn _manifest() -> u32 {
                let _setup = runicos_base_wasm::SetupGuard::default();
                let mut audio = runicos_base_wasm::Sound::default();
                audio.set_parameter("hz", 16000i32);

                audio.set_output_dimensions(&[18000usize]);

                let pipeline = move || {
                    let _guard = runicos_base_wasm::PipelineGuard::default();
                    let audio_out_0: Tensor<u8> = audio.generate();
                };

                unsafe {
                    PIPELINE = Some(Box::new(pipeline));
                }
                1
            }
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn initialize_audio_capability() {
        let rune = rune(
            r#"
        image: runicos/base
        version: 1

        pipeline:
          audio:
            capability: SOUND
            args:
              hz: 16000
        "#,
        );
        let id = rune.names["audio"];
        let node = &rune.stages[&id];
        let image_crate = quote!(runicos_base_wasm);

        let got =
            initialize_node(&rune, id, node, &image_crate).to_token_stream();

        let should_be = quote! {
            let mut audio = runicos_base_wasm::Sound::default();
            audio.set_parameter("hz", 16000i32);
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn initialize_model() {
        let rune = rune(
            r#"
        image: runicos/base
        version: 1

        pipeline:
          sine:
            model: ./sine_model.tflite
        "#,
        );
        let id = rune.names["sine"];
        let node = &rune.stages[&id];
        let image_crate = quote!(runicos_base_wasm);

        let got =
            initialize_node(&rune, id, node, &image_crate).to_token_stream();

        let should_be = quote! {
            let mut sine = runicos_base_wasm::Model::load(include_bytes!("sine.tflite"));
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn initialize_outputs() {
        let rune = rune(
            r#"
        image: runicos/base
        version: 1

        pipeline:
          serial:
            out: SERIAL
        "#,
        );
        let id = rune.names["serial"];
        let node = &rune.stages[&id];
        let image_crate = quote!(runicos_base_wasm);

        let got =
            initialize_node(&rune, id, node, &image_crate).to_token_stream();

        let should_be = quote! {
            let mut serial = runicos_base_wasm::Serial::default();
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn initialize_proc_block() {
        let rune = rune(
            r#"
        image: runicos/base
        version: 1

        pipeline:
          audio:
            capability: SOUND
            outputs:
              - type: u8
                dimensions: [18000]
          normalize:
            proc-block: hotg-ai/rune#proc_blocks/label
            inputs:
              - audio
            args:
              labels:
                - silence
                - unknown
                - up
                - down
                - left
                - right
        "#,
        );
        let id = rune.names["normalize"];
        let node = &rune.stages[&id];
        let image_crate = quote!(runicos_base_wasm);

        let got =
            initialize_node(&rune, id, node, &image_crate).to_token_stream();

        let should_be = quote! {
            let mut normalize = label::Label::default();
            normalize.set_labels(["silence", "unknown", "up", "down", "left", "right"]);
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn evaluate_audio_capability() {
        let rune = rune(
            r#"
        image: runicos/base
        version: 1

        pipeline:
          audio:
            capability: SOUND
            args:
              hz: 16000
            outputs:
            - type: u8
              dimensions: [18000]
        "#,
        );
        let id = rune.names["audio"];
        let node = &rune.stages[&id];

        let got = evaluate_node(&rune, id, &node).to_token_stream();

        let should_be = quote! {
            let audio_out_0: Tensor<u8> = audio.generate();
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn evaluate_capability_with_multiple_outputs() {
        let rune = rune(
            r#"
        image: runicos/base
        version: 1

        pipeline:
          audio:
            capability: SOUND
            args:
              hz: 16000
            outputs:
            - type: u8
              dimensions: [18000]
            - type: u8
              dimensions: [18000]
        "#,
        );
        let id = rune.names["audio"];
        let node = &rune.stages[&id];

        let got = evaluate_node(&rune, id, &node).to_token_stream();

        let should_be = quote! {
            let (audio_out_0, audio_out_1): (Tensor<u8>, Tensor<u8>) = audio.generate();
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn evaluate_sink_with_single_input() {
        let rune = rune(
            r#"
        image: runicos/base
        version: 1

        pipeline:
          audio:
            capability: SOUND
            args:
              hz: 16000
            outputs:
            - type: u8
              dimensions: [18000]
          debug:
            out: SERIAL
            inputs:
              - audio
        "#,
        );
        let id = rune.names["debug"];
        let node = &rune.stages[&id];

        let got = evaluate_node(&rune, id, &node).to_token_stream();

        let should_be = quote! {
            debug.consume(audio_out_0.clone());
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn evaluate_sink_with_multiple_inputs() {
        let rune = rune(
            r#"
        image: runicos/base
        version: 1

        pipeline:
          audio:
            capability: SOUND
            args:
              hz: 16000
            outputs:
            - type: u8
              dimensions: [18000]
            - type: f32
              dimensions: [8]
          debug:
            out: SERIAL
            inputs:
              - audio.0
              - audio.1
        "#,
        );
        let id = rune.names["debug"];
        let node = &rune.stages[&id];

        let got = evaluate_node(&rune, id, &node).to_token_stream();

        let should_be = quote! {
            debug.consume((audio_out_0.clone(), audio_out_1.clone()));
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn evaluate_model_with_single_input_and_output() {
        let rune = rune(
            r#"
        image: runicos/base
        version: 1

        pipeline:
          audio:
            capability: SOUND
            outputs:
            - type: u8
              dimensions: [18000]
          sine:
            model: ./sine.tflite
            inputs:
            - audio
            outputs:
              - type: u8
                dimensions: [1]
          debug:
            out: SERIAL
            inputs:
              - sine
        "#,
        );
        let id = rune.names["sine"];
        let node = &rune.stages[&id];

        let got = evaluate_node(&rune, id, &node).to_token_stream();

        let should_be = quote! {
            let sine_out_0: Tensor<u8> = sine.transform(audio_out_0.clone());
        };
        assert_quote_eq!(got, should_be);
    }
}
