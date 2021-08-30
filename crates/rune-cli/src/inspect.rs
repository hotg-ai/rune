use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
};
use anyhow::{Context, Error};
use build_info::BuildInfo;
use hotg_rune_syntax::{
    hir::{HirId, Rune, SourceKind},
    yaml::{Type, Value},
};
use serde::{Serialize, Serializer};
use strum::VariantNames;
use wasmparser::{Parser, Payload};
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
        let meta = Metadata::from_wasm_binary(&wasm);

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
    if let Some(build_info) = &meta.rune_cli_build_info {
        let git = build_info
            .version_control
            .as_ref()
            .expect("The project uses version control")
            .git()
            .expect("The project uses git");

        println!(
            "Compiled by: {} v{} ({} {})",
            build_info.crate_info.name,
            build_info.crate_info.version,
            git.commit_short_id,
            git.commit_timestamp.date().naive_utc(),
        );
    }

    if let Some(SimplifiedRune { capabilities }) = &meta.simplified_rune {
        if !capabilities.is_empty() {
            print_capabilities(&capabilities);
        }
    }
}

fn print_capabilities(capabilities: &BTreeMap<String, SimplifiedCapability>) {
    println!("Capabilities:");

    for (name, value) in capabilities {
        let SimplifiedCapability {
            capability_type,
            outputs,
            parameters,
        } = value;
        println!("  {} ({})", name, capability_type);

        if !outputs.is_empty() {
            println!("    Outputs:");
            for output in outputs {
                println!("    - {}{:?}", output.name, output.dimensions);
            }
        }

        if !parameters.is_empty() {
            println!("    Parameters:");
            for (key, value) in parameters {
                println!("    - {}: {:?}", key, value);
            }
        }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub(crate) struct Metadata {
    rune_cli_build_info: Option<BuildInfo>,
    #[serde(skip)]
    rune: Option<Rune>,
    simplified_rune: Option<SimplifiedRune>,
}

impl Metadata {
    pub(crate) fn from_wasm_binary(wasm: &[u8]) -> Self {
        Metadata::from_custom_sections(wasm_custom_sections(wasm))
    }

    fn from_custom_sections<'a>(
        sections: impl Iterator<Item = CustomSection<'a>>,
    ) -> Self {
        let mut meta = Metadata::default();

        for section in sections {
            match section.name {
                hotg_rune_codegen::GRAPH_CUSTOM_SECTION => {
                    match serde_json::from_slice(section.data) {
                        Ok(rune) => {
                            meta.simplified_rune =
                                Some(SimplifiedRune::from_rune(&rune));
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
                hotg_rune_codegen::VERSION_CUSTOM_SECTION => {
                    match serde_json::from_slice(section.data) {
                        Ok(v) => {
                            meta.rune_cli_build_info = Some(v);
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

    pub(crate) fn take_rune(&mut self) -> Option<Rune> { self.rune.take() }
}

#[derive(Debug, Clone, serde::Serialize)]
struct SimplifiedRune {
    capabilities: BTreeMap<String, SimplifiedCapability>,
}

impl SimplifiedRune {
    fn from_rune(rune: &Rune) -> Self {
        let mut capabilities = BTreeMap::new();

        for (&id, node) in &rune.stages {
            let name = rune.names[id].to_string();
            let outputs = node
                .output_slots
                .iter()
                .map(|slot| rune.slots[slot].element_type)
                .map(|type_id| resolve_type(&rune, type_id))
                .collect();

            match &node.stage {
                hotg_rune_syntax::hir::Stage::Source(
                    hotg_rune_syntax::hir::Source { kind, parameters },
                ) => {
                    let kind = kind.clone();
                    let parameters = parameters.clone();
                    capabilities.insert(
                        name,
                        SimplifiedCapability {
                            capability_type: kind,
                            parameters,
                            outputs,
                        },
                    );
                },
                hotg_rune_syntax::hir::Stage::Sink(_) => {},
                hotg_rune_syntax::hir::Stage::Model(_) => {},
                hotg_rune_syntax::hir::Stage::ProcBlock(_) => {},
            }
        }

        SimplifiedRune { capabilities }
    }
}

fn resolve_type(rune: &Rune, type_id: HirId) -> Type {
    let (primitive, dims) = match &rune.types[&type_id] {
        hotg_rune_syntax::hir::Type::Primitive(p) => (p, vec![1]),
        hotg_rune_syntax::hir::Type::Buffer {
            underlying_type,
            dimensions,
        } => match &rune.types[underlying_type] {
            hotg_rune_syntax::hir::Type::Primitive(p) => {
                (p, dimensions.clone())
            },
            _ => unreachable!(),
        },
        hotg_rune_syntax::hir::Type::Unknown
        | hotg_rune_syntax::hir::Type::Any => {
            unreachable!("All types should have been resolved")
        },
    };

    Type {
        name: primitive.rust_name().to_string(),
        dimensions: dims,
    }
}

#[derive(Debug, Clone, serde::Serialize)]
struct SimplifiedCapability {
    #[serde(serialize_with = "serialize_source_kind")]
    capability_type: SourceKind,
    outputs: Vec<Type>,
    parameters: HashMap<String, Value>,
}

fn serialize_source_kind<S: Serializer>(
    kind: &SourceKind,
    ser: S,
) -> Result<S::Ok, S::Error> {
    kind.to_string().serialize(ser)
}

pub(crate) fn wasm_custom_sections(
    wasm: &[u8],
) -> impl Iterator<Item = CustomSection<'_>> + '_ {
    Parser::default()
        .parse_all(wasm)
        .filter_map(Result::ok)
        .filter_map(|payload| match payload {
            Payload::CustomSection { name, data, .. } => {
                Some(CustomSection { name, data })
            },
            _ => None,
        })
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct CustomSection<'a> {
    pub(crate) name: &'a str,
    pub(crate) data: &'a [u8],
}
