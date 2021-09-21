//! The various types that make up Rune's *High-level Internal Representation*.

use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    hash::Hash,
    ops::Deref,
    path::PathBuf,
    sync::Arc,
};
use hotg_rune_core::Shape;
use legion::Entity;
use crate::parse::{Path, ResourceType, Value};

/// An output.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Sink {
    pub kind: SinkKind,
}

/// The kind of output.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum SinkKind {
    Serial,
    Other(String),
}

impl Display for SinkKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SinkKind::Serial => write!(f, "serial"),
            SinkKind::Other(s) => write!(f, "{}", s),
        }
    }
}

impl<'a> From<&'a str> for SinkKind {
    fn from(s: &'a str) -> SinkKind {
        match s {
            "serial" | "SERIAL" => SinkKind::Serial,
            _ => SinkKind::Other(s.to_string()),
        }
    }
}

/// A ML model.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Model {
    pub model_file: ModelFile,
}

/// Where to load a model from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelFile {
    /// Load the model from a file on disk.
    FromDisk(PathBuf),
    /// Load the model from a resource embedded/injected into the Rune.
    Resource(Entity),
}

/// Something which can generate data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Source {
    pub kind: SourceKind,
    pub parameters: HashMap<String, Value>,
}

/// Where should a [`Source`] pull its data from?
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum SourceKind {
    Random,
    Accelerometer,
    Sound,
    Image,
    Raw,
    Other(String),
}

impl<'a> From<&'a str> for SourceKind {
    fn from(s: &'a str) -> SourceKind {
        match s {
            "rand" | "RAND" => SourceKind::Random,
            "accel" | "ACCEL" => SourceKind::Accelerometer,
            "sound" | "SOUND" => SourceKind::Sound,
            "image" | "IMAGE" => SourceKind::Image,
            "raw" | "RAW" => SourceKind::Raw,
            _ => SourceKind::Other(s.to_string()),
        }
    }
}

impl Display for SourceKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SourceKind::Other(custom) => Display::fmt(custom, f),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProcBlock {
    pub path: Path,
    pub parameters: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    /// Where to read the [`Resource`]'s default value from.
    pub default_value: Option<ResourceSource>,
    pub ty: ResourceType,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ResourceSource {
    /// The value is specified in-line as a string.
    Inline(String),
    /// The value should be read from disk.
    FromDisk(PathBuf),
}

/// An identifier used to refer to an item in a Runefile.
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
pub struct Name(String);

impl<S: Into<String>> From<S> for Name {
    fn from(s: S) -> Self { Name(s.into()) }
}

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &String { &self.0 }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
}

impl Borrow<String> for Name {
    fn borrow(&self) -> &String { &self.0 }
}
impl Borrow<str> for Name {
    fn borrow(&self) -> &str { &self.0 }
}

impl<S> AsRef<S> for Name
where
    String: AsRef<S>,
{
    fn as_ref(&self) -> &S { self.0.as_ref() }
}

/// A lookup table mapping [`Name`] components back to their [`Entity`].
#[derive(
    Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize,
)]
pub struct NameTable(HashMap<Name, Entity>);

impl NameTable {
    pub(crate) fn clear(&mut self) { self.0.clear(); }

    pub(crate) fn insert(&mut self, name: Name, ent: Entity) {
        self.0.insert(name, ent);
    }
}

impl Deref for NameTable {
    type Target = HashMap<Name, Entity>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<HashMap<Name, Entity>> for NameTable {
    fn from(m: HashMap<Name, Entity>) -> Self { NameTable(m) }
}

/// A tag component indicating this [`Entity`] is part of the Rune's pipeline.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct PipelineNode;

/// The [`Shape`] a tensor may take.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tensor(pub Shape<'static>);

impl From<Shape<'static>> for Tensor {
    fn from(s: Shape<'static>) -> Self { Tensor(s) }
}

/// The list of [`Tensor`]s that may be the output from a [`PipelineNode`].
#[derive(
    Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Outputs {
    pub tensors: Vec<Entity>,
}

/// The list of [`Tensor`]s that may be the inputs to a [`PipelineNode`].
#[derive(
    Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Inputs {
    pub tensors: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourceData(pub Arc<[u8]>);

impl<T: Into<Arc<[u8]>>> From<T> for ResourceData {
    fn from(data: T) -> Self { ResourceData(data.into()) }
}

impl AsRef<[u8]> for ResourceData {
    fn as_ref(&self) -> &[u8] { &*self }
}

impl Deref for ResourceData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModelData(pub Arc<[u8]>);

impl<A: Into<Arc<[u8]>>> From<A> for ModelData {
    fn from(data: A) -> Self { ModelData(data.into()) }
}

impl Deref for ModelData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target { &self.0 }
}
