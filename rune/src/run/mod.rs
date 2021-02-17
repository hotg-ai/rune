use log;
pub mod vm;
use anyhow::{Context, Error};
use vm::*;

pub fn run(container: &str, number_of_runs: i32) -> Result<(), Error> {
    log::info!("Running rune: {}", container);

    let vm = VM::init(container)
        .context("Unable to initialize the virtual machine")?;

    // Create a Provider
    // Set up capabilities and use inputs from CLI params
    for _ in 0..number_of_runs {
        vm.call(vec![]).context("Call failed")?;
    }

    Ok(())
}
