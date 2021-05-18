use std::path::{Path, PathBuf};
use anyhow::Error;

use crate::{Compilation, Project};

pub trait Environment {
    /// Compile a Rust project to WebAssembly, returning the contents of the
    /// compiled binary.
    fn compile(&mut self, project: Project) -> Result<Vec<u8>, Error>;
    fn read_file(&mut self, filename: &Path) -> Result<Vec<u8>, Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultEnvironment {
    working_directory: PathBuf,
    current_directory: PathBuf,
    optimize: bool,
    rust_version: String,
}

impl DefaultEnvironment {
    pub fn for_compilation(c: &Compilation) -> Self {
        DefaultEnvironment {
            working_directory: c.working_directory.clone(),
            current_directory: c.current_directory.clone(),
            optimize: c.optimized,
            rust_version: crate::rustup::NIGHTLY_VERSION.clone(),
        }
    }
}

impl Environment for DefaultEnvironment {
    fn compile(&mut self, _project: Project) -> Result<Vec<u8>, Error> {
        todo!()
    }

    fn read_file(&mut self, _filename: &Path) -> Result<Vec<u8>, Error> {
        todo!()
    }
}
