mod build;
// mod graph;
mod model_info;
mod run;
mod version;

use std::str::FromStr;
use anyhow::Error;
use codespan_reporting::term::termcolor;
use structopt::{clap::AppSettings, StructOpt};
use env_logger::{Env, WriteStyle};
use crate::{
    // graph::Graph,
    model_info::ModelInfo,
    run::Run,
    build::Build,
    version::{Format, Version},
};

const DEFAULT_RUST_LOG: &str = concat!(
    "info,",
    "rune=debug,",
    "rune_runtime=debug,",
    "rune_codegen=debug,",
    "rune_syntax=debug,",
);

fn main() -> Result<(), Error> {
    let Args {
        colour,
        cmd,
        version,
        verbose,
    } = Args::from_args();

    let env = Env::new().default_filter_or(DEFAULT_RUST_LOG);
    env_logger::builder()
        .parse_env(env)
        .format_timestamp_millis()
        .format_indent(Some(2))
        .write_style(colour.into())
        .init();

    match cmd {
        Some(Cmd::Build(build)) => build.execute(colour.into()),
        Some(Cmd::Run(run)) => run.execute(),
        // Some(Cmd::Graph(graph)) => graph.execute(colour.into()),
        Some(Cmd::Version(mut version)) => {
            version.verbose |= verbose;
            version.execute()
        },
        Some(Cmd::ModelInfo(m)) => model_info::model_info(m),
        None if version => {
            let v = Version {
                format: Format::Text,
                verbose,
            };
            v.execute()
        },
        None => {
            // we haven't been asked to print the version or execute a command,
            // so just print out the usage and bail.
            Args::clap().print_help()?;
            Ok(())
        },
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
    #[structopt(short = "V", long, help = "Print out version information")]
    version: bool,
    #[structopt(short, long, help = "Prints even more detailed information")]
    verbose: bool,
    #[structopt(subcommand)]
    cmd: Option<Cmd>,
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
#[structopt(setting(AppSettings::DisableVersion))]
enum Cmd {
    /// Compile a Runefile into a Rune.
    Build(Build),
    /// Run a rune.
    Run(Run),
    // /// Parse a Runefile and generate a DOT graph showing the pipelines
    // inside. Graph(Graph),
    /// Print detailed version information.
    Version(Version),
    /// Load a TensorFlow Lite model and print information about it.
    #[structopt(name = "model-info")]
    ModelInfo(ModelInfo),
}
