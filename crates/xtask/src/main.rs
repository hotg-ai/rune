mod bulk_copy;
mod dist;
mod update_schema;

use std::path::PathBuf;

use anyhow::{Context, Error};
use env_logger::Env;
use structopt::StructOpt;

use crate::{bulk_copy::BulkCopy, dist::Dist, update_schema::UpdateSchema};

fn main() -> Result<(), Error> {
    let env = Env::new().default_filter_or("info,cbindgen=warn,globset=info");
    env_logger::builder().parse_env(env).init();

    let cmd = Command::from_args();

    log::debug!("Running {:?}", cmd);
    let project_root = project_root()?;

    match cmd {
        Command::Dist(dist) => dist.run()?,
        Command::UpdateSchema(u) => u.run(&project_root)?,
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "dist", about = "Generate a release bundle")]
    Dist(Dist),
    #[structopt(
        name = "update-schema",
        about = "Update the JSON schema for a Runefile"
    )]
    UpdateSchema(UpdateSchema),
}

fn project_root() -> Result<PathBuf, Error> {
    let cwd = std::env::current_dir()
        .context("Unable to determine the current directory")?;

    for ancestor in cwd.ancestors() {
        if ancestor.join(".git").exists() {
            return Ok(ancestor.to_path_buf());
        }
    }

    Err(Error::msg("Unable to find the project directory"))
}
