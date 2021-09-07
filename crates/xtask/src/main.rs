mod bulk_copy;
mod check_manifests;
mod dist;

use crate::{bulk_copy::BulkCopy, check_manifests::CheckManifests, dist::Dist};
use std::path::PathBuf;
use anyhow::{Context, Error};
use env_logger::Env;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let env = Env::new().default_filter_or("info,cbindgen=warn,globset=info");
    env_logger::builder().parse_env(env).init();

    let cmd = Command::from_args();

    log::debug!("Running {:?}", cmd);
    let project_root = project_root()?;

    match cmd {
        Command::Dist(dist) => dist.run()?,
        Command::CheckManifests(c) => c.run(&project_root)?,
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "dist", about = "Generate a release bundle")]
    Dist(Dist),
    #[structopt(
        name = "check-manifests",
        about = "Check all Cargo.toml files are"
    )]
    CheckManifests(CheckManifests),
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
