use anyhow::Error;

use crate::{builtins::Arguments, Tensor};

pub fn raw(args: &Arguments, bytes: &[u8]) -> Result<Tensor, Error> {
    let length: usize = args.parse_or_default("length", bytes.len())?;

    if bytes.len() < length {
        anyhow::bail!(
            "Requested {} bytes but only {} were provided",
            length,
            bytes.len()
        );
    }

    Ok(Tensor::new(&bytes[..length], &[1, length]))
}
