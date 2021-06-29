//! Native bindings to the `rune` project.

mod error;
mod image;
mod result;
#[cfg(feature = "wasmer-runtime")]
mod wasmer_runtime;

/// Header file generation.
///
/// This module contains a test that generates a header file for this library.
/// You can use the `RUNE_HEADER_FILE` environment variable to alter where it
/// will be written to (`<repo_root>/target/rune.h` by default).
#[safer_ffi::cfg_headers]
#[allow(dead_code)]
mod headers {
    use std::{
        fmt::Write,
        path::{Path, PathBuf},
    };
    use anyhow::{Context, Error};
    use build_info::{BuildInfo, CrateInfo, GitInfo, VersionControl};

    build_info::build_info!(fn get_build_info);

    fn banner() -> Result<String, Error> {
        let mut crate_docs = String::new();

        writeln!(crate_docs, "/** \\file")?;

        let BuildInfo {
            crate_info:
                CrateInfo {
                    name,
                    version,
                    authors,
                    license,
                    enabled_features,
                    ..
                },
            compiler,
            version_control,
            ..
        } = get_build_info();

        writeln!(crate_docs, " * {} v{}", name, version)?;
        writeln!(crate_docs)?;
        writeln!(crate_docs, " * Authors: {}", authors.join(", "))?;
        if let Some(license) = license {
            writeln!(crate_docs, " * License: {}", license)?;
        }
        if let Some(VersionControl::Git(GitInfo { commit_id, .. })) =
            version_control
        {
            writeln!(crate_docs, " * Commit: {}", commit_id)?;
        }
        writeln!(crate_docs, " * Compiler: {}", compiler)?;
        writeln!(
            crate_docs,
            " * Enabled Features: {}",
            enabled_features
                .iter()
                .map(|f| f.as_str())
                .filter(|&f| f != "c-headers")
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        writeln!(crate_docs, " *")?;

        for line in include_str!("lib.rs").lines() {
            if let Some(doc) = line.strip_prefix("//! ") {
                crate_docs.push_str(" * ");
                crate_docs.push_str(doc);
                crate_docs.push('\n');
            }
        }
        crate_docs.push_str(" */\n");

        Ok(crate_docs)
    }

    fn header_file() -> Result<PathBuf, Error> {
        if let Some(env) = std::env::var_os("RUNE_HEADER_FILE") {
            return Ok(PathBuf::from(env));
        }

        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let project_root = manifest_dir
            .ancestors()
            .find(|d| d.join(".git").exists())
            .context("Unable to determine the project root")?;

        let target_dir = project_root.join("target");
        Ok(target_dir.join("rune.h"))
    }

    #[test]
    fn generate_headers() -> Result<(), Error> {
        let header_file = header_file()?;
        let banner = banner()?;

        if let Some(parent) = header_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        safer_ffi::headers::builder()
            .with_guard("_RUST_RUNE_NATIVE_")
            .with_banner(&banner)
            .to_file(&header_file)?
            .generate()?;

        Ok(())
    }
}
