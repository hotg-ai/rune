use anyhow::{Context, Error};
use compiletest::{Callbacks, Name, TestContext};
use once_cell::sync::Lazy;
use structopt::StructOpt;
use std::{
    path::PathBuf,
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
    let args = Args::from_args();

    let tests = compiletest::discover(&args.test_directory)
        .context("Unable to discover tests")?;
    println!("{:?}", tests);

    let ctx = TestContext::release(&args.rune_project_dir)
        .context("Unable to establish the test context")?;

    let mut printer = Printer::default();
    tests.run(&ctx, &mut printer);

    printer.exit_code()
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(
        help = "The directory containing all your tests",
        default_value = ".",
        parse(from_os_str)
    )]
    test_directory: PathBuf,
    #[structopt(long = "rune-root",
    help = "The Rune repository's root directory",
    default_value = &*RUNE_PROJECT_DIR)]
    pub rune_project_dir: PathBuf,
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

#[derive(Debug, Default)]
pub struct Printer {
    pass: usize,
    skip: usize,
    fail: usize,
    bug: usize,
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
    fn on_pass(&mut self, name: Name<'_>) {
        self.pass += 1;
        log::info!("{} ... ✓", name);
    }

    fn on_skip(&mut self, name: Name<'_>) {
        self.skip += 1;
        log::info!("{} ... (skip)", name);
    }

    fn on_bug(&mut self, name: Name<'_>, error: Error) {
        self.bug += 1;
        log::error!("{} ... 🐛", name);
        log::error!("Bug: {:?}", error);
    }

    fn on_fail(&mut self, name: Name<'_>, errors: Vec<Error>, output: Output) {
        self.fail += 1;
        log::error!("{} ... ✗", name);
        for error in &errors {
            log::error!("{:?}", error);
        }

        if !output.stderr.is_empty() {
            if let Ok(stderr) = std::str::from_utf8(&output.stderr) {
                log::error!("{}", stderr);
            }
        }
    }
}
