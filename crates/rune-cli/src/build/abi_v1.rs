use anyhow::Error;

use crate::{Build, Unstable};

pub(crate) fn execute(build: Build, unstable: Unstable) -> Result<(), Error> {
    if !unstable.unstable {
        anyhow::bail!("Building with the new ABI is still experimental. Please use the `--unstable` flag.");
    }

    todo!()
}
