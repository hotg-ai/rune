use std::{path::PathBuf, fs::File, io::Write};
use anyhow::{Error, Context};

#[derive(Debug, Clone, structopt::StructOpt)]
pub struct Convert {
    #[structopt(
        short,
        long,
        help = "Where to write the converted Runefile (stdout if not provided)"
    )]
    output: Option<PathBuf>,
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

        let document = rune_syntax::yaml::document_from_runefile(runefile);

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
