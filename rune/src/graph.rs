use std::{fs::File, io::Write, path::PathBuf};
use anyhow::{Context, Error};
use rune_syntax::hir::{HirId, NameTable, Node, Rune, Slot};
use codespan_reporting::term::termcolor::ColorChoice;
use indexmap::IndexMap;

use crate::inspect::Metadata;

const WASM_MAGIC_BYTES: &[u8; 4] = b"\0asm";

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Graph {
    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Where to write the generated file (stdout by default)"
    )]
    output: Option<PathBuf>,
    #[structopt(parse(from_os_str), help = "The Rune or Runefile to graph")]
    input: PathBuf,
}

impl Graph {
    pub fn execute(self, color: ColorChoice) -> Result<(), Error> {
        let rune = self.load_rune(color).context("unable to load the input")?;

        let mut writer = self.writer()?;
        generate_graph(&mut *writer, &rune)?;
        writer.flush()?;

        Ok(())
    }

    fn load_rune(&self, color: ColorChoice) -> Result<Rune, Error> {
        let bytes = std::fs::read(&self.input).with_context(|| {
            format!("Unable to read \"{}\"", self.input.display())
        })?;

        if bytes.starts_with(WASM_MAGIC_BYTES) {
            // It's a compiled Rune
            Metadata::from_wasm_binary(&bytes)
                .take_rune()
                .context("Unable to load the Rune metadata from the input")
        } else {
            // Try to analyse it as a Runefile
            crate::build::analyze(&self.input, color)
        }
    }

    fn writer(&self) -> Result<Box<dyn Write>, Error> {
        match &self.output {
            Some(path) => {
                let f = File::create(path).with_context(|| {
                    format!("Unable to open \"{}\" for writing", path.display())
                })?;

                Ok(Box::new(f))
            },
            None => Ok(Box::new(std::io::stdout())),
        }
    }
}

fn generate_graph(w: &mut dyn Write, rune: &Rune) -> Result<(), Error> {
    writeln!(w, "digraph {{")?;
    writeln!(w, "  rankdir=LR;")?;
    writeln!(w, "  node [shape=record];")?;

    declare_nodes(w, &rune.stages, &rune.names)?;
    declare_edges(w, &rune.stages, &rune.slots)?;

    writeln!(w, "}}")?;

    Ok(())
}

fn declare_edges(
    w: &mut dyn Write,
    stages: &IndexMap<HirId, Node>,
    slots: &IndexMap<HirId, Slot>,
) -> Result<(), Error> {
    for (id, slot) in slots {
        let Slot {
            input_node,
            output_node,
            ..
        } = slot;
        write!(w, "  node_{}:output_{}", input_node, id)?;
        write!(w, " -> ")?;

        writeln!(w, "node_{}:input_{};", output_node, id)?;
    }

    Ok(())
}

fn declare_nodes(
    w: &mut dyn Write,
    stages: &IndexMap<HirId, Node>,
    names: &NameTable,
) -> Result<(), Error> {
    for (&id, node) in stages {
        let name = names.get_name(id).with_context(|| {
            format!("Unable to get the name for node {}", id)
        })?;

        write!(w, "  node_{} [label=", id)?;

        format_node_label(w, name, node)?;
        writeln!(w, "];")?;
    }

    Ok(())
}

fn format_node_label(
    w: &mut dyn Write,
    name: &str,
    node: &Node,
) -> Result<(), Error> {
    write!(w, "\"")?;

    if !node.input_slots.is_empty() {
        write!(w, "{{")?;
        for (i, slot) in node.input_slots.iter().enumerate() {
            if i > 0 {
                write!(w, "|")?;
            }
            write!(w, "<input_{}> {}", slot, i)?;
        }
        write!(w, "}}|")?;
    }

    write!(w, "{}", name)?;

    if !node.output_slots.is_empty() {
        write!(w, "|{{")?;
        for (i, slot) in node.output_slots.iter().enumerate() {
            if i > 0 {
                write!(w, "|")?;
            }
            write!(w, "<output_{}> {}", slot, i)?;
        }
        write!(w, "}}")?;
    }

    write!(w, "\"")?;

    Ok(())
}
