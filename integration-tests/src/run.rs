use std::{path::Path, process::Output};
use anyhow::{Context, Error};
use crate::{CommandOutput, FullName, TestContext};

pub(crate) fn rune_output(
    full_name: &FullName,
    directory: &Path,
    ctx: &TestContext,
) -> Result<Output, Error> {
    log::debug!("Compiling");
    let output = crate::compile::rune_output(full_name, directory, ctx)?;

    if !output.status.success() {
        return Err(Error::msg("Unable to compile the Rune")
            .context(CommandOutput::new(output)));
    }

    anyhow::ensure!(output.status.success(), "Unable to compile the Rune");

    let mut cmd = ctx.rune_cmd();

    cmd.arg("run").arg(format!("{}.rune", full_name.name));

    for entry in directory
        .read_dir()
        .context("Unable to read the directory")?
    {
        let entry = entry?;
        let filename = entry.path();
        let extension = match filename.extension().and_then(|ext| ext.to_str())
        {
            Some(ext) => ext,
            None => continue,
        };

        let argument = match extension {
            "png" => "--image",
            "wav" => "--sound",
            "csv" => "--accelerometer",
            "rand" => "--random",
            "bin" => "--raw",
            _ => continue,
        };

        cmd.arg(argument).arg(filename);
    }

    log::debug!("Executing {:?}", cmd);

    cmd.current_dir(directory)
        .output()
        .context("Unable to run `rune`")
}
