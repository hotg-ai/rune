use anyhow::{Context, Error};
use log;
use rune_runtime::{DefaultEnvironment, Runtime};

pub fn run(container: &str, number_of_runs: i32) -> Result<(), Error> {
    log::info!("Running rune: {}", container);

    let rune = std::fs::read(container)
        .with_context(|| format!("Unable to read \"{}\"", container))?;

    let env = DefaultEnvironment::default();
    let mut runtime = Runtime::load(&rune, env)
        .context("Unable to initialize the virtual machine")?;

    for _ in 0..number_of_runs {
        runtime.call().context("Call failed")?;
    }

    Ok(())
}
