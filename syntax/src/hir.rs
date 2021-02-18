use std::collections::HashMap;

use crate::ast::Ident;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rune {
    pub base_image: Option<String>,
    pub labels: HashMap<String, HirId>,
    pub sinks: HashMap<HirId, Sink>,
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
