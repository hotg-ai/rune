mod assertions;
mod compile;
mod fs;

pub use crate::{
    compile::{CompilationTest, discover_compile_pass},
    assertions::Assertion,
};

use std::{
    fmt::{self, Debug, Display, Formatter},
    path::{Path, PathBuf},
    process::{Command, Output},
};
use anyhow::{Context, Error};

pub fn discover(test_directory: impl AsRef<Path>) -> Result<TestSuite, Error> {
    log::info!("Looking for tests");
    let test_directory = test_directory.as_ref();

    let compile_pass =
        compile::discover_compile_pass(test_directory.join("compile-pass"))?;

    Ok(TestSuite { compile_pass })
}

#[derive(Debug)]
pub struct TestSuite {
    compile_pass: Vec<CompilationTest>,
}

impl TestSuite {
    pub fn run(&self, ctx: &TestContext, cb: &mut dyn Callbacks) {
        for test in &self.compile_pass {
            let name = Name {
                family: "compile-pass",
                name: &test.name,
            };

            match test.run(ctx) {
                Outcome::Skipped => cb.on_skip(name),
                Outcome::Pass => cb.on_pass(name),
                Outcome::Fail { errors, output } => {
                    cb.on_fail(name, errors, output)
                },
                Outcome::Bug(error) => cb.on_bug(name, error),
            }
        }
    }
}

pub trait Callbacks {
    fn on_pass(&mut self, name: Name<'_>);
    fn on_skip(&mut self, name: Name<'_>);
    fn on_bug(&mut self, name: Name<'_>, error: Error);
    fn on_fail(&mut self, name: Name<'_>, errors: Vec<Error>, output: Output);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Name<'a> {
    family: &'static str,
    name: &'a str,
}

impl<'a> Display for Name<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.family, self.name)
    }
}

#[derive(Debug)]
pub enum Outcome {
    Skipped,
    Pass,
    Fail { errors: Vec<Error>, output: Output },
    Bug(Error),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestContext {
    pub cache_dir: PathBuf,
    pub rune_binary: PathBuf,
    pub rune_project_dir: PathBuf,
    pub target_dir: PathBuf,
}

impl TestContext {
    pub fn release(
        rune_project_dir: impl Into<PathBuf>,
    ) -> Result<Self, Error> {
        let rune_project_dir = rune_project_dir.into();
        let target_dir = rune_project_dir.join("target");
        let cache_dir = target_dir.join("compiletest");

        log::debug!("Compiling `rune` in release mode");

        let status = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--package=rune")
            .current_dir(&rune_project_dir)
            .status()
            .context("Unable to invoke cargo. Is it installed?")?;
        anyhow::ensure!(status.success(), "Compilation failed");

        let rune_binary = target_dir.join("release").join("rune");

        anyhow::ensure!(
            rune_binary.exists(),
            "The compiler should have generated \"{}\"",
            rune_binary.display()
        );

        Ok(TestContext {
            cache_dir,
            rune_binary,
            rune_project_dir,
            target_dir,
        })
    }

    pub fn rune_cmd(&self) -> Command {
        let mut cmd = Command::new(&self.rune_binary);
        cmd.env("CARGO_TARGET_DIR", self.cache_dir.join("target"));

        cmd
    }
}
