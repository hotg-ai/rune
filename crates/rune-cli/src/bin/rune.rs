use anyhow::Error;
use structopt::{clap::AppSettings, StructOpt};
use env_logger::Env;
use strum::VariantNames;
use hotg_rune_cli::{
    Build, ColorChoice, DEFAULT_RUST_LOG, Format, Graph, Inspect, ModelInfo,
    Run, Version,
};

fn main() -> Result<(), Error> {
    let Args {
        colour,
        cmd,
        version,
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
        Some(Cmd::Graph(graph)) => graph.execute(colour.into()),
        Some(Cmd::Version(version)) => version.execute(),
        Some(Cmd::ModelInfo(m)) => m.execute(),
        Some(Cmd::Inspect(i)) => i.execute(),
        None if version => {
            let v = Version {
                format: Format::Text,
                verbose: false,
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
        possible_values = ColorChoice::VARIANTS)
    ]
    colour: ColorChoice,
    /// Prints out version information.
    #[structopt(short = "V", long)]
    version: bool,
    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Debug, Clone, PartialEq, StructOpt)]
#[structopt(setting(AppSettings::DisableVersion))]
enum Cmd {
    /// Compile a Runefile into a Rune.
    Build(Build),
    /// Execute a Rune on the current device.
    Run(Run),
    /// Print version information about the rune CLI.
    Version(Version),
    /// Load a TensorFlow Lite model and print information about it.
    #[structopt(name = "model-info")]
    ModelInfo(ModelInfo),
    /// Show which capabilities are used by a compiled Rune.
    Inspect(Inspect),
    /// Visualise the flow of data through a Rune.
    Graph(Graph),
}
