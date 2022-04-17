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
pub(crate) mod inputs;

use std::path::Path;

pub use components::*;
use im::Vector;
use legion::Registry;

use crate::{
    codegen::{
        generate_rune_graph_section::rune_graph_section, inputs::CodegenInputs,
    },
    lowering::{Name, ResourceData},
    phases::Phase,
    serialize::RegistryExt,
};

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

#[salsa::query_group(CodegenGroup)]
pub trait Codegen: CodegenInputs {
    fn rust_toolchain_toml(&self) -> File;
    fn cargo_config(&self) -> File;
    fn cargo_toml(&self) -> File;
    fn model_files(&self) -> Vector<File>;

    fn resource_section(&self, name: Name, data: ResourceData)
        -> CustomSection;
    fn resource_sections(&self) -> Vector<CustomSection>;
    fn version_section(&self) -> Option<CustomSection>;
    fn rune_graph_section(&self) -> CustomSection;
    fn lib_rs(&self) -> File;

    fn custom_sections(&self) -> Vector<CustomSection>;

    fn files(&self) -> Vector<File>;
}

fn rust_toolchain_toml(_: &dyn Codegen) -> File {
    let rust_toolchain = crate::rust_toolchain();
    let contents = toml::to_vec(&rust_toolchain)
        .expect("We can always serialize a hard-coded TOML object");

    File::new("rust-toolchain.toml", contents)
}

fn cargo_config(db: &dyn Codegen) -> File {
    let ctx = db.build_context();
    generate_cargo_config::generate_config(ctx.optimized)
}

fn cargo_toml(db: &dyn Codegen) -> File {
    let features = db.feature_flags();
    let proc_blocks = db.all_proc_blocks();
    let ctx = db.build_context();

    generate_cargo_toml::generate(&features, proc_blocks.iter(), &ctx)
}

fn model_files(db: &dyn Codegen) -> Vector<File> {
    let mut files = Vector::new();

    for (name, data) in db.all_model_data() {
        let path = Path::new("models").join(name.as_str());
        let file = File::new(path, data.0.clone());
        files.push_back(file);
    }

    files
}

fn resource_section(
    _: &dyn Codegen,
    name: Name,
    data: ResourceData,
) -> CustomSection {
    generate_resource_section::inline_resource(&name, &data)
}

fn resource_sections(db: &dyn Codegen) -> Vector<CustomSection> {
    db.all_resource_data()
        .into_iter()
        .map(|(name, data)| db.resource_section(name, data))
        .collect()
}

fn version_section(db: &dyn Codegen) -> Option<CustomSection> {
    let ctx = db.build_context();
    generate_version_section::version_section(&ctx)
}

fn lib_rs(_db: &dyn Codegen) -> File { todo!() }

fn custom_sections(db: &dyn Codegen) -> Vector<CustomSection> {
    let mut sections = Vector::new();

    sections.push_back(db.rune_graph_section());
    if let Some(version) = db.version_section() {
        sections.push_back(version);
    }
    sections.extend(db.resource_sections());

    sections
}

fn files(db: &dyn Codegen) -> Vector<File> {
    let mut files = Vector::new();

    files.push_back(db.rust_toolchain_toml());
    files.push_back(db.cargo_config());
    files.push_back(db.cargo_toml());
    files.extend(db.model_files());

    files
}
