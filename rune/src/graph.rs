use std::{fs::File, io::Write, path::PathBuf, str::FromStr};
use anyhow::{Context, Error};
use codespan_reporting::term::termcolor::ColorChoice;
use petgraph::{
    dot::{Config, Dot},
    graph::{EdgeReference},
};
use rune_syntax::hir::{Edge, Rune, Stage};

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Graph {
    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Where to write the generated file (stdout by default)"
    )]
    output: Option<PathBuf>,
    #[structopt(
        short,
        long,
        parse(try_from_str),
        help = "The format to print the graph in",
        possible_values = &["dot", "json"],
        default_value = "dot"
    )]
    format: Format,
    #[structopt(
        default_value = "Runefile",
        parse(from_os_str),
        help = "The Runefile to graph"
    )]
    runefile: PathBuf,
}

impl Graph {
    pub fn execute(self, color: ColorChoice) -> Result<(), Error> {
        let rune = crate::build::analyze(&self.runefile, color)?;

        let mut writer = self.writer()?;
        match self.format {
            Format::Dot => dot_graph(&mut writer, &rune)?,
            Format::Json => json_graph(&mut *writer, &rune)?,
        }
        writer.flush()?;

        Ok(())
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

fn json_graph(w: &mut dyn Write, rune: &Rune) -> Result<(), Error> {
    serde_json::to_writer_pretty(&mut *w, rune)?;
    writeln!(w)?;
    Ok(())
}

fn dot_graph(w: &mut dyn Write, rune: &Rune) -> Result<(), Error> {
    let format_edge = |_, edge: EdgeReference<Edge>| {
        let type_id = edge.weight().type_id;
        let ty = &rune.types[&type_id];
        let name = ty.rust_type_name(&rune.types).unwrap_or_default();

        format!("label = \"{}\"", name)
    };
    let format_node = |_, (node_ix, stage): (_, &Stage)| {
        let name = rune
            .node_index_to_hir_id
            .get(&node_ix)
            .and_then(|id| rune.names.get_name(*id))
            .unwrap_or("<anon>");

        match stage {
            Stage::ProcBlock(pb) => {
                format!(
                    "label=\"{}: {}\", fillcolor=lightgoldenrod1,style=filled",
                    name, pb.path
                )
            },
            Stage::Model(m) => {
                format!(
                    "label=\"{}: {}\", fillcolor=\"#FF6F00\",style=filled",
                    name,
                    m.model_file.display()
                )
            },
            Stage::Source(s) => format!(
                "label=\"{}: {:?}\", fillcolor=seagreen2, style=filled",
                name, s.kind
            ),
            Stage::Sink(s) => format!(
                "label=\"{}: {:?}\", fillcolor=crimson, fontcolor=white, style=filled",
                name, s.kind
            ),
        }
    };

    let dot = Dot::with_attr_getters(
        &rune.graph,
        &[Config::NodeNoLabel, Config::EdgeNoLabel],
        &format_edge,
        &format_node,
    );

    writeln!(w, "{:?}", dot)?;

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Format {
    Dot,
    Json,
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dot" => Ok(Format::Dot),
            "json" => Ok(Format::Json),
            _ => Err(Error::msg("Expected \"dot\" or \"json\"")),
        }
    }
}
