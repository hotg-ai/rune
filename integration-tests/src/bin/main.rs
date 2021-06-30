use regex::Regex;
use anyhow::{Context, Error};
use rune_integration_tests::{Callbacks, FullName, TestContext};
use once_cell::sync::Lazy;
use structopt::StructOpt;
use std::{
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};
use env_logger::Env;

const DEFAULT_RUST_LOG: &str = "info";

fn main() -> Result<(), Error> {
    let env = Env::new().default_filter_or(DEFAULT_RUST_LOG);
    env_logger::builder()
        .parse_env(env)
        .format_timestamp_millis()
        .format_indent(Some(2))
        .init();
    let Args {
        test_directory,
        rune_project_dir,
        filters,
    } = Args::from_args();

    let tests = rune_integration_tests::discover(&test_directory)
        .context("Unable to discover tests")?;

    let ctx = TestContext::build(&rune_project_dir)
        .context("Unable to establish the test context")?;

    let mut printer = Printer {
        filters,
        ..Default::default()
    };
    tests.run(&ctx, &mut printer);

    printer.exit_code()
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(
        help = "The directory containing all your tests",
        default_value = &*DEFAULT_TEST_DIRECTORY,
        parse(from_os_str)
    )]
    test_directory: PathBuf,
    #[structopt(long = "rune-root",
    help = "The Rune repository's root directory",
    default_value = &*RUNE_PROJECT_DIR)]
    pub rune_project_dir: PathBuf,
    #[structopt(short, long = "filter", parse(try_from_str))]
    filters: Vec<Regex>,
}

static RUNE_PROJECT_DIR: Lazy<String> = Lazy::new(|| {
    // Try to use git because that's the most reliable
    if let Ok(output) = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .stdout(Stdio::piped())
        .output()
    {
        if output.status.success() {
            if let Ok(stdout) = std::str::from_utf8(&output.stdout) {
                return stdout.trim().to_string();
            }
        }
    }

    // otherwise, traverse parent directories until we find the .git directory
    if let Ok(current_dir) = std::env::current_dir() {
        for ancestor in current_dir.ancestors() {
            if ancestor.join(".git").is_dir() {
                return ancestor.display().to_string();
            }
        }
    }

    // Oh well, we tried...
    String::from(".")
});

static DEFAULT_TEST_DIRECTORY: Lazy<String> = Lazy::new(|| {
    Path::new(&*RUNE_PROJECT_DIR)
        .join("integration-tests")
        .display()
        .to_string()
});

#[derive(Debug, Default)]
pub struct Printer {
    pass: usize,
    skip: usize,
    fail: usize,
    bug: usize,
    filters: Vec<Regex>,
}

impl Printer {
    fn exit_code(self) -> Result<(), Error> {
        if self.fail == 0 && self.bug == 0 {
            Ok(())
        } else {
            Err(Error::msg("Test suite failed"))
        }
    }
}

impl Callbacks for Printer {
    fn should_run(&mut self, name: &FullName) -> bool {
        let name = name.to_string();

        self.filters.iter().any(|pattern| pattern.is_match(&name))
    }

    fn on_pass(&mut self, name: &FullName) {
        self.pass += 1;
        log::info!("{} ... ✓", name);
    }

    fn on_skip(&mut self, name: &FullName) {
        self.skip += 1;
        log::info!("{} ... (skip)", name);
    }

    fn on_bug(&mut self, name: &FullName, error: Error) {
        self.bug += 1;
        log::error!("{} ... 🐛", name);
        log::error!("Bug: {:?}", error);
    }

    fn on_fail(
        &mut self,
        name: &FullName,
        errors: Vec<Error>,
        _output: Output,
    ) {
        self.fail += 1;
        log::error!("{} ... ✗", name);

        for error in &errors {
            log::error!("{:?}", error);
        }
    }
}
