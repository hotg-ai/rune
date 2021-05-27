use std::{
    ffi::OsStr,
    fmt::{self, Debug, Display, Formatter},
    path::Path,
    process::Output,
};
use anyhow::{Context, Error};

pub trait Assertion: Debug {
    fn check_for_errors(&self, output: &Output) -> Result<(), Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchStderr {
    expected: String,
}

impl MatchStderr {
    pub fn for_file(filename: impl AsRef<Path>) -> Result<Option<Self>, Error> {
        let filename = filename.as_ref();

        if filename.extension() != Some(OsStr::new("stderr")) {
            return Ok(None);
        }

        let expected =
            std::fs::read_to_string(filename).with_context(|| {
                format!("Unable to read \"{}\"", filename.display())
            })?;

        Ok(Some(MatchStderr::new(expected.trim())))
    }

    pub fn new(expected: impl Into<String>) -> Self {
        MatchStderr {
            expected: expected.into(),
        }
    }
}

impl Assertion for MatchStderr {
    fn check_for_errors(&self, output: &Output) -> Result<(), Error> {
        let stderr = std::str::from_utf8(&output.stderr)
            .context("Unable to parse stderr as UTF-8")?;

        if !stderr.contains(&self.expected) {
            return Err(Error::from(MismatchedStderr {
                expected: self.expected.clone(),
                actual: stderr.to_string(),
            }));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MismatchedStderr {
    expected: String,
    actual: String,
}

impl Display for MismatchedStderr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Unable to find the expected output in stderr.")?;
        writeln!(f, "Expected:")?;

        for line in self.expected.lines() {
            writeln!(f, "\t{}", line)?;
        }
        writeln!(f)?;

        writeln!(f, "Actual:")?;

        for line in self.actual.lines() {
            writeln!(f, "\t{}", line)?;
        }

        Ok(())
    }
}

impl std::error::Error for MismatchedStderr {}

#[derive(Debug, Clone, PartialEq)]
pub struct ExitSuccessfully;

impl Assertion for ExitSuccessfully {
    fn check_for_errors(&self, output: &Output) -> Result<(), Error> {
        if !output.status.success() {
            match output.status.code() {
                Some(code) => anyhow::bail!(
                    "Completed unsuccessfully with error code {}",
                    code
                ),
                None => anyhow::bail!("Completed unsuccessfully"),
            };
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExitUnsuccessfully;

impl Assertion for ExitUnsuccessfully {
    fn check_for_errors(&self, output: &Output) -> Result<(), Error> {
        anyhow::ensure!(
            !output.status.success(),
            "The command should have failed but it exited successfully"
        );

        Ok(())
    }
}
