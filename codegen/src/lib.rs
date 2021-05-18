mod config;
mod environment;
mod manifest;
mod project;
mod rustup;

pub use crate::{
    environment::{Environment, DefaultEnvironment},
    project::Project,
};

use std::path::{PathBuf};
use anyhow::{Context, Error};
use rune_syntax::hir::Rune;

#[derive(Debug)]
pub struct Compilation {
    /// The name of the [`Rune`] being compiled.
    pub name: String,
    /// The [`Rune`] being compiled to WebAssembly.
    pub rune: Rune,
    /// A directory that can be used for any temporary artifacts.
    pub working_directory: PathBuf,
    /// The directory that all paths (e.g. to models) are resolved relative to.
    pub current_directory: PathBuf,
    /// How to find the Rune project.
    pub rune_project: RuneProject,
    /// Generate an optimized build.
    pub optimized: bool,
}

pub fn generate(c: Compilation) -> Result<Vec<u8>, Error> {
    let env = DefaultEnvironment::for_compilation(&c);
    generate_with_env(c, env)
}

pub fn generate_with_env(
    c: Compilation,
    _env: impl Environment,
) -> Result<Vec<u8>, Error> {
    let _manifest =
        crate::manifest::generate(&c.rune, &c.name, &c.rune_project);
    let _config = crate::config::generate(c.optimized)
        .context("Unable to construct the \"config.toml\" file")?;

    todo!();
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuneProject {
    Disk(PathBuf),
    Git {
        repo: String,
        specifier: GitSpecifier,
    },
}

impl RuneProject {
    pub const GITHUB_REPO: &'static str = "https://github.com/hotg-ai/rune";
}

impl Default for RuneProject {
    fn default() -> Self {
        RuneProject::Git {
            repo: String::from(RuneProject::GITHUB_REPO),
            specifier: GitSpecifier::Branch(String::from("master")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GitSpecifier {
    Commit(String),
    Tag(String),
    Branch(String),
}
