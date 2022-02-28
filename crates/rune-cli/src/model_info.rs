use std::{io::Write, path::PathBuf};

use anyhow::{Context, Error};
use hotg_runecoral::{
    mimetype, AccelerationBackend, InferenceContext, TensorDescriptor,
};
use strum::VariantNames;

use crate::Format;

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct ModelInfo {
    #[structopt(
        help = "The TensorFlow Lite model to inspect",
        parse(from_os_str)
    )]
    file: PathBuf,
    #[structopt(
        short,
        long,
        help = "The format to print output in",
        default_value = "text",
        possible_values = Format::VARIANTS,
        parse(try_from_str)
    )]
    format: Format,
}

impl ModelInfo {
    pub fn execute(self) -> Result<(), Error> {
        let raw = std::fs::read(&self.file).with_context(|| {
            format!("Unable to read \"{}\"", &self.file.display())
        })?;

        let ctx = InferenceContext::create_context(
            mimetype(),
            &raw,
            AccelerationBackend::NONE,
        )
        .context("Unable to an inference context")?;

        match self.format {
            Format::Text => print_info(&ctx),
            Format::Json => {
                let mut stdout = std::io::stdout();
                serde_json::to_writer_pretty(
                    stdout.lock(),
                    &ModelDescription {
                        inputs: ctx
                            .inputs()
                            .map(|x| TensorInfo::from(&x))
                            .collect(),
                        outputs: ctx
                            .outputs()
                            .map(|x| TensorInfo::from(&x))
                            .collect(),
                        ops: ctx.opcount() as usize,
                    },
                )
                .context("Unable to print to stdout")?;
                writeln!(stdout)?;
            },
        }

        Ok(())
    }
}

fn print_info(ctx: &InferenceContext) {
    println!("Ops: {}", ctx.opcount());

    println!("Inputs:");
    for input in ctx.inputs() {
        println!("\t{}", input);
    }

    println!("Outputs:");
    for output in ctx.outputs() {
        println!("\t{}", output);
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
struct ModelDescription {
    inputs: Vec<TensorInfo>,
    outputs: Vec<TensorInfo>,
    ops: usize,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
struct TensorInfo {
    name: String,
    element_kind: String,
    dims: Vec<usize>,
}

impl From<&TensorDescriptor<'_>> for TensorInfo {
    fn from(t: &TensorDescriptor<'_>) -> TensorInfo {
        TensorInfo {
            name: t.name.to_str().unwrap().to_string(),
            element_kind: t.element_type.to_string(),
            dims: t.shape.iter().map(|&x| x as usize).collect(),
        }
    }
}
