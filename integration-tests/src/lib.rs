mod assertions;
mod compile;
mod loader;
mod run;

use std::{
    fmt::{self, Debug, Display, Formatter},
    path::{Path, PathBuf},
    process::{Command, Output},
};

use anyhow::{Context, Error};
use rayon::prelude::*;

pub use crate::loader::{Category, ExitCondition, FullName, Test};

pub fn discover(test_directory: impl AsRef<Path>) -> Result<TestSuite, Error> {
    log::info!("Looking for tests");
    let test_directory = test_directory.as_ref();

    let tests = loader::load(test_directory)?;

    Ok(TestSuite { tests })
}

#[derive(Debug)]
pub struct TestSuite {
    tests: Vec<Test>,
}

impl TestSuite {
    pub fn run(&self, ctx: &TestContext, cb: &dyn Callbacks) {
        self.tests.par_iter().for_each(|test| {
            let name = &test.name;

            if !cb.should_run(name) {
                cb.on_skip(name);
                return;
            }

            match test.run(ctx) {
                Outcome::Skipped => cb.on_skip(name),
                Outcome::Pass => cb.on_pass(name),
                Outcome::Fail { errors, output } => {
                    cb.on_fail(name, errors, output)
                },
                Outcome::Bug(error) => cb.on_bug(name, error),
            }
        });
    }
}

pub trait Callbacks: Sync {
    fn on_pass(&self, name: &FullName);
    fn on_skip(&self, name: &FullName);
    fn on_bug(&self, name: &FullName, error: Error);
    fn on_fail(&self, name: &FullName, errors: Vec<Error>, output: Output);
    /// Should this test be executed?
    fn should_run(&self, _name: &FullName) -> bool { true }
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
    cache_dir: PathBuf,
    pub rune_binary: PathBuf,
    pub rune_project_dir: PathBuf,
    pub target_dir: PathBuf,
    pub engine: String,
}

impl TestContext {
    pub fn build(
        rune_project_dir: impl Into<PathBuf>,
        engine: String,
    ) -> Result<Self, Error> {
        TestContext::build_inner(
            rune_project_dir,
            engine,
            cfg!(debug_assertions),
        )
    }

    fn build_inner(
        rune_project_dir: impl Into<PathBuf>,
        engine: String,
        debug: bool,
    ) -> Result<Self, Error> {
        let rune_project_dir = rune_project_dir.into();
        let target_dir = rune_project_dir.join("target");
        let cache_dir = target_dir.join("integration-tests");

        log::debug!("Compiling `rune` in release mode");

        let mut cmd = Command::new("cargo");
        cmd.arg("build").arg("--package=hotg-rune-cli");

        if !debug {
            cmd.arg("--release");
        }

        let status = cmd
            .current_dir(&rune_project_dir)
            .status()
            .context("Unable to invoke cargo. Is it installed?")?;
        anyhow::ensure!(status.success(), "Compilation failed");

        let dir = if debug { "debug" } else { "release" };
        let rune_binary = target_dir.join(dir).join("rune");

        anyhow::ensure!(
            rune_binary.exists(),
            "The compiler should have generated \"{}\"",
            rune_binary.display()
        );

        Ok(TestContext {
            engine,
            cache_dir,
            rune_binary,
            rune_project_dir,
            target_dir,
        })
    }

    pub fn rune_cmd(&self) -> Command {
        let mut cmd = Command::new(&self.rune_binary);
        cmd.arg("--unstable")
            .arg("--colour=never")
            .arg("--rune-repo-dir")
            .arg(&self.rune_project_dir);

        cmd.env("RUST_LOG", "debug,wasmer_compiler_cranelift=warn");
        cmd
    }

    fn cache_dir(&self, name: &FullName) -> PathBuf {
        let FullName {
            category,
            exit_condition,
            name,
        } = name;
        let family = format!("{}-{}", category, exit_condition);
        self.cache_dir.join(family).join(name)
    }
}

#[derive(Debug, Clone)]
struct CommandOutput(Output);

impl CommandOutput {
    fn new(output: Output) -> Self { CommandOutput(output) }
}

impl Display for CommandOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let CommandOutput(Output {
            status,
            stderr,
            stdout,
        }) = self;

        match (status.success(), status.code()) {
            (false, Some(code)) => writeln!(f, "Command failed with {}", code)?,
            (true, Some(code)) => {
                writeln!(f, "Command exited successfully with {}", code)?
            },
            (true, None) => writeln!(f, "Command failed")?,
            (false, None) => writeln!(f, "Command exited successfully")?,
        }

        if let Ok(stdout) = std::str::from_utf8(stdout) {
            if !stdout.trim().is_empty() {
                writeln!(f, "\nStdout:\n{}", stdout)?;
            }
        }

        if let Ok(stderr) = std::str::from_utf8(stderr) {
            if !stderr.trim().is_empty() {
                writeln!(f, "\nStderr:\n{}", stderr)?;
            }
        }

        Ok(())
    }
}
