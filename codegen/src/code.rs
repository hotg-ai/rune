use std::collections::HashMap;

use anyhow::Error;
use heck::{CamelCase, SnakeCase};
use quote::{ToTokens, TokenStreamExt, quote};
use proc_macro2::{Ident, Span, TokenStream};
use rune_syntax::{
    ast::{ArgumentValue, Literal, LiteralKind},
    hir::{
        HirId, Node, ProcBlock, Rune, Sink, SinkKind, Source, SourceKind, Stage,
    },
};

/// Generate the Rune's `lib.rs` file.
pub fn generate(rune: &Rune) -> Result<String, Error> {
    let preamble = preamble();
    let manifest = manifest_function(rune);
    let call = call();

    let tokens = quote! {
        #preamble
        #manifest
        #call
    };

    Ok(tokens.to_token_stream().to_string())
}

fn manifest_function(rune: &Rune) -> impl ToTokens {
    let sorted_pipeline: Vec<_> = rune.sorted_pipeline().collect();

    let initialized_node = sorted_pipeline
        .iter()
        .copied()
        .map(|(id, node)| initialize_node(rune, id, node));

    quote! {
        #[no_mangle]
        pub extern "C" fn _manifest() -> u32 {
            let _setup = SetupGuard::default();

            #( #initialized_node )*

            let pipeline = move || {
                let _guard = PipelineGuard::default();
            };

            unsafe {
                PIPELINE = Some(Box::new(pipeline));
            }

            1
        }
    }
}

fn initialize_node(rune: &Rune, id: HirId, node: &Node) -> impl ToTokens {
    let name = &rune.names[id];
    let name = Ident::new(name, Span::call_site());

    match &node.stage {
        Stage::Source(Source { kind, parameters }) => {
            initialize_source(name, kind, parameters)
        },
        Stage::Sink(Sink { kind }) => {
            let type_name = sink_type_name(kind);

            quote! {
                let mut #name = #type_name::default();
            }
        },
        Stage::Model(_) => {
            let model_file = format!("{}.tflite", name);
            quote! {
                let mut #name = Model::load(include_bytes!(#model_file));
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

fn sink_type_name(kind: &SinkKind) -> TokenStream {
    match kind {
        SinkKind::Serial => quote!(runic_types::wasm32::Serial),
        SinkKind::Other(other) => ident(other),
    }
}

fn initialize_source(
    name: Ident,
    kind: &SourceKind,
    parameters: &HashMap<String, ArgumentValue>,
) -> TokenStream {
    let type_name = source_type_name(kind);
    let setters = parameters.iter().map(|(key, value)| {
        let arg = quote_value(value);
        quote! { #name.set_parameter(#key, #arg); }
    });

    quote! {
        let mut #name = #type_name::default();
        #( #setters )*
    }
}

fn quote_value(value: &ArgumentValue) -> TokenStream {
    match value {
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::Integer(i),
            ..
        }) => quote!(#i),
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::Float(f),
            ..
        }) => quote!(#f),
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::String(s),
            ..
        }) if s.starts_with("@") => todo!("Verbatim Rust"),
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::String(s),
            ..
        }) => quote!(#s),
        ArgumentValue::List(strings) => quote!([ #(#strings),* ]),
    }
}

fn source_type_name(kind: &SourceKind) -> TokenStream {
    let name = match kind {
        SourceKind::Random => "runic_types::wasm32::Random",
        SourceKind::Accelerometer => "runic_types::wasm32::Accelerometer",
        SourceKind::Sound => "runic_types::wasm32::Sound",
        SourceKind::Image => "runic_types::wasm32::Image",
        SourceKind::Raw => "runic_types::wasm32::Raw",
        SourceKind::Other(other) => other.as_str(),
    };

    ident(name)
}

fn ident(fully_qualified_path: &str) -> TokenStream {
    let segments = fully_qualified_path
        .split("::")
        .map(|segment| Ident::new(segment, Span::call_site()));

    let mut tokens = TokenStream::new();
    tokens.append_separated(segments, quote! {::});
    tokens
}

fn preamble() -> impl ToTokens {
    quote! {
        //! Automatically generated by rune. DO NOT EDIT!

        #![no_std]
        #![feature(alloc_error_handler)]
        #![allow(unused_imports, dead_code)]

        extern crate alloc;

        use runic_types::{*, wasm32::*};
        use alloc::boxed::Box;

        static mut PIPELINE: Option<Box<dyn FnMut()>> = None;
    }
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

        pipeline:
          audio:
            capability: SOUND
            args:
              hz: 16000
        "#,
        );

        let got = manifest_function(&rune).to_token_stream();

        let should_be = quote! {
            #[no_mangle]
            pub extern "C" fn _manifest() -> u32 {
                let _setup = SetupGuard::default();
                let mut audio = runic_types::wasm32::Sound::default();
                audio.set_parameter("hz", 16000i64);

                let pipeline = move || {
                    let _guard = PipelineGuard::default();
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

        pipeline:
          audio:
            capability: SOUND
            args:
              hz: 16000
        "#,
        );
        let id = rune.names["audio"];
        let node = &rune.stages[&id];

        let got = initialize_node(&rune, id, node).to_token_stream();

        let should_be = quote! {
            let mut audio = runic_types::wasm32::Sound::default();
            audio.set_parameter("hz", 16000i64);
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn initialize_model() {
        let rune = rune(
            r#"
        image: runicos/base

        pipeline:
          sine:
            model: ./sine_model.tflite
        "#,
        );
        let id = rune.names["sine"];
        let node = &rune.stages[&id];

        let got = initialize_node(&rune, id, node).to_token_stream();

        let should_be = quote! {
            let mut sine = Model::load(include_bytes!("sine.tflite"));
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn initialize_outputs() {
        let rune = rune(
            r#"
        image: runicos/base

        pipeline:
          serial:
            out: SERIAL
        "#,
        );
        let id = rune.names["serial"];
        let node = &rune.stages[&id];

        let got = initialize_node(&rune, id, node).to_token_stream();

        let should_be = quote! {
            let mut serial = runic_types::wasm32::Serial::default();
        };
        assert_quote_eq!(got, should_be);
    }

    #[test]
    fn initialize_proc_block() {
        let rune = rune(
            r#"
        image: runicos/base

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

        let got = initialize_node(&rune, id, node).to_token_stream();

        let should_be = quote! {
            let mut normalize = label::Label::default();
            normalize.set_labels(["silence", "unknown", "up", "down", "left", "right"]);
        };
        assert_quote_eq!(got, should_be);
    }
}
