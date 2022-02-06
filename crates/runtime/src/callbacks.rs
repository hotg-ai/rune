use std::collections::HashMap;
use anyhow::Error;
use log::Record;

pub trait Callbacks: Send + Sync + 'static {
    /// A callback fired after a Rune is loaded.
    fn loaded(&self, _rune: &RuneGraph<'_>) -> Result<(), Error> { Ok(()) }

    fn read_capability(
        &self,
        id: u32,
        meta: &NodeMetadata,
        buffer: &mut [u8],
    ) -> Result<(), Error>;

    fn write_output(
        &self,
        id: u32,
        meta: &NodeMetadata,
        data: &[u8],
    ) -> Result<(), Error>;

    /// Set up any necessary internal bookkeeping to load a model.
    fn load_model(
        &self,
        id: u32,
        mimetype: &str,
        model: &[u8],
    ) -> Result<(), Error>;

    /// Run inference on a model.
    fn model_infer(
        &self,
        id: u32,
        inputs: &[&[u8]],
        outputs: &mut [&mut [u8]],
    ) -> Result<(), Error>;

    /// Get the value of a global resource.
    fn get_resource(&self, name: &str) -> Option<&[u8]>;

    fn log(&self, _record: &Record<'_>) {}
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct NodeMetadata {
    pub kind: String,
    pub arguments: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct RuneGraph<'a> {
    pub capabilities: &'a HashMap<u32, NodeMetadata>,
    pub outputs: &'a HashMap<u32, NodeMetadata>,
}
