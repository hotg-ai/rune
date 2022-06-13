pub mod build;
mod graph;
mod inspect;
mod model_info;
pub mod run;
mod unstable;
mod version;

use codespan_reporting::term::termcolor;

pub use crate::{
    build::Build, graph::Graph, inspect::Inspect, model_info::ModelInfo,
    run::Run, unstable::Unstable, version::Version,
};

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

#[derive(
    Debug, Copy, Clone, PartialEq, strum::EnumVariantNames, strum::EnumString,
)]
#[strum(serialize_all = "snake_case")]
pub enum Format {
    Json,
    Text,
}
