use std::{
    ffi::OsStr,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
    process::Output,
};

use anyhow::{Context, Error};

use crate::{
    Outcome, TestContext,
    assertions::{
        Assertion, ExitSuccessfully, ExitUnsuccessfully, MatchStderr,
    },
};

pub fn load(test_root: &Path) -> Result<Vec<Test>, Error> {
    let runefiles =
        walkdir::WalkDir::new(test_root)
            .into_iter()
            .filter_map(|e| match e {
                Ok(entry)
                    if entry.path().file_name()
                        == Some(OsStr::new("Runefile.yml")) =>
                {
                    Some(entry.into_path())
                },
                _ => None,
            });

    let mut tests = Vec::new();

    for runefile in runefiles {
        let directory =
            runefile.parent().expect("We are at least 2 levels deep");
        let test = Test::for_directory(directory).with_context(|| {
            format!("Unable to load tests from \"{}\"", directory.display())
        })?;

        log::debug!("Found \"{}\"", test.name);
        tests.push(test);
    }

    Ok(tests)
}

#[derive(Debug, Clone, PartialEq)]
pub struct FullName {
    pub category: Category,
    pub exit_condition: ExitCondition,
    pub name: String,
}

impl Display for FullName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}/{}", self.category, self.exit_condition, self.name)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Test {
    pub directory: PathBuf,
    pub expected_output: Vec<MatchStderr>,
    pub name: FullName,
}

impl Test {
    pub fn for_directory(directory: impl AsRef<Path>) -> Result<Self, Error> {
        let directory = directory.as_ref();
        let directory = directory
            .canonicalize()
            .context("Unable to get the full path")?;

        let name = directory
            .file_name()
            .context("Unable to determine the directory's name")?
            .to_string_lossy()
            .into_owned();

        let parent = directory
            .parent()
            .and_then(|parent| parent.file_name())
            .and_then(|parent| parent.to_str())
            .context("Unable to determine the parent directory")?;

        let (category, exit_successfully) = match parent {
            "compile-pass" => (Category::Compile, true),
            "compile-fail" => (Category::Compile, false),
            "run-pass" => (Category::Run, true),
            "run-fail" => (Category::Run, false),
            _ => anyhow::bail!("Unable to determine the family, expected one of compile-pass, compile-fail, run-pass, or run-fail, but found \"{}\"", parent),
        };

        let expected_output = load_stderr_files(&directory)
            .context("Unable to load the *stderr files")?;

        let exit_condition = if exit_successfully {
            ExitCondition::Success(ExitSuccessfully)
        } else {
            ExitCondition::Fail(ExitUnsuccessfully)
        };

        let name = FullName {
            name,
            category,
            exit_condition,
        };

        Ok(Test {
            name,
            directory,
            expected_output,
        })
    }

    pub fn is_ignored(&self) -> bool { self.name.name.starts_with("_") }

    fn get_rune_output(&self, ctx: &TestContext) -> Result<Output, Error> {
        match self.name.category {
            Category::Run => {
                crate::run::rune_output(&self.name, &self.directory, ctx)
            },
            Category::Compile => {
                crate::compile::rune_output(&self.name, &self.directory, ctx)
            },
        }
    }

    pub fn run(&self, ctx: &TestContext) -> Outcome {
        if self.is_ignored() {
            return Outcome::Skipped;
        }

        let output = match self.get_rune_output(ctx) {
            Ok(output) => output,
            Err(e) => {
                return Outcome::Bug(
                    e.context("Unable to run the `rune` command"),
                )
            },
        };

        let mut errors = Vec::new();

        for assertion in self.assertions() {
            if let Err(e) = assertion.check_for_errors(&output) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Outcome::Pass
        } else {
            Outcome::Fail { errors, output }
        }
    }

    fn exit_condition(&self) -> &dyn Assertion {
        match &self.name.exit_condition {
            ExitCondition::Success(s) => s,
            ExitCondition::Fail(f) => f,
        }
    }

    fn assertions(&self) -> impl Iterator<Item = &dyn Assertion> + '_ {
        let expected_output =
            self.expected_output.iter().map(|a| a as &dyn Assertion);

        std::iter::once(self.exit_condition()).chain(expected_output)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExitCondition {
    Success(ExitSuccessfully),
    Fail(ExitUnsuccessfully),
}

impl Assertion for ExitCondition {
    fn check_for_errors(&self, output: &Output) -> Result<(), Error> {
        match self {
            ExitCondition::Success(e) => e.check_for_errors(output),
            ExitCondition::Fail(e) => e.check_for_errors(output),
        }
    }
}

impl Display for ExitCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExitCondition::Success(_) => write!(f, "pass"),
            ExitCondition::Fail(_) => write!(f, "fail"),
        }
    }
}

fn load_stderr_files(directory: &Path) -> Result<Vec<MatchStderr>, Error> {
    let mut stderr = Vec::new();

    for entry in directory.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if let Some(assertion) = MatchStderr::for_file(&path)
            .with_context(|| format!("Unable to load \"{}\"", path.display()))?
        {
            stderr.push(assertion);
        }
    }

    Ok(stderr)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Category {
    Run,
    Compile,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Category::Run => write!(f, "run"),
            Category::Compile => write!(f, "compile"),
        }
    }
}
