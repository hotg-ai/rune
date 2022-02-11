use std::{
    path::{Path, PathBuf},
    process::Command,
};
use anyhow::{Error, Context};
use hotg_rune_proc_blocks::{
    ProcBlockDescriptor, TransformDescriptor, TensorDescriptors,
    TensorDescriptor,
};
use crate::{Format, inspect::wasm_custom_sections};

pub fn inspect(format: Format, proc_block_dir: &Path) -> Result<(), Error> {
    log::info!("Inspecting \"{}\"", proc_block_dir.display());

    let dest = cache_dir(proc_block_dir);

    log::debug!("Writing probe to \"{}\"", dest.display());

    generate_project(&dest, proc_block_dir)
        .context("Unable to generate the probe project")?;

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .current_dir(&dest);

    log::debug!("Executing {:?}", cmd);

    let status = cmd
        .status()
        .with_context(|| format!("Unable to start cargo"))?;

    if !status.success() {
        anyhow::bail!("Compilation failed");
    }

    let binary = dest
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("debug")
        .join("probe.wasm");
    let wasm = std::fs::read(&binary)
        .with_context(|| format!("Unable to read \"{}\"", binary.display()))?;

    log::debug!("Read {} bytes from \"{}\"", wasm.len(), binary.display());

    let sections = wasm_custom_sections(&wasm)
        .context("Unable to parse the WebAssembly module")?;

    let section = sections
        .iter()
        .find(|s| s.name == ProcBlockDescriptor::CUSTOM_SECTION_NAME)
        .context("Unable to locate the proc-block's metadata")?;

    let metadata: ProcBlockDescriptor = serde_json::from_slice(section.data)
        .context("Unable to parse the proc-block metadata")?;

    match format {
        Format::Json => {
            let json = serde_json::to_string_pretty(&metadata)?;
            println!("{}", json);
            Ok(())
        },
        Format::Text => {
            print_descriptor(&metadata);
            Ok(())
        },
    }
}

fn print_descriptor(metadata: &ProcBlockDescriptor) {
    let ProcBlockDescriptor {
        type_name,
        description,
        available_transforms,
    } = metadata;

    println!("{}", type_name);
    println!("{}", "-".repeat(type_name.chars().count()));

    for line in description.lines() {
        println!("{}", line);
    }

    if !description.is_empty() {
        println!();
    }

    if available_transforms.is_empty() {
        println!("(no transforms registered)");
    } else {
        println!("Transforms:");

        for transform in available_transforms.iter() {
            print_transform(transform);
        }
    }
}

fn print_transform(transform: &TransformDescriptor) {
    let TransformDescriptor { inputs, outputs } = transform;

    print!("  inputs: ");
    print_tensor_descriptors(inputs);
    print!("  outputs: ");
    print_tensor_descriptors(outputs);
}

fn print_tensor_descriptors(tensors: &TensorDescriptors) {
    if tensors.len() == 1 {
        print_tensor_descriptor(&tensors[0]);
        println!();
    } else {
        print!("(");
        for (i, tensor) in tensors.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }

            print_tensor_descriptor(tensor);
        }
        println!(")");
    }
}

fn print_tensor_descriptor(tensor: &TensorDescriptor) {
    let TensorDescriptor {
        element_type,
        dimensions,
    } = tensor;
    print!("{}[{}]", element_type, dimensions);
}

const CARGO_TOML_TEMPLATE: &str = r#"
[package]
name = "probe"
version = "0.0.0"
edition = "2018"

[lib]
path = "lib.rs"
crate-type = ["cdylib"]

[dependencies]
"$NAME" = { path = "$PATH" }
"#;

const LIB_RS_TEMPLATE: &str = "
pub use $NAME::*;
";

fn generate_project(dest: &PathBuf, filename: &Path) -> Result<(), Error> {
    let path = filename.to_string_lossy();
    let name = filename
        .file_stem()
        .and_then(|s| s.to_str())
        .context("Unable to determine the package's name")?;

    write(
        dest.join("Cargo.toml"),
        CARGO_TOML_TEMPLATE
            .replace("$PATH", &path)
            .replace("$NAME", name),
    )?;

    let name = name.replace("-", "_");
    write(dest.join("lib.rs"), LIB_RS_TEMPLATE.replace("$NAME", &name))?;

    write(
        dest.join("rust-toolchain.toml"),
        hotg_rune_compiler::rust_toolchain().to_string(),
    )?;

    Ok(())
}

fn write(
    file: impl AsRef<Path>,
    contents: impl AsRef<[u8]>,
) -> Result<(), Error> {
    let file = file.as_ref();

    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!("Unable to create the \"{}\" directory", parent.display())
        })?;
    }

    std::fs::write(file, contents.as_ref())
        .with_context(|| format!("Unable to write to \"{}\"", file.display()))
}

fn cache_dir(project: &Path) -> PathBuf {
    let cache_dir = dirs::cache_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| project.join("target").join("probe"));

    cache_dir.join("rune").join("proc-blocks")
}
