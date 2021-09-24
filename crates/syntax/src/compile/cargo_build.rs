use std::{
    path::Path,
    process::{Command, Output, Stdio},
};

use crate::{BuildContext, Verbosity};

#[legion::system]
pub(crate) fn run(#[resource] ctx: &BuildContext) {
    let BuildContext {
        working_directory,
        optimized,
        verbosity,
        ..
    } = ctx;

    rustfmt(working_directory);
    build(working_directory, *optimized, *verbosity);
}

fn build(working_directory: &Path, optimized: bool, verbosity: Verbosity) {
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--manifest-path")
        .arg(working_directory.join("Cargo.toml"));

    if optimized {
        cmd.arg("--release");
    }

    verbosity.add_flags(&mut cmd);

    log::debug!("Executing {:?}", cmd);

    cmd.current_dir(working_directory);

    match cmd.status() {
        Ok(status) if status.success() => log::debug!("Compiled successfully"),
        Ok(status) => {
            log::error!(
                "Compilation failed with exit code {}",
                status.code().unwrap_or(1)
            );
        },
        Err(e) => {
            log::error!("Unable to compile the project: {}", e);
            log::error!("Is cargo installed?");
        },
    }
}

fn rustfmt(working_directory: &Path) {
    let output = Command::new("cargo")
        .arg("fmt")
        .arg("--manifest-path")
        .arg(working_directory.join("Cargo.toml"))
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(Output { status, .. }) if status.success() => {
            log::debug!("Formatted the generated code");
        },
        Ok(Output { status, stderr, .. }) => {
            log::warn!(
                "Rustfmt exited with status code: {}",
                status.code().unwrap_or(1)
            );
            let stderr = String::from_utf8_lossy(&stderr);
            log::warn!("Stderr:\n{}", stderr);
        },
        Err(e) => {
            log::warn!(
                "Unable to run \"cargo fmt\" on the generated project: {}",
                e
            );
            log::warn!("Is rustfmt installed?");
        },
    }
}
