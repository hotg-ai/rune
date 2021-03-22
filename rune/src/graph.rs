use std::{fs::File, io::Write, path::PathBuf};
use anyhow::{Context, Error};
use codespan_reporting::term::termcolor::ColorChoice;
use petgraph::{
    dot::{Config, Dot},
    graph::{EdgeReference},
};
use rune_syntax::hir::{Edge, Stage};

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Graph {
    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Where to write the generated DOT file (stdout by default)"
    )]
    output: Option<PathBuf>,
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

        let format_edge = |_, edge: EdgeReference<Edge>| {
            let type_id = edge.weight().ty;
            let ty = &rune.types[&type_id];
            let name = ty.rust_type_name(&rune.types).unwrap_or_default();

            format!("label = \"{}\"", name)
        };
        let format_node = |_, (node_ix, stage): (_, &Stage)| {
            let name = rune
                .nodes_to_hir_id
                .get(&node_ix)
                .and_then(|id| rune.names.get_name(*id))
                .unwrap_or("<anon>");

            let formatted = match stage {
                Stage::ProcBlock(pb) => {
                    format!("{}: ProcBlock({})", name, pb.path)
                },
                Stage::Model(m) => {
                    format!("{}: Model({})", name, m.model_file.display())
                },
                Stage::Source(s) => format!("{}: {:?}", name, s.kind),
                Stage::Sink(s) => format!("{}: {:?}", name, s.kind),
            };
            format!("label = \"{}\"", formatted)
        };

        let dot = Dot::with_attr_getters(
            &rune.graph,
            &[Config::NodeNoLabel, Config::EdgeNoLabel],
            &format_edge,
            &format_node,
        );

        let mut writer = self.writer()?;
        writeln!(writer, "{:?}", dot)?;
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
