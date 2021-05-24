use std::{fmt::Debug, process::Output};
use anyhow::Error;

pub trait Assertion: Debug {
    fn check_for_errors(&self, output: &Output) -> Vec<Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchStderr {
    expected: String,
}

impl MatchStderr {
    pub fn new(expected: impl Into<String>) -> Self {
        MatchStderr {
            expected: expected.into(),
        }
    }
}

impl Assertion for MatchStderr {
    fn check_for_errors(&self, _output: &Output) -> Vec<Error> { todo!() }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExitSuccessfully;

impl Assertion for ExitSuccessfully {
    fn check_for_errors(&self, output: &Output) -> Vec<Error> {
        let mut errors = Vec::new();

        if !output.status.success() {
            let err = match output.status.code() {
                Some(code) => anyhow::anyhow!(
                    "Completed unsuccessfully with error code {}",
                    code
                ),
                None => anyhow::anyhow!("Completed unsuccessfully"),
            };

            errors.push(err);
        }

        errors
    }
}
