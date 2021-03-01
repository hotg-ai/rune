use assert_cmd::Command;
use std::path::Path;

#[test]
fn compile_and_run_sine() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples")
        .join("sine");
    let runefile = dir.join("Runefile");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build").arg(&runefile).env("RUST_LOG", "debug");

    cmd.assert().success().code(0);

    let rune = dir.join("sine.rune");
    assert!(rune.exists());

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run").arg(&rune).env("RUST_LOG", "debug");

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("Output: [0.21078247]"));
}
