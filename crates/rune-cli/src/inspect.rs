use std::{collections::HashMap, path::PathBuf};
use anyhow::{Context, Error};
use hotg_rune_compiler::{
    codegen::{
        CapabilitySummary, ModelSummary, OutputSummary, ProcBlockSummary,
        RuneGraph, RuneVersion, TensorId,
    },
    lowering::{Name, Resource, },
    parse::{ResourceType, ResourceOrString},
};
use hotg_rune_core::Shape;
use strum::VariantNames;
use wasmparser::{BinaryReaderError, Parser, Payload};
use crate::Format;

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Inspect {
    #[structopt(
        short,
        long,
        help = "The format to use when printing output",
        default_value = "text",
        possible_values = Format::VARIANTS,
        parse(try_from_str)
    )]
    format: Format,
    #[structopt(help = "The Rune to inspect", parse(from_os_str))]
    rune: PathBuf,
}

impl Inspect {
    pub fn execute(self) -> Result<(), Error> {
        let wasm = std::fs::read(&self.rune).with_context(|| {
            format!("Unable to read \"{}\"", self.rune.display())
        })?;
        let meta = Metadata::from_wasm_binary(&wasm)
            .context("Unable to parse metadata from the WebAssembly module")?;

        match self.format {
            Format::Json => {
                let s = serde_json::to_string_pretty(&meta)
                    .context("Unable to format the metadata as JSON")?;
                println!("{}", s);
            },
            Format::Text => print_meta(&meta),
        }

        Ok(())
    }
}

fn print_meta(meta: &Metadata) {
    if let Some(rune) = &meta.rune {
        print_rune(rune);
    }

    if let Some(version) = &meta.version {
        println!("Compiled by: Rune {}", version);
    }
}

fn print_rune(rune: &RuneGraph) {
    let RuneGraph {
        rune,
        capabilities,
        models,
        proc_blocks,
        outputs,
        resources,
        tensors,
    } = rune;

    println!("Name: {}", rune.name);

    print_capabilities(capabilities, tensors);
    print_models(models, tensors);
    print_proc_blocks(proc_blocks, tensors);
    print_outputs(outputs, tensors);
    print_resources(resources);
}

fn print_outputs(
    outputs: &HashMap<Name, OutputSummary>,
    tensors: &HashMap<TensorId, Shape<'static>>,
) {
    if outputs.is_empty() {
        return;
    }

    println!("Outputs:");

    for (name, output) in outputs {
        println!("- {}: {}", name, output.kind);
        print_tensors("Inputs", &output.inputs, tensors);
    }
}

fn print_resources(resources: &HashMap<Name, Resource>) {
    if resources.is_empty() {
        return;
    }

    println!("Resources:");

    for (name, resource) in resources {
        match resource.ty {
            ResourceType::String => println!("\t{} (string)", name),
            ResourceType::Binary => println!("\t{} (binary)", name),
        }
    }
}

fn print_models(
    models: &HashMap<Name, ModelSummary>,
    tensors: &HashMap<TensorId, Shape<'static>>,
) {
    if models.is_empty() {
        return;
    }

    println!("Models:");

    for (name, model) in models {
        println!("- {}: {}", name, model.file);
        print_tensors("Inputs", &model.inputs, tensors);
        print_tensors("Outputs", &model.outputs, tensors);
    }
}

fn print_proc_blocks(
    proc_blocks: &HashMap<Name, ProcBlockSummary>,
    tensors: &HashMap<TensorId, Shape<'static>>,
) {
    if proc_blocks.is_empty() {
        return;
    }

    println!("Proc Blocks:");

    for (name, proc_block) in proc_blocks {
        println!("- {}: {}", name, proc_block.path);
        print_tensors("Inputs", &proc_block.inputs, tensors);
        print_tensors("Outputs", &proc_block.outputs, tensors);
        print_args(&proc_block.args);
    }
}

fn print_capabilities(
    capabilities: &HashMap<Name, CapabilitySummary>,
    tensors: &HashMap<TensorId, Shape<'static>>,
) {
    if !capabilities.is_empty() {
        return;
    }

    println!("Capabilities:");

    for (name, cap) in capabilities {
        println!("- {}: {}", name, cap.kind);
        print_tensors("Outputs", &cap.outputs, tensors);
        print_args(&cap.args);
    }
}

fn print_tensors(
    name: &str,
    tensors: &[TensorId],
    tensor_shapes: &HashMap<TensorId, Shape<'static>>,
) {
    println!("  {}:", name);

    for tensor in tensors {
        println!("  - {}", tensor_shapes[tensor]);
    }
}

fn print_args(args: &HashMap<String, ResourceOrString>) {
    if !args.is_empty() {
        println!("  Arguments:");

        for (arg, value) in args {
            print!("    {}: {}", arg, value);
        }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub(crate) struct Metadata {
    pub(crate) version: Option<RuneVersion>,
    pub(crate) rune: Option<RuneGraph>,
}

impl Metadata {
    pub(crate) fn from_wasm_binary(
        wasm: &[u8],
    ) -> Result<Self, BinaryReaderError> {
        wasm_custom_sections(wasm).map(Metadata::from_custom_sections)
    }

    fn from_custom_sections<'a>(
        sections: impl IntoIterator<Item = CustomSection<'a>>,
    ) -> Self {
        let mut meta = Metadata::default();

        for section in sections {
            match section.name {
                hotg_rune_compiler::codegen::GRAPH_CUSTOM_SECTION => {
                    match serde_json::from_slice(section.data) {
                        Ok(rune) => {
                            meta.rune = Some(rune);
                        },
                        Err(e) => {
                            log::warn!(
                                "Unable to deserialize the Rune graph: {}",
                                e
                            );
                        },
                    }
                },
                hotg_rune_compiler::codegen::VERSION_CUSTOM_SECTION => {
                    match serde_json::from_slice(section.data) {
                        Ok(v) => {
                            meta.version = Some(v);
                        },
                        Err(e) => {
                            log::warn!(
                                "Unable to deserialize the version: {}",
                                e
                            );
                        },
                    }
                },
                _ => {},
            }
        }

        meta
    }
}

pub(crate) fn wasm_custom_sections(
    wasm: &[u8],
) -> Result<Vec<CustomSection<'_>>, BinaryReaderError> {
    let mut sections = Vec::new();

    for payload in Parser::default().parse_all(wasm) {
        if let Payload::CustomSection { name, data, .. } = payload? {
            sections.push(CustomSection { name, data });
        }
    }

    Ok(sections)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct CustomSection<'a> {
    pub(crate) name: &'a str,
    pub(crate) data: &'a [u8],
}
