use std::{collections::HashMap, path::PathBuf};
use cargo_toml::Manifest;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Project {
    pub manifest: Manifest,
    pub config: toml::Value,
    pub lib_rs: String,
    pub models: HashMap<PathBuf, Vec<u8>>,
}
