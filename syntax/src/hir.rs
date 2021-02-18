//! The *High-level Internal Representation*.

use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rune {
    pub base_image: Option<String>,
    pub labels: HashMap<String, HirId>,
    pub sinks: HashMap<HirId, Sink>,
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NameTable {
    name_to_id: HashMap<String, HirId>,
    id_to_name: HashMap<HirId, String>,
}

impl NameTable {
    pub fn add(&mut self, name: &str, id: HirId) {
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
