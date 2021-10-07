use std::{
    fs::File,
    process::{Command, Stdio},
};
use anyhow::{Error, Context};
use std::path::{PathBuf, Path};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct UpdateSchema {
    /// Where to write the schema to.
    #[structopt(short, long)]
    output: Option<PathBuf>,
}

impl UpdateSchema {
    pub fn run(self, project_root: &Path) -> Result<(), Error> {
        let dest = self.output.unwrap_or_else(|| {
            project_root
                .join("crates")
                .join("compiler")
                .join("runefile-schema.json")
        });
        let f = File::create(&dest).with_context(|| {
            format!("Unable to open \"{}\" for writing", dest.display())
        })?;

        let output = Command::new("cargo")
            .arg("run")
            .arg("--example=json-schema")
            .stdout(f)
            .stderr(Stdio::inherit())
            .output()
            .context("Unable to start cargo")?;
        anyhow::ensure!(
            output.status.success(),
            "Unable to generate the JSON schema"
        );

        Ok(())
    }
}
