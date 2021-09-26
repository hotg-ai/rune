//! The code generation phase.
//!
//! This takes the parsed and analysed Rune and generates all the necessary
//! files to make a Rust project.

mod compile_generated_project;
mod components;
mod generate_cargo_config;
mod generate_cargo_toml;
mod generate_lib_rs;
mod generate_model_files;
mod generate_resource_section;
mod generate_rune_graph_section;
mod generate_rust_toolchain_toml;
mod generate_version_section;

pub use components::*;
use legion::Registry;

use crate::{phases::Phase, serialize::RegistryExt};

pub fn phase() -> Phase {
    Phase::new()
        .and_then(generate_rust_toolchain_toml::run_system)
        .and_then(generate_cargo_config::run_system)
        .and_then(generate_cargo_toml::run_system)
        .and_then(generate_model_files::run_system)
        .and_then(generate_resource_section::run_system)
        .and_then(generate_version_section::run_system)
        .and_then(generate_rune_graph_section::run_system)
        .and_then(generate_lib_rs::run_system)
        .and_then(compile_generated_project::run_system)
}

pub(crate) fn register_components(registry: &mut Registry<String>) {
    registry
        .register_with_type_name::<CustomSection>()
        .register_with_type_name::<RuneGraph>()
        .register_with_type_name::<RuneVersion>()
        .register_with_type_name::<File>();
}
