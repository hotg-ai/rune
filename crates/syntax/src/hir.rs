//! The *High-level Internal Representation*.

use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    hash::Hash,
    ops::Deref,
    path::PathBuf,
};
use hotg_rune_core::Shape;
use legion::Entity;
use crate::yaml::{Path, ResourceType, Value};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Sink {
    pub kind: SinkKind,
}

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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Model {
    pub model_file: ModelFile,
}

/// Where to load a model from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelFile {
    FromDisk(PathBuf),
    Resource(Entity),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Source {
    pub kind: SourceKind,
    pub parameters: HashMap<String, Value>,
}

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
    pub source: Option<ResourceSource>,
    pub ty: ResourceType,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ResourceSource {
    Inline(String),
    FromDisk(PathBuf),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Image(pub Path);

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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tensor(Shape<'static>);

impl From<Shape<'static>> for Tensor {
    fn from(s: Shape<'static>) -> Self { Tensor(s) }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Outputs {
    pub tensors: Vec<Entity>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Inputs {
    pub tensors: Vec<Entity>,
}
