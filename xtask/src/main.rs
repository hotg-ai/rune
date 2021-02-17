use std::path::Path;

use anyhow::{Context, Error};
use devx_pre_commit::PreCommitContext;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let project_root = devx_pre_commit::locate_project_root()
        .context("Unable to find the project root")?;

    if is_pre_commit() {
        return run_pre_commit_hook(&project_root);
    }

    let cmd = Command::from_args();

    match cmd {
        Command::InstallPreCommit => {
            devx_pre_commit::install_self_as_hook(&project_root)
                .context("Unable to install the pre-commit hook")?;
        },
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
}
