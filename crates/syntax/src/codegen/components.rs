use std::{path::PathBuf, sync::Arc};
use serde::Serialize;

pub const GRAPH_CUSTOM_SECTION: &str = ".rune_graph";
pub const VERSION_CUSTOM_SECTION: &str = ".rune_version";
pub const RESOURCE_CUSTOM_SECTION: &str = ".rune_resource";

/// A file that will be written to the Rune's build directory.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct File {
    pub path: PathBuf,
    pub data: Arc<[u8]>,
}

impl File {
    pub fn new(path: impl Into<PathBuf>, data: impl Into<Arc<[u8]>>) -> Self {
        File {
            path: path.into(),
            data: data.into(),
        }
    }
}

/// A WebAssembly custom section to be embedded in the Rune.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CustomSection {
    pub section_name: String,
    pub value: Arc<[u8]>,
}

impl CustomSection {
    pub fn from_json(
        name: impl Into<String>,
        value: &impl Serialize,
    ) -> Result<Self, serde_json::Error> {
        let value = serde_json::to_vec(value)?;
        let name = name.into();

        Ok(CustomSection {
            section_name: name,
            value: value.into(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RuneVersion {
    pub version: String,
}

impl RuneVersion {
    pub(crate) fn as_custom_section(&self) -> Result<CustomSection, serde_json::Error> {
        CustomSection::from_json(VERSION_CUSTOM_SECTION, self)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RuneGraph {

}

impl RuneGraph {
    pub(crate) fn as_custom_section(&self) -> Result<CustomSection, serde_json::Error> {
        CustomSection::from_json(GRAPH_CUSTOM_SECTION, self)
    }
}
