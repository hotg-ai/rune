use std::{
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::{Context, Error};
use env_logger::Env;
use hotg_rune_integration_tests::{Callbacks, FullName, TestContext};
use log::LevelFilter;
use once_cell::sync::Lazy;
use regex::Regex;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::new().default_filter_or("info");
    env_logger::builder()
        .parse_env(env)
        .format_timestamp_millis()
        .format_indent(Some(2))
        // Some modules are known to generate loads of logs that aren't relevant
        .filter_module("cranelift_codegen", LevelFilter::Warn)
        .filter_module("regalloc", LevelFilter::Warn)
        .init();
    let Args {
        test_directory,
        rune_project_dir,
        filters,
        engine,
    } = Args::from_args();

    let tests = hotg_rune_integration_tests::discover(&test_directory)
        .context("Unable to discover tests")?;

    let ctx = TestContext::build(&rune_project_dir, engine)
        .context("Unable to establish the test context")?;

    let mut printer = Printer {
        filters,
        ..Default::default()
    };
    tests.run(&ctx, &mut printer);

    log::info!(
        "Test results... pass: {}, skip: {}, fail: {}, bugs: {}",
        printer.pass.load(Ordering::SeqCst),
        printer.skip.load(Ordering::SeqCst),
        printer.fail.load(Ordering::SeqCst),
        printer.bug.load(Ordering::SeqCst),
    );

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
    rune_project_dir: PathBuf,
    #[structopt(short, long = "filter", parse(try_from_str))]
    filters: Vec<Regex>,
    #[structopt(short, long, default_value = "wasmer")]
    engine: String,
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
    pass: AtomicUsize,
    skip: AtomicUsize,
    fail: AtomicUsize,
    bug: AtomicUsize,
    filters: Vec<Regex>,
}

impl Printer {
    fn exit_code(self) -> Result<(), Box<dyn std::error::Error>> {
        if self.fail.load(Ordering::SeqCst) == 0
            && self.bug.load(Ordering::SeqCst) == 0
        {
            Ok(())
        } else {
            Err("Test suite failed".into())
        }
    }
}

impl Callbacks for Printer {
    fn should_run(&self, name: &FullName) -> bool {
        if self.filters.is_empty() {
            return true;
        }

        let name = name.to_string();

        self.filters.iter().any(|pattern| pattern.is_match(&name))
    }

    fn on_pass(&self, name: &FullName) {
        self.pass.fetch_add(1, Ordering::SeqCst);
        log::info!("{} ... ✓", name);
    }

    fn on_skip(&self, name: &FullName) {
        self.skip.fetch_add(1, Ordering::SeqCst);
        log::info!("{} ... (skip)", name);
    }

    fn on_bug(&self, name: &FullName, error: Error) {
        self.bug.fetch_add(1, Ordering::SeqCst);
        log::error!("{} ... 🐛", name);
        log::error!("Bug: {:?}", error);
    }

    fn on_fail(&self, name: &FullName, errors: Vec<Error>, _output: Output) {
        self.fail.fetch_add(1, Ordering::SeqCst);
        log::error!("{} ... ✗", name);

        for error in &errors {
            log::error!("{:?}", error);
        }
    }
}
