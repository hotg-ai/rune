//! The code generation phase.
//!
//! This takes the parsed and analysed Rune and generates all the necessary
//! files to make a Rust project.

mod compile_generated_project;
mod generate_cargo_config;
mod generate_cargo_toml;
mod generate_custom_sections;
mod generate_lib_rs;
mod generate_manifest_function;
mod generate_model_files;

use crate::phases::Phase;

pub fn phase() -> Phase {
    Phase::new()
        .and_then(generate_cargo_config::run_system)
        .and_then(generate_cargo_toml::run_system)
        .and_then(generate_model_files::run_system)
        .and_then(generate_manifest_function::run_system)
        .and_then(generate_lib_rs::run_system)
        .and_then(compile_generated_project::run_system)
}
