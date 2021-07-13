use std::{
    fmt::{self, Display, Formatter},
    io::Write,
    path::{Path, PathBuf},
};
use strum::VariantNames;
use anyhow::{Error, Context};
use tflite::{
    FlatBufferModel, Interpreter, InterpreterBuilder,
    ops::builtin::BuiltinOpResolver,
};

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
        let interpreter =
            load_model(&self.file).context("Unable to load the model")?;

        let info = parse_info(&interpreter);

        match self.format {
            Format::Text => print_info(&info),
            Format::Json => {
                let mut stdout = std::io::stdout();
                serde_json::to_writer_pretty(stdout.lock(), &info)
                    .context("Unable to print to stdout")?;
                writeln!(stdout)?;
            },
        }

        Ok(())
    }
}

fn print_info(info: &ModelDescription) {
    println!("Ops: {}", info.ops);

    println!("Inputs:");
    for input in &info.inputs {
        println!("\t{}", input);
    }

    println!("Outputs:");
    for output in &info.outputs {
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

impl Display for TensorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}[", self.name, self.element_kind)?;

        for (i, dim) in self.dims.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", dim)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}

impl From<tflite::context::TensorInfo> for TensorInfo {
    fn from(t: tflite::context::TensorInfo) -> Self {
        let tflite::context::TensorInfo {
            name,
            element_kind,
            dims,
        } = t;

        TensorInfo {
            name,
            dims,
            element_kind: format!("{:?}", element_kind)
                .trim_start_matches("kTfLite")
                .to_string(),
        }
    }
}

fn parse_info(
    interpreter: &Interpreter<'static, BuiltinOpResolver>,
) -> ModelDescription {
    let inputs = interpreter
        .inputs()
        .iter()
        .map(|ix| interpreter.tensor_info(*ix).unwrap())
        .map(TensorInfo::from)
        .collect();
    let outputs = interpreter
        .outputs()
        .iter()
        .map(|ix| interpreter.tensor_info(*ix).unwrap())
        .map(TensorInfo::from)
        .collect();

    ModelDescription {
        inputs,
        outputs,
        ops: interpreter.nodes_size(),
    }
}

fn load_model(
    filename: &Path,
) -> Result<Interpreter<'static, BuiltinOpResolver>, Error> {
    let raw = std::fs::read(filename).with_context(|| {
        format!("Unable to read \"{}\"", filename.display())
    })?;

    let flat_buffer = FlatBufferModel::build_from_buffer(raw)
        .context("Unable to load the buffer as a TensorFlow Lite model")?;

    let resolver = BuiltinOpResolver::default();

    let interpreter = InterpreterBuilder::new(flat_buffer, resolver)
        .context("Unable to create a model interpreter builder")?
        .build()
        .context("Unable to initialize the model interpreter")?;

    Ok(interpreter)
}
