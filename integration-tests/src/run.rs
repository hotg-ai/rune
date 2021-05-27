use std::{path::Path, process::Output};
use anyhow::{Context, Error};

use crate::TestContext;

pub(crate) fn rune_output(
    name: &str,
    directory: &Path,
    ctx: &TestContext,
) -> Result<Output, Error> {
    log::debug!("Compiling");
    let output = crate::compile::rune_output(name, directory, ctx)?;
    anyhow::ensure!(output.status.success(), "Unable to compile the Rune");

    ctx.rune_cmd()
        .arg("run")
        .arg(format!("{}.rune", name))
        .current_dir(directory)
        .output()
        .context("Unable to run `rune`")
}
