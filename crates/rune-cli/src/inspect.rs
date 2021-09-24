use std::{path::PathBuf};
use anyhow::{Context, Error};
use hotg_rune_compiler::codegen::{RuneGraph, RuneVersion};
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
    if let Some(version) = &meta.version {
        print!("Compiled by: Rune {}", version);
    }

    if let Some(simplified_rune) = &meta.rune {
        todo!();
    }
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub(crate) struct Metadata {
    version: Option<RuneVersion>,
    rune: Option<RuneGraph>,
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
