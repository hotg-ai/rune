use std::{
    path::Path,
    process::{Output, Stdio},
};

use anyhow::{Context, Error};

use crate::TestContext;

pub(crate) fn rune_output(
    name: &str,
    directory: &Path,
    ctx: &TestContext,
) -> Result<Output, Error> {
    let mut cmd = ctx.rune_cmd();

    cmd.arg("build")
        .arg(directory.join("Runefile.yml"))
        .arg("--debug")
        .arg("--cache-dir")
        .arg(ctx.cache_dir.join("compile").join(name))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    log::debug!("Executing {:?}", cmd);

    cmd.output().context("Unable to run `rune`")
}
