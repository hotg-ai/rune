mod bulk_copy;
mod check_manifests;
mod convert;
mod dist;

use crate::{
    bulk_copy::BulkCopy, check_manifests::CheckManifests, convert::Convert,
    dist::Dist,
};
use std::path::{Path, PathBuf};
use anyhow::{Context, Error};
use devx_pre_commit::PreCommitContext;
use env_logger::Env;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let project_root = devx_pre_commit::locate_project_root()
        .context("Unable to find the project root")?;

    if is_pre_commit() {
        return run_pre_commit_hook(&project_root);
    }

    let env = Env::new().default_filter_or("info,cbindgen=warn,globset=info");
    env_logger::builder().parse_env(env).init();

    let cmd = Command::from_args();

    log::debug!("Running {:?}", cmd);

    match cmd {
        Command::InstallPreCommit => {
            log::info!("Installing this binary as the pre-commit hook");
            devx_pre_commit::install_self_as_hook(&project_root)
                .context("Unable to install the pre-commit hook")?;
        },
        Command::Dist(dist) => dist.run()?,
        Command::Convert(c) => c.run()?,
        Command::CheckManifests(c) => c.run(&project_root)?,
    }

    Ok(())
}

fn is_pre_commit() -> bool {
    match std::env::args().next() {
        Some(binary_name) => binary_name.contains("pre-commit"),
        None => false,
    }
}

fn run_pre_commit_hook(project_root: &Path) -> Result<(), Error> {
    let ctx = PreCommitContext::from_git_diff(project_root)
        .context("Unable to load the pre-commit context")?;
    ctx.rustfmt().context("rustfmt failed")?;
    ctx.stage_new_changes().context("Unable to stage changes")?;

    Ok(())
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(
        name = "install-pre-commit-hook",
        about = "Install the common pre-commit hook"
    )]
    InstallPreCommit,
    #[structopt(name = "dist", about = "Generate a release bundle")]
    Dist(Dist),
    #[structopt(
        name = "convert",
        about = "Convert an old style Runefile to its YAML equivalent"
    )]
    Convert(Convert),
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
