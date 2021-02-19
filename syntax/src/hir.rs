//! The *High-level Internal Representation*.

use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rune {
    pub base_image: Option<String>,
    pub sinks: HashMap<HirId, Sink>,
    pub sources: HashMap<HirId, Source>,
    pub models: HashMap<HirId, Model>,
    pub types: HashMap<HirId, Type>,
    pub pipelines: HashMap<HirId, Pipeline>,
    pub proc_blocks: HashMap<HirId, ProcBlock>,
    pub names: NameTable,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HirId(u32);

impl HirId {
    pub const ERROR: HirId = HirId(0);

    pub fn is_error(self) -> bool { self == HirId::ERROR }

    pub(crate) fn next(self) -> Self { HirId(self.0 + 1) }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sink {
    Serial,
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
            todo!("How do we want to signal duplicate names?");
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
pub struct Model {
    pub input: HirId,
    pub output: HirId,
    pub model_file: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// The concrete type isn't yet known.
    Unknown,
    /// A multidimensional array of data.
    Buffer {
        underlying_type: Primitive,
        dimensions: Vec<usize>,
    },
    /// This can be *any* type.
    Any,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Primitive {
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    pub kind: SourceKind,
    pub output_type: HirId,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceKind {
    Rand,
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pipeline {
    /// A linked list representing a pipeline.
    ///
    /// Note: We use a linked list to make sure it is impossible to create an
    /// illogical pipeline (e.g. with a sink in the middle) and so you can
    /// later include some sort of "merge" node for joining two
    /// sub-pipelines.
    pub last_step: PipelineNode,
    pub output_type: HirId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PipelineNode {
    Source(HirId),
    Model {
        model: HirId,
        previous: Box<PipelineNode>,
    },
    ProcBlock {
        model: HirId,
        previous: Box<PipelineNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcBlock {
    pub input: HirId,
    pub output: HirId,
    pub path: String,
    pub params: HashMap<String, String>,
}
