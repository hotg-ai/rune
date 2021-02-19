//! The *High-level Internal Representation*.

use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rune {
    pub base_image: Option<String>,
    pub sinks: HashMap<HirId, Sink>,
    pub models: HashMap<HirId, Model>,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Model {
    pub input: Type,
    pub output: Type,
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
