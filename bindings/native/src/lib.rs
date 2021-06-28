//! Native bindings to the `rune` project.

mod error;
mod image;
#[cfg(feature = "wasmer-runtime")]
mod wasmer_runtime;

/// The following test function is necessary for the header generation.
#[safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> Result<(), anyhow::Error> {
    use std::path::Path;
    use anyhow::Context;

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let project_root = manifest_dir
        .ancestors()
        .find(|d| d.join(".git").exists())
        .context("Unable to determine the project root")?;

    let target_dir = project_root.join("target");
    let header_file = target_dir.join("rune.h");

    safer_ffi::headers::builder()
        .with_guard("_RUST_RUNE_NATIVE_")
        .to_file(&header_file)?
        .generate()?;

    Ok(())
}
