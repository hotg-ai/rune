use std::{collections::HashMap, path::PathBuf};
use cargo_toml::Manifest;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Project {
    pub name: String,
    pub manifest: Manifest,
    pub config: toml::Value,
    pub lib_rs: String,
    /// TensorFlow Lite binaries that should be included in the generated
    /// project, with paths relative to the project root.
    pub models: HashMap<PathBuf, Vec<u8>>,
}
