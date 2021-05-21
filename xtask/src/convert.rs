use std::{path::PathBuf, fs::File, io::Write};
use anyhow::{Error, Context};
use rune_syntax::Diagnostics;
use codespan_reporting::{
    files::SimpleFile,
    term::{ColorArg, Config, termcolor::StandardStream},
};

#[derive(Debug, Clone, structopt::StructOpt)]
pub struct Convert {
    #[structopt(
        short,
        long,
        help = "Where to write the converted Runefile (stdout if not provided)"
    )]
    output: Option<PathBuf>,
    /// Configure coloring of output
    #[structopt(
        long = "color",
        default_value = "auto",
        possible_values = ColorArg::VARIANTS,
        case_insensitive = true,
    )]
    colour: ColorArg,
    #[structopt(default_value = "Runefile", help = "The Runefile to convert")]
    filename: PathBuf,
}

impl Convert {
    pub fn run(&self) -> Result<(), Error> {
        let src =
            std::fs::read_to_string(&self.filename).with_context(|| {
                format!("Unable to read \"{}\"", self.filename.display())
            })?;

        let runefile =
            rune_syntax::parse(&src).context("Unable to parse the Runefile")?;

        let mut diags = Diagnostics::new();
        let document =
            rune_syntax::yaml::document_from_runefile(&runefile, &mut diags);

        let mut writer = StandardStream::stdout(self.colour.0);
        let config = Config::default();
        let file = SimpleFile::new(self.filename.display().to_string(), &src);

        for diag in &diags {
            codespan_reporting::term::emit(&mut writer, &config, &file, diag)
                .context("Unable to print the diagnostic")?;
        }

        if diags.has_errors() {
            anyhow::bail!("Aborting due to conversion errors");
        }

        let mut writer = self.writer()?;
        document
            .write_as_yaml(&mut writer)
            .context("Unable to serialize as YAML")?;
        writer.flush().context("Unable to flush output")?;

        Ok(())
    }

    fn writer(&self) -> Result<Box<dyn Write>, Error> {
        match &self.output {
            Some(filename) => {
                if let Some(parent) = filename.parent() {
                    std::fs::create_dir_all(parent).with_context(|| {
                        format!(
                            "Unable to create the \"{}\" directory",
                            parent.display()
                        )
                    })?;
                }

                let f = File::create(filename).with_context(|| {
                    format!(
                        "Unable to open \"{}\" for writing",
                        filename.display()
                    )
                })?;

                Ok(Box::new(f) as Box<dyn Write>)
            },
            None => Ok(Box::new(std::io::stdout()) as Box<dyn Write>),
        }
    }
}
