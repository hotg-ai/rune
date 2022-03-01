use std::{
    fs::File,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Context, Error};
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

        log::info!("Generating a JSON schema based on the serde types");
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

        log::info!("Generating TypeScript types for our Runefile");
        let status = Command::new("yarn")
            .arg("generate-runefile-types")
            .current_dir(project_root.join("bindings").join("web"))
            .status()
            .context("unable to start \"yarn\", is it installed?")?;

        anyhow::ensure!(
            status.success(),
            "Unable to generate TypeScript types. Do you need to run \"yarn \
             install\" in the web bindings folder?"
        );

        Ok(())
    }
}
