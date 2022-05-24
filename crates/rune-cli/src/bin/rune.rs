use anyhow::Error;
use hotg_rune_cli::{
    Build, ColorChoice, Format, Graph, Inspect, ModelInfo, Run, Unstable,
    Version,
};
use structopt::{clap::AppSettings, StructOpt};
use strum::VariantNames;
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Error> {
    let _ = dotenv::dotenv();
    human_panic::setup_panic!();

    if std::env::var_os("RUST_LOG").is_none() {
        // Some modules are known to generate loads of logs that aren't relevant
        // so we only log warnings by default.
        std::env::set_var(
            "RUST_LOG",
            [
                "info",
                "cranelift_codegen=warn",
                "regalloc=warn",
                "salsa=warn",
                "wasmer_compiler_cranelift=warn",
            ]
            .join(","),
        );
    }

    let Args {
        colour,
        cmd,
        version,
        unstable,
    } = Args::from_args();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    match cmd {
        Some(Cmd::Build(build)) => build.execute(colour.into(), unstable),
        Some(Cmd::Run(run)) => run.execute(),
        Some(Cmd::Graph(graph)) => graph.execute(),
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
    #[structopt(flatten)]
    unstable: Unstable,
    #[structopt(
        long,
        default_value = "auto",
        aliases = &["color"],
        parse(try_from_str),
        possible_values = ColorChoice::VARIANTS,
        global = true)
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
