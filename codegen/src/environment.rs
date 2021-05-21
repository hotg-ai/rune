use std::{
    path::{Path, PathBuf},
    process::Command,
};
use anyhow::{Context, Error};
use serde::Serialize;

use crate::{Compilation, Project};

pub trait Environment {
    /// Compile a Rust project to WebAssembly, returning the contents of the
    /// compiled binary.
    fn compile(&mut self, project: Project) -> Result<Vec<u8>, Error>;

    /// Read a file from the file system, relative to the "current" directory.
    fn read_file(&mut self, filename: &Path) -> Result<Vec<u8>, Error>;

    /// Get a JSON object which contains information about the program compiling
    /// this Rune.
    fn build_info(&self) -> Option<serde_json::Value> { None }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultEnvironment {
    working_directory: PathBuf,
    current_directory: PathBuf,
    optimize: bool,
    rust_version: String,
}

impl DefaultEnvironment {
    pub fn for_compilation(c: &Compilation) -> Self {
        DefaultEnvironment {
            working_directory: c.working_directory.clone(),
            current_directory: c.current_directory.clone(),
            optimize: c.optimized,
            rust_version: crate::rustup::NIGHTLY_VERSION.clone(),
        }
    }

    fn write_toml(
        &self,
        path: impl AsRef<Path>,
        value: &impl Serialize,
    ) -> Result<(), Error> {
        let full_path = self.working_directory.join(path);

        if let Some(parent) = full_path.parent() {
            create_dir_all(parent)?;
        }

        let text = toml::to_string_pretty(value)
            .context("Unable to serialize the Cargo.toml file")?;
        write(full_path, text.as_bytes())?;

        Ok(())
    }

    fn write_project_to_disk(&self, project: &Project) -> Result<(), Error> {
        log::info!(
            "Generating the project in \"{}\"",
            self.working_directory.display()
        );

        self.write_toml(Path::new("Cargo.toml"), &project.manifest)
            .context("Unable to serialize the Cargo.toml file")?;
        self.write_toml(
            Path::new(".cargo").join("config.toml"),
            &project.config,
        )
        .context("Unable to serialize the config.toml file")?;

        for (path, model) in &project.models {
            let full_path = self.working_directory.join(path);
            write(full_path, model)?;
        }

        let lib_rs = self.working_directory.join("lib.rs");
        write(lib_rs, project.lib_rs.as_bytes())?;

        Ok(())
    }

    fn cargo_build(&self) -> Result<(), Error> {
        let mut cmd = Command::new("cargo");
        cmd.arg(format!("+{}", self.rust_version))
            .arg("build")
            .arg("--target=wasm32-unknown-unknown")
            .arg("--quiet")
            .current_dir(&self.working_directory);

        if self.optimize {
            cmd.arg("--release");
        }

        log::debug!("Executing {:?}", cmd);
        let status = cmd
            .status()
            .context("Unable to start `cargo`. Is it installed?")?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Cargo exited with a return code of {}",
                status.code().unwrap_or(1)
            ))
        }
    }

    fn rustfmt(&self) -> Result<(), Error> {
        log::debug!("Formatting the generate code");

        let status = Command::new("cargo")
            .arg("fmt")
            .current_dir(&self.working_directory)
            .status()
            .context("unable to call `cargo fmt`")?;

        anyhow::ensure!(
            status.success(),
            "`cargo fmt` failed with return code: {}",
            status.code().unwrap_or(1)
        );

        Ok(())
    }
}

impl Environment for DefaultEnvironment {
    fn compile(&mut self, project: Project) -> Result<Vec<u8>, Error> {
        self.write_project_to_disk(&project)
            .context("Unable to write the project to disk")?;

        if let Err(e) = self
            .rustfmt()
            .context("Unable to format the generated code")
        {
            log::warn!("{:?}", e);
        }

        self.cargo_build().context("Compilation failed")?;

        let dir = if self.optimize { "release" } else { "debug" };

        let binary = self
            .working_directory
            .join("target")
            .join("wasm32-unknown-unknown")
            .join(dir)
            .join(project.name.replace("-", "_"))
            .with_extension("wasm");

        read(binary)
    }

    fn read_file(&mut self, filename: &Path) -> Result<Vec<u8>, Error> {
        let path = self.current_directory.join(filename);
        read(&path)
    }
}

fn write(path: impl AsRef<Path>, data: &[u8]) -> Result<(), Error> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    std::fs::write(&path, data).with_context(|| {
        format!("Unable to write to \"{}\"", path.display())
    })?;
    log::debug!("Wrote {} bytes to \"{}\"", data.len(), path.display());

    Ok(())
}

fn create_dir_all(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();

    if path.exists() {
        return Ok(());
    }

    log::debug!("Making sure the \"{}\" directory exists", path.display());
    std::fs::create_dir_all(path).with_context(|| {
        format!("Unable to create the \"{}\" directory", path.display())
    })
}

fn read(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    let path = path.as_ref();

    let data = std::fs::read(&path)
        .with_context(|| format!("Unable to read \"{}\"", path.display()))?;
    log::debug!("Read {} bytes from \"{}\"", data.len(), path.display());

    Ok(data)
}
