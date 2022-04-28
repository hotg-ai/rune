use std::{
    fmt::{self, Debug, Display, Formatter},
    path::Path,
    process::Output,
};

use anyhow::{Context, Error};
use regex::Regex;

pub trait Assertion: Debug {
    fn check_for_errors(&self, output: &Output) -> Result<(), Error>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StdioStream {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchStdioStream {
    expected: String,
    stream: StdioStream,
}

impl MatchStdioStream {
    pub fn for_file(filename: impl AsRef<Path>) -> Result<Option<Self>, Error> {
        let filename = filename.as_ref();

        let stream = match filename.extension().and_then(|s| s.to_str()) {
            Some("stderr") => StdioStream::Stderr,
            Some("stdout") => StdioStream::Stdout,
            _ => return Ok(None),
        };

        let expected =
            std::fs::read_to_string(filename).with_context(|| {
                format!("Unable to read \"{}\"", filename.display())
            })?;

        Ok(Some(MatchStdioStream::new(expected.trim(), stream)))
    }

    pub fn new(expected: impl Into<String>, stream: StdioStream) -> Self {
        MatchStdioStream {
            expected: expected.into(),
            stream,
        }
    }
}

impl Assertion for MatchStdioStream {
    fn check_for_errors(&self, output: &Output) -> Result<(), Error> {
        let raw = match self.stream {
            StdioStream::Stdout => &output.stdout,
            StdioStream::Stderr => &output.stderr,
        };
        let output = std::str::from_utf8(raw)
            .context("Unable to parse output as UTF-8")?;

        use super::*;
        if !reduce_precision(output).contains(&reduce_precision(&self.expected))
        {
            return Err(Error::from(MismatchedStdio {
                expected: self.expected.clone(),
                actual: output.to_string(),
            }));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MismatchedStdio {
    expected: String,
    actual: String,
}

impl Display for MismatchedStdio {
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

impl std::error::Error for MismatchedStdio {}

#[derive(Debug, Clone, PartialEq)]
pub struct ExitSuccessfully;

impl Assertion for ExitSuccessfully {
    fn check_for_errors(&self, output: &Output) -> Result<(), Error> {
        if output.status.success() {
            return Ok(());
        }

        let Output {
            status,
            stdout,
            stderr,
        } = output;

        let stdout = String::from_utf8_lossy(stdout).into_owned();
        let stderr = String::from_utf8_lossy(stderr).into_owned();
        let cause = SubprocessOutput { stdout, stderr };

        let e = match status.code() {
            Some(code) => Error::new(cause).context(format!(
                "Completed unsuccessfully with error code {}",
                code
            )),
            None => Error::new(cause).context("Completed unsuccessfully"),
        };

        Err(e)
    }
}

#[derive(Debug, Clone)]
pub struct SubprocessOutput {
    stdout: String,
    stderr: String,
}

impl Display for SubprocessOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let SubprocessOutput { stdout, stderr } = self;

        if !stdout.is_empty() {
            writeln!(f, "Stdout:")?;
            for line in stdout.lines() {
                writeln!(f, "  {}", line)?;
            }
        }

        if !stderr.is_empty() {
            writeln!(f, "Stderr:")?;
            for line in stderr.lines() {
                writeln!(f, "  {}", line)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for SubprocessOutput {}

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

fn reduce_precision(s: &str) -> String {
    let re = Regex::new(r"(\d*\.\d+)").unwrap();
    let mut result = s.to_string();
    let precision = 5;

    for m in re.find_iter(s) {
        let m = m.as_str();
        let decimal_at = m.chars().position(|c| c == '.').unwrap();
        let number_of_decimals = m.len() - decimal_at - 1;

        if number_of_decimals > precision {
            result =
                result.replace(m, m.split_at(decimal_at + precision + 1).0);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_precision() {
        assert_eq!(reduce_precision("5"), "5");
        assert_eq!(reduce_precision("abcdef.abcdef"), "abcdef.abcdef");
        assert_eq!(reduce_precision("5.12345678"), "5.12345");
        assert_eq!(
            reduce_precision("5.12345678 6.12345678"),
            "5.12345 6.12345"
        );
        assert_eq!(
            reduce_precision("5.12345678 6.12345678 7.1234567"),
            "5.12345 6.12345 7.12345"
        );
        assert_eq!(
            reduce_precision("abcdef 5.12345678 6.12345678"),
            "abcdef 5.12345 6.12345"
        )
    }
}
