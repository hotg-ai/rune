use std::{
    path::Path,
    process::{Command, Output, Stdio},
    sync::Mutex,
};
use legion::systems::CommandBuffer;
use crate::{
    BuildContext, Verbosity,
    compile::{CompilationResult, CompileError, CompiledBinary},
};

#[legion::system]
pub(crate) fn run(cmd: &mut CommandBuffer, #[resource] ctx: &BuildContext) {
    let BuildContext {
        working_directory,
        optimized,
        verbosity,
        name,
        ..
    } = ctx;

    rustfmt(working_directory);

    let result = build(name, working_directory, *optimized, *verbosity);

    // Note: the exec_mut() method takes a Fn() closure and not a FnOnce(), so
    // we need to use a Mutex<Option<_>> to move the result.
    let result = Mutex::new(Some(result));
    cmd.exec_mut(move |_, res| {
        let result = result.lock().unwrap().take().unwrap();
        res.insert(CompilationResult(result));
    })
}

fn build(
    name: &str,
    working_directory: &Path,
    optimized: bool,
    verbosity: Verbosity,
) -> Result<CompiledBinary, CompileError> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--manifest-path")
        .arg(working_directory.join("Cargo.toml"))
        .arg("--target=wasm32-unknown-unknown");

    if optimized {
        cmd.arg("--release");
    }

    verbosity.add_flags(&mut cmd);

    log::debug!("Executing {:?}", cmd);

    cmd.current_dir(working_directory);

    let status = cmd.status().map_err(CompileError::DidntStart)?;

    if !status.success() {
        return Err(CompileError::BuildFailed(status));
    }

    log::debug!("Compiled successfully");

    let config = if optimized { "release" } else { "debug" };

    let wasm = working_directory
        .join("target")
        .join("wasm32-unknown-unknown")
        .join(config)
        .join(name.replace("-", "_"))
        .with_extension("wasm");

    std::fs::read(&wasm)
        .map(CompiledBinary::from)
        .map_err(|error| CompileError::UnableToReadBinary { path: wasm, error })
}

fn rustfmt(working_directory: &Path) {
    let mut cmd = Command::new("cargo");
    cmd.arg("fmt")
        .arg("--manifest-path")
        .arg(working_directory.join("Cargo.toml"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    log::debug!("Executing {:?}", cmd);

    let output = cmd.output();

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
