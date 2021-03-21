//! The *High-level Internal Representation*.

use std::{
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
    path::PathBuf,
};
use codespan::Span;
use crate::ast::{ArgumentValue, Path};
use petgraph::graph::{DiGraph, IndexType, NodeIndex};

#[derive(Debug, Default, Clone)]
pub struct Rune {
    pub base_image: Option<Path>,
    pub graph: DiGraph<Stage, Edge>,
    pub pipelines: HashMap<HirId, Pipeline>,
    pub types: HashMap<HirId, Type>,
    pub names: NameTable,
    pub spans: HashMap<HirId, Span>,
    pub nodes_to_hir_id: HashMap<NodeIndex, HirId>,
    pub hir_id_to_nodes: HashMap<HirId, NodeIndex>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HirId(u32);

impl HirId {
    pub const ERROR: HirId = HirId(0);

    pub fn is_error(self) -> bool { self == HirId::ERROR }

    pub(crate) fn next(self) -> Self { HirId(self.0 + 1) }
}

impl Default for HirId {
    fn default() -> Self { HirId::ERROR }
}

unsafe impl IndexType for HirId {
    fn new(x: usize) -> Self { HirId(x.try_into().unwrap()) }

    fn index(&self) -> usize { self.0.try_into().unwrap() }

    fn max() -> Self { HirId(u32::max_value()) }
}

/// A table mapping names to [`HirId`]s.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NameTable {
    name_to_id: HashMap<String, HirId>,
    id_to_name: HashMap<HirId, String>,
}

impl NameTable {
    pub fn register(&mut self, name: &str, id: HirId) {
        if self.name_to_id.contains_key(name)
            || self.id_to_name.contains_key(&id)
        {
            unimplemented!("How do we want to signal duplicate names?");
        }

        self.name_to_id.insert(name.to_string(), id);
        self.id_to_name.insert(id, name.to_string());
    }

    pub fn get_name(&self, id: HirId) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }

    pub fn get_id(&self, name: &str) -> Option<HirId> {
        self.name_to_id.get(name).copied()
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Sink {
    pub kind: SinkKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SinkKind {
    Serial,
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub model_file: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    pub kind: SourceKind,
    pub parameters: HashMap<String, ArgumentValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceKind {
    Random,
    Accelerometer,
    Sound,
    Image,
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcBlock {
    pub path: Path,
    pub parameters: HashMap<String, ArgumentValue>,
}

impl ProcBlock {
    pub fn name(&self) -> &str {
        let full_name = self.path.sub_path.as_ref().unwrap_or(&self.path.base);

        let start_of_name = full_name.rfind("/").map(|ix| ix + 1).unwrap_or(0);

        &full_name[start_of_name..]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pipeline {
    /// The edges associated with this pipeline.
    pub edges: HashSet<HirId>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Edge {
    pub ty: HirId,
}
