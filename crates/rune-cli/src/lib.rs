mod build;
mod graph;
mod inspect;
mod model_info;
mod run;
mod version;

use codespan_reporting::term::termcolor;
use env_logger::WriteStyle;

pub use crate::{
    graph::Graph, model_info::ModelInfo, inspect::Inspect, run::Run,
    build::Build, version::Version,
};

pub const DEFAULT_RUST_LOG: &str = concat!(
    "info,",
    "rune=debug,",
    "rune_cli=debug,",
    "rune_codegen=debug,",
    "rune_core=debug,",
    "rune_runtime=debug,",
    "rune_syntax=debug,",
    "rune_wasmer_runtime=debug,",
    "runicos_base_runtime=debug,",
    "regalloc=warn,",
);

#[derive(
    Debug, Copy, Clone, PartialEq, strum::EnumVariantNames, strum::EnumString,
)]
#[strum(serialize_all = "snake_case")]
pub enum ColorChoice {
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

#[derive(
    Debug, Copy, Clone, PartialEq, strum::EnumVariantNames, strum::EnumString,
)]
#[strum(serialize_all = "snake_case")]
pub enum Format {
    Json,
    Text,
}
