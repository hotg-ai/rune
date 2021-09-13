//! The *High-level Internal Representation*.

use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    convert::{TryFrom},
    fmt::{self, Display, Formatter},
    hash::Hash,
    io::{Error, ErrorKind, Write},
    ops::{Deref, Index},
    path::PathBuf,
};
use codespan::Span;
use indexmap::IndexMap;
use legion::Entity;
use crate::yaml::{Path, ResourceName, ResourceType, Value};

#[derive(
    Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct Rune {
    pub base_image: Option<Image>,
    pub stages: IndexMap<HirId, Node>,
    pub slots: IndexMap<HirId, Slot>,
    pub types: IndexMap<HirId, Type>,
    pub spans: IndexMap<HirId, Span>,
    pub resources: IndexMap<HirId, Resource>,
    pub names: NameTable,
}

impl Rune {
    pub fn stages(&self) -> impl Iterator<Item = (HirId, &Stage)> + '_ {
        self.stages.iter().map(|(id, node)| (*id, &node.stage))
    }

    pub fn proc_blocks(
        &self,
    ) -> impl Iterator<Item = (HirId, &ProcBlock)> + '_ {
        self.stages().filter_map(|(h, stage)| match stage {
            Stage::ProcBlock(pb) => Some((h, pb)),
            _ => None,
        })
    }

    pub fn models(&self) -> impl Iterator<Item = (HirId, &Model)> + '_ {
        self.stages().filter_map(|(h, stage)| match stage {
            Stage::Model(m) => Some((h, m)),
            _ => None,
        })
    }

    pub fn sinks(&self) -> impl Iterator<Item = (HirId, &Sink)> + '_ {
        self.stages().filter_map(|(h, stage)| match stage {
            Stage::Sink(s) => Some((h, s)),
            _ => None,
        })
    }

    pub fn sources(&self) -> impl Iterator<Item = (HirId, &Source)> + '_ {
        self.stages().filter_map(|(h, stage)| match stage {
            Stage::Source(s) => Some((h, s)),
            _ => None,
        })
    }

    /// Get a topological sorting of the pipeline graph.
    pub fn sorted_pipeline(&self) -> impl Iterator<Item = (HirId, &Node)> + '_ {
        // https://www.geeksforgeeks.org/topological-sorting/

        let mut visited = HashSet::with_capacity(self.stages.len());
        let mut stack = Vec::with_capacity(self.stages.len());

        for key in self.stages.keys() {
            if !visited.contains(key) {
                topo_sort(*key, self, &mut visited, &mut stack);
            }
        }

        stack.into_iter().map(move |id| (id, &self.stages[&id]))
    }
}

impl Rune {
    pub fn register_name(
        &mut self,
        name: &str,
        id: HirId,
    ) -> Result<(), HirId> {
        self.names.register(name, id)
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<HirId> {
        self.names.get_id(name)
    }

    pub fn get_name_by_id(&self, id: HirId) -> Option<&str> {
        self.names.get_name(id)
    }

    pub fn register_stage(&mut self, id: HirId, stage: Node) {
        self.stages.insert(id, stage);
    }

    pub fn get_stage_mut(&mut self, id: &HirId) -> Option<&mut Node> {
        self.stages.get_mut(id)
    }

    pub fn get_stage(&self, id: &HirId) -> Option<&Node> { self.stages.get(id) }

    pub fn get_resource(&self, id: &HirId) -> Option<&Resource> {
        self.resources.get(id)
    }
}

fn topo_sort(
    id: HirId,
    rune: &Rune,
    visited: &mut HashSet<HirId>,
    stack: &mut Vec<HirId>,
) {
    visited.insert(id);

    let node = &rune.stages[&id];

    for incoming in &node.input_slots {
        let slot = &rune.slots[incoming];
        let input = slot.input_node;

        if !visited.contains(&input) {
            topo_sort(input, rune, visited, stack);
        }
    }

    stack.push(id);
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct HirId(u64);

impl HirId {
    pub const ERROR: HirId = HirId(0);
    pub const UNKNOWN: HirId = HirId(1);

    /// The first non-builtin [`HirId`] that can be allocated to a HIR object.
    pub(crate) const fn first_user_defined() -> HirId { HirId(2) }

    pub fn is_error(self) -> bool { self == HirId::ERROR }

    pub fn is_unknown(self) -> bool { self == HirId::UNKNOWN }

    pub(crate) fn next(self) -> Self { HirId(self.0 + 1) }
}

impl Display for HirId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_error() {
            write!(f, "(error)")
        } else if self.is_unknown() {
            write!(f, "(unknown)")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl Default for HirId {
    fn default() -> Self { HirId::first_user_defined() }
}

/// A table mapping names to [`HirId`]s.
#[derive(
    Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct NameTable {
    name_to_id: HashMap<String, HirId>,
    id_to_name: HashMap<HirId, String>,
}

impl NameTable {
    pub fn register(&mut self, name: &str, id: HirId) -> Result<(), HirId> {
        if let Some(existing_item_id) = self.get_id(name) {
            return Err(existing_item_id);
        }

        self.name_to_id.insert(name.to_string(), id);
        self.id_to_name.insert(id, name.to_string());

        Ok(())
    }

    pub fn get_name(&self, id: HirId) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }

    pub fn get_id(&self, name: &str) -> Option<HirId> {
        self.name_to_id.get(name).copied()
    }
}

impl Index<HirId> for NameTable {
    type Output = str;

    #[track_caller]
    fn index(&self, index: HirId) -> &Self::Output {
        self.get_name(index).unwrap()
    }
}

impl<S: AsRef<str>> Index<S> for NameTable {
    type Output = HirId;

    #[track_caller]
    fn index(&self, index: S) -> &Self::Output {
        self.name_to_id.get(index.as_ref()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum StageError {
    MissingResource(ResourceName),
    NotAResource { id: HirId, name: String },
}

impl Display for StageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StageError::MissingResource(r) => {
                write!(f, "Unable to find the resource called \"{}\"", r,)
            },
            StageError::NotAResource { name, .. } => {
                write!(f, "\"{}\" is not a resource", name)
            },
        }
    }
}

impl std::error::Error for StageError {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Stage {
    Source(Source),
    Sink(Sink),
    Model(Model),
    ProcBlock(ProcBlock),
}

impl From<Sink> for Stage {
    fn from(s: Sink) -> Self { Stage::Sink(s) }
}

impl TryFrom<Stage> for Sink {
    type Error = ();

    fn try_from(stage: Stage) -> Result<Self, Self::Error> {
        match stage {
            Stage::Sink(s) => Ok(s),
            _ => Err(()),
        }
    }
}

impl From<Source> for Stage {
    fn from(s: Source) -> Self { Stage::Source(s) }
}

impl TryFrom<Stage> for Source {
    type Error = ();

    fn try_from(stage: Stage) -> Result<Self, Self::Error> {
        match stage {
            Stage::Source(s) => Ok(s),
            _ => Err(()),
        }
    }
}

impl From<Model> for Stage {
    fn from(m: Model) -> Self { Stage::Model(m) }
}

impl TryFrom<Stage> for Model {
    type Error = ();

    fn try_from(stage: Stage) -> Result<Self, Self::Error> {
        match stage {
            Stage::Model(m) => Ok(m),
            _ => Err(()),
        }
    }
}

impl From<ProcBlock> for Stage {
    fn from(p: ProcBlock) -> Self { Stage::ProcBlock(p) }
}

impl TryFrom<Stage> for ProcBlock {
    type Error = ();

    fn try_from(stage: Stage) -> Result<Self, Self::Error> {
        match stage {
            Stage::ProcBlock(pb) => Ok(pb),
            _ => Err(()),
        }
    }
}

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

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Type {
    Primitive(Primitive),
    /// The concrete type isn't yet known.
    Unknown,
    /// A multidimensional array of data.
    Buffer {
        underlying_type: HirId,
        dimensions: Vec<usize>,
    },
    /// This can be *any* type.
    Any,
}

impl Type {
    pub fn underlying_primitive(
        &self,
        types: &HashMap<HirId, Type>,
    ) -> Option<Primitive> {
        match self {
            Type::Primitive(p) => Some(*p),
            Type::Buffer {
                underlying_type, ..
            } => {
                let underlying_type = types.get(underlying_type)?;
                underlying_type.underlying_primitive(types)
            },
            Type::Unknown | Type::Any => None,
        }
    }

    pub fn rust_type_name(
        &self,
        types: &HashMap<HirId, Type>,
    ) -> Result<String, Error> {
        let mut buffer = Vec::new();
        self.write_rust_type_name(&mut buffer, types)?;
        let name = String::from_utf8(buffer)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        Ok(name)
    }

    fn write_rust_type_name<W: Write>(
        &self,
        w: &mut W,
        types: &HashMap<HirId, Type>,
    ) -> Result<(), Error> {
        match self {
            Type::Primitive(p) => write!(w, "{}", p.rust_name()),
            Type::Buffer {
                underlying_type,
                dimensions,
            } => write_rust_array_type_name(
                w,
                *underlying_type,
                dimensions,
                types,
            ),
            Type::Any | Type::Unknown => Err(Error::new(
                ErrorKind::Other,
                "The concrete type isn't known",
            )),
        }
    }
}

fn write_rust_array_type_name<W: Write>(
    w: &mut W,
    underlying_type: HirId,
    dimensions: &[usize],
    types: &HashMap<HirId, Type>,
) -> Result<(), Error> {
    match dimensions.split_first() {
        Some((dim, rest)) => {
            write!(w, "[")?;
            write_rust_array_type_name(w, underlying_type, rest, types)?;

            write!(w, "; {}]", dim)?;
            Ok(())
        },
        None => {
            let ty = types
                .get(&underlying_type)
                .ok_or_else(|| Error::new(ErrorKind::Other, "Unknown type"))?;
            ty.write_rust_type_name(w, types)?;
            Ok(())
        },
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Primitive {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    String,
}

impl Primitive {
    pub fn rust_name(self) -> &'static str {
        match self {
            Primitive::U8 => "u8",
            Primitive::I8 => "i8",
            Primitive::U16 => "u16",
            Primitive::I16 => "i16",
            Primitive::U32 => "u32",
            Primitive::I32 => "i32",
            Primitive::U64 => "u64",
            Primitive::I64 => "i64",
            Primitive::F32 => "f32",
            Primitive::F64 => "f64",
            Primitive::String => "&'static str",
        }
    }
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

impl ProcBlock {
    pub fn name(&self) -> &str {
        let full_name = self.path.sub_path.as_ref().unwrap_or(&self.path.base);

        let start_of_name = full_name.rfind('/').map(|ix| ix + 1).unwrap_or(0);

        &full_name[start_of_name..]
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pipeline {
    /// The edges associated with this pipeline.
    pub edges: HashSet<HirId>,
}

#[derive(
    Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Edge {
    pub type_id: HirId,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PipelineGraph {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub stage: Stage,
    /// The [`Slot`]s that this [`Node`] receives data from.
    pub input_slots: Vec<HirId>,
    /// The [`Slot`]s that this [`Node`] sends data to.
    pub output_slots: Vec<HirId>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Slot {
    pub element_type: HirId,
    pub input_node: HirId,
    pub output_node: HirId,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    pub source: Option<ResourceSource>,
    pub ty: ResourceType,
}

impl Resource {
    pub fn span(&self) -> Span {
        // TODO: Get span from serde_yaml
        Span::new(0, 0)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ResourceSource {
    Inline(String),
    FromDisk(PathBuf),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Image(pub Path);

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
