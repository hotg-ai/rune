use std::path::Path;

use anyhow::{Context, Error};
use log;
use rune_runtime::{DefaultEnvironment, Runtime};

pub fn run(
    container: impl AsRef<Path>,
    number_of_runs: usize,
) -> Result<(), Error> {
    let rune = container.as_ref();
    log::info!("Running rune: {}", rune.display());

    let rune = std::fs::read(rune)
        .with_context(|| format!("Unable to read \"{}\"", rune.display()))?;

    let env = DefaultEnvironment::default();
    let mut runtime = Runtime::load(&rune, env)
        .context("Unable to initialize the virtual machine")?;

    for _ in 0..number_of_runs {
        runtime.call().context("Call failed")?;
    }

    Ok(())
}
