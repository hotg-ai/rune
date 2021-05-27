use std::path::Path;
use anyhow::{Error, Context};

pub(crate) fn read_to_string(path: impl AsRef<Path>) -> Result<String, Error> {
    let path = path.as_ref();

    std::fs::read_to_string(path)
        .with_context(|| format!("Unable to read \"{}\"", path.display()))
}
