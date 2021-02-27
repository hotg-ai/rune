use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;

const RUST_LOG: &str = "debug,cranelift_codegen=warn";

#[test]
fn compile_and_run_sine() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples")
        .join("sine");
    let runefile = dir.join("Runefile");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build").arg(&runefile).env("RUST_LOG", RUST_LOG);

    cmd.assert().success().code(0);

    let rune = dir.join("sine.rune");
    assert!(rune.exists());

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run").arg(&rune).env("RUST_LOG", RUST_LOG);

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("Output: [0.21078247]"));
}

#[test]
fn compile_and_run_gesture() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples")
        .join("gesture");
    let runefile = dir.join("Runefile");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build").arg(&runefile).env("RUST_LOG", RUST_LOG);

    cmd.assert().success().code(0);

    let rune = dir.join("gesture.rune");
    assert!(rune.exists());

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run").arg(&rune).env("RUST_LOG", RUST_LOG);

    // FIXME: We should probably check the output for some well-known string
    // indicating success.
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::is_empty().not());
}
