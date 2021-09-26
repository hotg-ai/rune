use hotg_rune_compiler::{
    codegen::{
        CapabilitySummary, ModelSummary, OutputSummary, ProcBlockSummary,
        RuneGraph, TensorId,
    },
};
use hotg_rune_core::Shape;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};
use anyhow::{Context, Error};
use crate::inspect::Metadata;

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Graph {
    /// Where to write the generated file (stdout by default).
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    /// A compiled Rune to graph.
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Graph {
    pub fn execute(self) -> Result<(), Error> {
        let bytes = std::fs::read(&self.input).with_context(|| {
            format!("Unable to read \"{}\"", self.input.display())
        })?;

        let Metadata { rune, .. } = Metadata::from_wasm_binary(&bytes)
            .context(
                "Unable to extract metadata from the WebAssembly module",
            )?;
        let rune =
            rune.context("Unable to find the Rune graph custom section")?;

        let mut writer = self.writer()?;
        render(&mut *writer, &rune).context("Render failed")?;
        writer.flush().context("Flush failed")?;

        Ok(())
    }

    fn writer(&self) -> Result<Box<dyn Write>, Error> {
        match &self.output {
            Some(path) => {
                let file = File::open(path).with_context(|| {
                    format!("Unable to open \"{}\" for writing", path.display())
                })?;
                Ok(Box::new(file))
            },
            None => {
                let stdout = std::io::stdout();
                Ok(Box::new(stdout))
            },
        }
    }
}

fn render(w: &mut dyn Write, rune: &RuneGraph) -> Result<(), Error> {
    writeln!(w, "digraph {{")?;
    writeln!(w, "  rankdir=TD;")?;
    writeln!(w, "  node [shape=plaintext];")?;

    declare_nodes(w, rune)?;
    declare_edges(w, rune)?;

    writeln!(w, "}}")?;
    Ok(())
}

fn declare_edges(w: &mut dyn Write, rune: &RuneGraph) -> Result<(), Error> {
    let nodes: Vec<_> = pipeline_nodes(rune).collect();
    for node in nodes.iter().copied() {
        declare_input_edges(w, node, &rune.tensors, &nodes)?;
    }

    Ok(())
}

fn declare_input_edges(
    w: &mut dyn Write,
    node: PipelineNode<'_>,
    tensors: &HashMap<TensorId, Shape<'_>>,
    nodes: &[PipelineNode<'_>],
) -> Result<(), Error> {
    for (i, tensor_id) in node.inputs.iter().enumerate() {
        let shape = &tensors[tensor_id];

        let (input_node, input_index) = nodes
            .iter()
            .find_map(|n| {
                n.outputs
                    .iter()
                    .position(|t| t == tensor_id)
                    .map(|i| (n, i))
            })
            .expect("The graph was malformed");

        writeln!(
            w,
            "  node_{}:output_{}:s -> node_{}:input_{}:n [label=\"{}\"];",
            input_node.name, input_index, node.name, i, shape
        )?;
    }

    Ok(())
}

fn declare_nodes(w: &mut dyn Write, rune: &RuneGraph) -> Result<(), Error> {
    for node in pipeline_nodes(rune) {
        let colour = node_colour(node.specifics);
        write!(
            w,
            "  node_{} [fillcolor={}, style=\"filled\", label=",
            node.name, colour
        )?;
        let qualifier = node.specifics.qualifier();
        format_node_label(
            w,
            node.name,
            &qualifier,
            &node.inputs,
            &node.outputs,
        )?;
        writeln!(w, "];")?;
    }

    Ok(())
}

fn node_colour(specifics: NodeType<'_>) -> &'static str {
    match specifics {
        NodeType::Capability(_) => "lightgreen",
        NodeType::Model(_) => "violet",
        NodeType::ProcBlock(_) => "tan1",
        NodeType::Output(_) => "indianred1",
    }
}

fn format_node_label(
    w: &mut dyn Write,
    name: &str,
    qualifier: &str,
    inputs: &[TensorId],
    outputs: &[TensorId],
) -> Result<(), Error> {
    writeln!(w, "<")?;
    writeln!(
        w,
        r#"    <table border="0" cellborder="0" cellspacing="5">"#
    )?;

    if !inputs.is_empty() {
        write!(
            w,
            r#"      <tr><td><table cellborder="1" cellspacing="0"><tr>"#
        )?;
        for i in 0..inputs.len() {
            write!(w, "<td port=\"input_{}\">{}</td>", i, i)?;
        }
        writeln!(w, "</tr></table></td></tr>")?;
    }

    writeln!(w, "      <tr><td>{}: {}</td></tr>", name, qualifier)?;

    if !outputs.is_empty() {
        write!(
            w,
            r#"      <tr><td><table cellborder="1" cellspacing="0"><tr>"#
        )?;
        for i in 0..outputs.len() {
            write!(w, "<td port=\"output_{}\">{}</td>", i, i)?;
        }
        writeln!(w, "</tr></table></td></tr>")?;
    }

    writeln!(w, "    </table>")?;
    write!(w, "  >")?;

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum NodeType<'a> {
    Capability(&'a CapabilitySummary),
    Model(&'a ModelSummary),
    ProcBlock(&'a ProcBlockSummary),
    Output(&'a OutputSummary),
}

#[derive(Debug, Copy, Clone)]
struct PipelineNode<'a> {
    name: &'a str,
    specifics: NodeType<'a>,
    inputs: &'a [TensorId],
    outputs: &'a [TensorId],
}

fn pipeline_nodes(
    rune: &RuneGraph,
) -> impl Iterator<Item = PipelineNode<'_>> + '_ {
    let RuneGraph {
        capabilities,
        models,
        proc_blocks,
        outputs,
        ..
    } = rune;

    const EMPTY: &[TensorId] = &[];

    let capabilities = capabilities.iter().map(|(name, cap)| PipelineNode {
        name: name.as_str(),
        specifics: NodeType::Capability(cap),
        inputs: EMPTY,
        outputs: cap.outputs.as_slice(),
    });
    let models = models.iter().map(|(name, model)| PipelineNode {
        name: name.as_str(),
        specifics: NodeType::Model(model),
        inputs: model.inputs.as_slice(),
        outputs: model.outputs.as_slice(),
    });
    let proc_blocks = proc_blocks.iter().map(|(name, pb)| PipelineNode {
        name: name.as_str(),
        specifics: NodeType::ProcBlock(pb),
        inputs: pb.inputs.as_slice(),
        outputs: pb.outputs.as_slice(),
    });
    let outputs = outputs.iter().map(|(name, out)| PipelineNode {
        name: name.as_str(),
        specifics: NodeType::Output(out),
        inputs: out.inputs.as_slice(),
        outputs: EMPTY,
    });

    capabilities.chain(models).chain(proc_blocks).chain(outputs)
}
impl NodeType<'_> {
    pub(crate) fn qualifier(self) -> String {
        match self {
            NodeType::Capability(cap) => cap.kind.to_string(),
            NodeType::Model(model) => model.file.to_string(),
            NodeType::ProcBlock(pb) => pb.path.to_string(),
            NodeType::Output(out) => out.kind.to_string(),
        }
    }
}
