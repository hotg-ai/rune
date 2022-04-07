use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    ops::Deref,
    path::PathBuf,
    sync::Arc,
};

use hotg_rune_core::Shape;
use serde::Serialize;

use crate::{
    lowering::{Name, Resource, SinkKind, SourceKind},
    parse::{Path, ResourceOrString},
};

pub const GRAPH_CUSTOM_SECTION: &str = ".rune_graph";
pub const VERSION_CUSTOM_SECTION: &str = ".rune_version";
pub const RESOURCE_CUSTOM_SECTION: &str = ".rune_resource";

/// A file that will be written to the Rune's build directory.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
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
    pub fn new(name: impl Into<String>, value: impl Into<Arc<[u8]>>) -> Self {
        let section_name = name.into();
        let value = value.into();

        debug_assert!(
            section_name.starts_with('.'),
            "Link section names should start with a \".\", found \"{}\"",
            section_name
        );

        CustomSection {
            section_name,
            value,
        }
    }

    pub fn from_json(
        name: impl Into<String>,
        value: &impl Serialize,
    ) -> Result<Self, serde_json::Error> {
        let value = serde_json::to_vec(value)?;
        let name = name.into();
        Ok(CustomSection::new(name, value))
    }

    pub(crate) fn identifier(&self) -> &str {
        self.section_name.trim_start_matches('.')
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct RuneVersion {
    /// The version of the tool generating a Rune, typically what you'd see
    /// when running `rune --version`.
    pub version: String,
}

impl RuneVersion {
    pub fn new(version: impl Into<String>) -> Self {
        RuneVersion {
            version: version.into(),
        }
    }

    pub(crate) fn as_custom_section(
        &self,
    ) -> Result<CustomSection, serde_json::Error> {
        CustomSection::from_json(VERSION_CUSTOM_SECTION, self)
    }
}

impl Display for RuneVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("v")?;
        f.write_str(&self.version)?;
        Ok(())
    }
}

/// A summary of the Rune pipeline that will be embedded in the Rune.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RuneGraph {
    pub rune: RuneSummary,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub capabilities: HashMap<Name, CapabilitySummary>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub models: HashMap<Name, ModelSummary>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub proc_blocks: HashMap<Name, ProcBlockSummary>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub outputs: HashMap<Name, OutputSummary>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub resources: HashMap<Name, Resource>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub tensors: HashMap<TensorId, Shape<'static>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RuneSummary {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CapabilitySummary {
    pub kind: SourceKind,
    pub args: HashMap<String, ResourceOrString>,
    pub outputs: Vec<TensorId>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModelSummary {
    pub file: ResourceOrString,
    pub args: HashMap<String, ResourceOrString>,
    pub inputs: Vec<TensorId>,
    pub outputs: Vec<TensorId>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProcBlockSummary {
    pub path: Path,
    pub args: HashMap<String, ResourceOrString>,
    pub inputs: Vec<TensorId>,
    pub outputs: Vec<TensorId>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OutputSummary {
    pub kind: SinkKind,
    pub args: HashMap<String, ResourceOrString>,
    pub inputs: Vec<TensorId>,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct TensorId(pub String);

impl From<String> for TensorId {
    fn from(s: String) -> Self { TensorId(s) }
}

impl Deref for TensorId {
    type Target = String;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl RuneGraph {
    pub(crate) fn as_custom_section(
        &self,
    ) -> Result<CustomSection, serde_json::Error> {
        CustomSection::from_json(GRAPH_CUSTOM_SECTION, self)
    }
}
