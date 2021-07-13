use anyhow::Error;
use crate::Model;

pub(crate) fn initialize_model(_raw: &[u8]) -> Result<Box<dyn Model>, Error> {
    anyhow::bail!("Models are not supported")
}
