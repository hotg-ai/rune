mod build;
mod run;

use std::str::FromStr;
use build::Build;
use anyhow::Error;
use codespan_reporting::term::termcolor;
use structopt::StructOpt;
use env_logger::{Env, WriteStyle};
use run::Run;

const DEFAULT_RUST_LOG: &str = concat!(
    "info,",
    "rune=debug,",
    "rune_runtime=debug,",
    "rune_codegen=debug,",
    "rune_syntax=debug,",
);

fn main() -> Result<(), Error> {
    let Args { colour, cmd } = Args::from_args();

    let env = Env::new().default_filter_or(DEFAULT_RUST_LOG);
    env_logger::builder()
        .parse_env(env)
        .format_timestamp_millis()
        .format_indent(Some(2))
        .write_style(colour.into())
        .init();

    match cmd {
        Cmd::Build(build) => build.execute(colour.into()),
        Cmd::Run(run) => run.execute(),
    }
}

#[derive(Debug, Clone, PartialEq, StructOpt)]
pub struct Args {
    #[structopt(
        long,
        default_value = "auto",
        aliases = &["color"],
        parse(try_from_str),
        possible_values = &["always", "never", "auto"])
    ]
    colour: ColorChoice,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ColorChoice {
    Always,
    Auto,
    Never,
}

impl From<ColorChoice> for termcolor::ColorChoice {
    fn from(c: ColorChoice) -> termcolor::ColorChoice {
        match c {
            ColorChoice::Always => termcolor::ColorChoice::Always,
            ColorChoice::Auto => termcolor::ColorChoice::Auto,
            ColorChoice::Never => termcolor::ColorChoice::Never,
        }
    }
}

impl From<ColorChoice> for WriteStyle {
    fn from(c: ColorChoice) -> WriteStyle {
        match c {
            ColorChoice::Always => WriteStyle::Always,
            ColorChoice::Auto => WriteStyle::Auto,
            ColorChoice::Never => WriteStyle::Never,
        }
    }
}

impl FromStr for ColorChoice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "always" => Ok(ColorChoice::Always),
            "auto" => Ok(ColorChoice::Auto),
            "never" => Ok(ColorChoice::Never),
            __ => Err(Error::msg("Invalid colour choice")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, StructOpt)]
enum Cmd {
    /// Compile a Runefile into a Rune.
    Build(Build),
    /// Run a rune.
    Run(Run),
}
