use anyhow::Error;

use crate::{builtins::Arguments, Tensor};

pub fn raw(args: &Arguments, text: &str) -> Result<Tensor, Error> {
    let length: usize = args.parse_or_default("length", text.len())?;

    if text.len() < length {
        anyhow::bail!(
            "Requested {} bytes but only {} were provided",
            length,
            text.len()
        );
    }

    let bytes = text.as_bytes();
    Ok(Tensor::new(&bytes[..length], &[1, length]))
}
