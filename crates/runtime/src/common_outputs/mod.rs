use anyhow::{Error, Context};
use crate::Output;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Serial {}

impl Output for Serial {
    fn consume(&mut self, buffer: &[u8]) -> Result<(), Error> {
        let json = std::str::from_utf8(buffer)
            .context("Unable to parse the input as UTF-8")?;

        log::info!("Serial: {}", json);

        Ok(())
    }
}
