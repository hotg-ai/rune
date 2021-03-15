use anyhow::{Error, Context};
use super::Output;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Serial {}

impl Output for Serial {
    fn consume(&mut self, buffer: &[u8]) -> Result<(), Error> {
        let deserialized = serde_json::from_slice(buffer)
            .context("Unable to parse the data as JSON")?;

        log::info!("Serial: {:?}", deserialized);

        Ok(())
    }
}
