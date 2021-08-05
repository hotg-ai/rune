#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod code;
mod config;
mod environment;
mod manifest;
mod models;
mod project;
pub mod rustup;

pub use crate::{
    environment::{Environment, DefaultEnvironment},
    project::Project,
};

pub const GRAPH_CUSTOM_SECTION: &str = ".rune_graph";
pub const VERSION_CUSTOM_SECTION: &str = ".rune_version";

use std::{
    path::{PathBuf},
    process::Command,
};
use anyhow::{Context, Error};
use hotg_rune_syntax::hir::Rune;

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
    pub verbosity: Verbosity,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

impl Verbosity {
    pub fn from_quiet_and_verbose(quiet: bool, verbose: bool) -> Option<Self> {
        match (verbose, quiet) {
            (true, false) => Some(Verbosity::Verbose),
            (false, true) => Some(Verbosity::Quiet),
            (false, false) => Some(Verbosity::Normal),
            (true, true) => None,
        }
    }

    /// Add a `--quiet` or `--verbose` argument to the command if necessary.
    pub(crate) fn add_flags(&self, cmd: &mut Command) {
        match self {
            Verbosity::Quiet => {
                cmd.arg("--quiet");
            },
            Verbosity::Verbose => {
                cmd.arg("--verbose");
            },
            Verbosity::Normal => {},
        }
    }
}

pub fn generate(c: Compilation) -> Result<Vec<u8>, Error> {
    let mut env = DefaultEnvironment::for_compilation(&c);
    generate_with_env(c, &mut env)
}

pub fn generate_with_env(
    c: Compilation,
    env: &mut dyn Environment,
) -> Result<Vec<u8>, Error> {
    let manifest = crate::manifest::generate(
        &c.rune,
        &c.name,
        &c.rune_project,
        &c.current_directory,
    );
    let config = crate::config::generate(c.optimized)
        .context("Unable to construct the \"config.toml\" file")?;
    let models = crate::models::load(&c.rune, env)
        .context("Unable to load the models")?;
    let lib_rs = crate::code::generate(&c.rune, env.build_info())
        .context("Unable to generate the \"lib.rs\" file")?;

    let project = Project {
        name: c.name,
        manifest,
        config,
        lib_rs,
        models,
    };

    env.compile(project)
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
