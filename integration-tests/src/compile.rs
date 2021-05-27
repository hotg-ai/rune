use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Stdio,
};

use anyhow::{Context, Error};

use crate::{
    Outcome, TestCase, TestContext,
    assertions::{
        Assertion, ExitSuccessfully, ExitUnsuccessfully, MatchStderr,
    },
};

pub fn discover_compile_pass(
    dir: impl AsRef<Path>,
) -> Result<Vec<CompilationTest>, Error> {
    let mut test_cases = discover_compile_tests(dir)?;

    for test_case in &mut test_cases {
        test_case.name.insert_str(0, "compile-pass/");
        test_case.assertions.push(Box::new(ExitSuccessfully));
    }

    Ok(test_cases)
}

pub fn discover_compile_fail(
    dir: impl AsRef<Path>,
) -> Result<Vec<CompilationTest>, Error> {
    let mut test_cases = discover_compile_tests(dir)?;

    for test_case in &mut test_cases {
        test_case.name.insert_str(0, "compile-fail/");
        test_case.assertions.push(Box::new(ExitUnsuccessfully));
    }

    Ok(test_cases)
}

pub fn discover_compile_tests(
    dir: impl AsRef<Path>,
) -> Result<Vec<CompilationTest>, Error> {
    let dir = dir.as_ref();
    log::debug!("Looking for tests in \"{}\"", dir.display());

    if !dir.exists() {
        log::debug!("The directory doesn't exist");
        return Ok(Vec::new());
    }

    let entries = dir
        .read_dir()
        .with_context(|| {
            format!("Unable to read the contents of \"{}\"", dir.display())
        })?
        .filter_map(Result::ok);

    let mut test_cases = Vec::new();

    for entry in entries {
        let path = entry.path();

        if let Some(test_case) = compilation_test(&path)
            .with_context(|| format!("Unable to check\"{}\"", path.display()))?
        {
            log::debug!("Found \"{}\"", test_case.name);
            test_cases.push(test_case);
        }
    }

    Ok(test_cases)
}

fn compilation_test(
    test_case_dir: &Path,
) -> Result<Option<CompilationTest>, Error> {
    log::debug!(
        "Checking if \"{}\" contains a test case",
        test_case_dir.display()
    );

    let meta = test_case_dir
        .metadata()
        .context("Unable to read the directory's metadata")?;

    if !meta.is_dir() {
        log::debug!("Not a directory. Skipping.");
        return Ok(None);
    }

    let name = test_case_dir
        .file_name()
        .context("Unable to get the directory name")?
        .to_string_lossy()
        .into_owned();

    let runefile = test_case_dir.join("Runefile.yml");

    if !runefile.exists() {
        log::debug!(
            "The directory doesn't contain a \"Runefile.yml\". Skipping."
        );
        return Ok(None);
    }

    let mut assertions: Vec<Box<dyn Assertion>> = Vec::new();

    for entry in test_case_dir
        .read_dir()
        .context("Unable to read the test case directory contents")?
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if path.extension() == Some(OsStr::new("stderr")) {
            let expected = crate::fs::read_to_string(&path)?;
            assertions
                .push(Box::new(MatchStderr::new(expected.trim().to_string())));
        }
    }

    Ok(Some(CompilationTest {
        name,
        directory: test_case_dir.to_path_buf(),
        assertions,
        runefile,
    }))
}

#[derive(Debug)]
pub struct CompilationTest {
    pub name: String,
    pub directory: PathBuf,
    pub runefile: PathBuf,
    pub assertions: Vec<Box<dyn Assertion>>,
}

impl TestCase for CompilationTest {
    fn name(&self) -> &str { &self.name }

    fn run(&self, ctx: &TestContext) -> Outcome {
        if self.name.starts_with("_") {
            return Outcome::Skipped;
        }

        log::debug!("Testing {}", self.name);

        let mut cmd = ctx.rune_cmd();

        cmd.arg("build")
            .arg(&self.runefile)
            .arg("--debug")
            .arg("--cache-dir")
            .arg(ctx.cache_dir.join("compile").join(&self.name))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        log::debug!("Executing {:?}", cmd);

        let output = match cmd.output() {
            Ok(output) => output,
            Err(e) => {
                return Outcome::Bug(
                    Error::from(e).context("Unable to run `rune build`"),
                )
            },
        };

        let mut errors = Vec::new();

        for assertion in &self.assertions {
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
}
