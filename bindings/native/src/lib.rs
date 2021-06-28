//! Native bindings to the `rune` project.

mod error;
mod image;
#[cfg(feature = "wasmer-runtime")]
mod wasmer_runtime;

#[safer_ffi::cfg_headers]
mod headers {
    use std::fmt::Write;
    use build_info::{BuildInfo, CrateInfo, GitInfo, VersionControl};

    build_info::build_info!(fn get_build_info);

    #[allow(dead_code)]
    fn banner() -> Result<String, Box<dyn std::error::Error>> {
        let mut crate_docs = String::new();

        writeln!(crate_docs, "/** \\file")?;

        let BuildInfo {
            crate_info:
                CrateInfo {
                    name,
                    version,
                    authors,
                    license,
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

        let banner = banner().unwrap();

        safer_ffi::headers::builder()
            .with_guard("_RUST_RUNE_NATIVE_")
            .with_banner(&banner)
            .to_file(&header_file)?
            .generate()?;

        Ok(())
    }
}
