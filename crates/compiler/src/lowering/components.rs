//! The various types that make up Rune's *High-level Internal Representation*.

use std::{
    borrow::{Borrow, Cow},
    fmt::{self, Display, Formatter},
    hash::Hash,
    ops::Deref,
    path::PathBuf,
    sync::Arc,
};

use hotg_rune_core::Shape;
use indexmap::IndexMap;
use legion::Entity;

use crate::parse::{Path, ResourceType};

/// An output.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Sink {
    pub kind: SinkKind,
    pub args: IndexMap<String, ResourceOrString>,
}

/// The kind of output.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum SinkKind {
    Serial,
    Tensor,
    Other(String),
}

impl Display for SinkKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SinkKind::Serial => write!(f, "serial"),
            SinkKind::Tensor => write!(f, "tensor"),
            SinkKind::Other(s) => write!(f, "{}", s),
        }
    }
}

impl<'a> From<&'a str> for SinkKind {
    fn from(s: &'a str) -> SinkKind {
        match s {
            "serial" | "SERIAL" => SinkKind::Serial,
            "tensor" | "TENSOR" => SinkKind::Tensor,
            _ => SinkKind::Other(s.to_string()),
        }
    }
}

/// A ML model.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Model {
    pub model_file: ModelFile,
    pub args: IndexMap<String, ResourceOrString>,
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
    pub parameters: IndexMap<String, ResourceOrString>,
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
    FloatImage,
    Other(String),
}

impl SourceKind {
    pub const fn as_capability_index(&self) -> Option<u32> {
        match self {
            SourceKind::Random => Some(hotg_rune_core::capabilities::RAND),
            SourceKind::Accelerometer => {
                Some(hotg_rune_core::capabilities::ACCEL)
            },
            SourceKind::Sound => Some(hotg_rune_core::capabilities::SOUND),
            SourceKind::Image => Some(hotg_rune_core::capabilities::IMAGE),
            SourceKind::Raw => Some(hotg_rune_core::capabilities::RAW),
            SourceKind::FloatImage => {
                Some(hotg_rune_core::capabilities::FLOAT_IMAGE)
            },
            _ => None,
        }
    }

    pub fn as_capability_name(&self) -> Option<&'static str> {
        self.as_capability_index()
            .and_then(hotg_rune_core::capabilities::name)
    }
}

impl<'a> From<&'a str> for SourceKind {
    fn from(s: &'a str) -> SourceKind {
        match s {
            "rand" | "RAND" => SourceKind::Random,
            "accel" | "ACCEL" => SourceKind::Accelerometer,
            "sound" | "SOUND" => SourceKind::Sound,
            "image" | "IMAGE" => SourceKind::Image,
            "raw" | "RAW" => SourceKind::Raw,
            "float-image" | "FLOAT_IMAGE" => SourceKind::FloatImage,
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProcBlock {
    pub path: Path,
    pub parameters: IndexMap<String, ResourceOrString>,
}

impl ProcBlock {
    /// The name of the Rust crate that implements this [`ProcBlock`].
    pub(crate) fn name(&self) -> &str {
        let full_name = self.path.sub_path.as_ref().unwrap_or(&self.path.base);
        let start_of_name = full_name.rfind('/').map(|ix| ix + 1).unwrap_or(0);

        &full_name[start_of_name..]
    }
}

// TODO: remove this
impl Hash for ProcBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.parameters.iter().for_each(|pair| pair.hash(state));
    }
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
pub struct NameTable(IndexMap<Name, Entity>);

impl NameTable {
    pub(crate) fn clear(&mut self) { self.0.clear(); }

    pub(crate) fn insert(&mut self, name: Name, ent: Entity) {
        self.0.insert(name, ent);
    }
}

impl Deref for NameTable {
    type Target = IndexMap<Name, Entity>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<IndexMap<Name, Entity>> for NameTable {
    fn from(m: IndexMap<Name, Entity>) -> Self { NameTable(m) }
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
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Inputs {
    pub tensors: Vec<Entity>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
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

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct ModelData(pub Arc<[u8]>);

impl<A: Into<Arc<[u8]>>> From<A> for ModelData {
    fn from(data: A) -> Self { ModelData(data.into()) }
}

impl Deref for ModelData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Mimetype(Cow<'static, str>);

impl Mimetype {
    pub const ONNX: Mimetype =
        Mimetype(Cow::Borrowed(hotg_rune_core::ONNX_MIMETYPE));
    pub const TENSORFLOW: Mimetype =
        Mimetype(Cow::Borrowed(hotg_rune_core::TF_MIMETYPE));
    pub const TENSORFLOW_JS: Mimetype =
        Mimetype(Cow::Borrowed(hotg_rune_core::TFJS_MIMETYPE));
    pub const TENSORFLOW_LITE: Mimetype =
        Mimetype(Cow::Borrowed(hotg_rune_core::TFLITE_MIMETYPE));
}

impl Deref for Mimetype {
    type Target = str;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Default for Mimetype {
    fn default() -> Self { Mimetype::TENSORFLOW_LITE }
}

impl From<&'static str> for Mimetype {
    fn from(s: &'static str) -> Self { Mimetype(Cow::Borrowed(s)) }
}

impl From<String> for Mimetype {
    fn from(s: String) -> Self { Mimetype(Cow::Owned(s)) }
}

impl AsRef<str> for Mimetype {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ResourceOrString {
    String(String),
    Resource(Entity),
}

impl<'a> From<&'a str> for ResourceOrString {
    fn from(s: &'a str) -> Self { ResourceOrString::String(s.into()) }
}

impl From<String> for ResourceOrString {
    fn from(s: String) -> Self { ResourceOrString::String(s) }
}
