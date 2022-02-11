mod rune;

pub(crate) use self::rune::{Metadata, CustomSection, wasm_custom_sections};

use std::path::PathBuf;
use anyhow::Error;
use strum::VariantNames;
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
    #[structopt(help = "The File to inspect", parse(from_os_str))]
    filename: PathBuf,
}

impl Inspect {
    pub fn execute(self) -> Result<(), Error> {
        let Inspect { format, filename } = self;

        match filename.extension().and_then(|s| s.to_str()) {
            Some("rune" | "wasm") => rune::inspect(format, &filename),
            Some(other) => {
                anyhow::bail!("Unable to inspect a \"{}\" file", other)
            },
            None => anyhow::bail!("Unable to inspect this file"),
        }
    }
}
