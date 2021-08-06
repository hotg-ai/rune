use std::{collections::HashMap, path::PathBuf};
use cargo_toml::Manifest;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Project {
    pub name: String,
    pub manifest: Manifest,
    pub config: toml::Value,
    pub lib_rs: String,
    /// The `rust-toolchain.toml` file used to specify which version of Rust
    /// to use.
    pub rust_toolchain_toml: String,
    /// TensorFlow Lite binaries that should be included in the generated
    /// project, with paths relative to the project root.
    pub models: HashMap<PathBuf, Vec<u8>>,
}
