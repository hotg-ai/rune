use assert_cmd::Command;
use predicates::prelude::*;
use std::path::{Path, PathBuf};

fn example_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples")
}

fn sine_dir() -> PathBuf { example_dir().join("sine") }
fn gesture_dir() -> PathBuf { example_dir().join("gesture") }
fn microspeech_dir() -> PathBuf { example_dir().join("microspeech") }

#[test]
fn compile_sine() {
    let dir = sine_dir();
    let runefile = dir.join("Runefile");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build").arg(&runefile);

    cmd.assert().success().code(0);

    let rune = dir.join("sine.rune");
    assert!(rune.exists());
}

#[test]
#[ignore = "We need to return a model's output, seed the RNG, then send it to the serial OUT"]
fn run_sine() {
    let dir = sine_dir();
    let runefile = dir.join("Runefile");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build").arg(&runefile).unwrap();

    let rune = dir.join("sine.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run").arg(&rune);

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("Output: [0.21078247]"));
}

#[test]
fn compile_gesture() {
    let dir = gesture_dir();
    let runefile = dir.join("Runefile");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build").arg(&runefile);

    cmd.assert().success().code(0);

    let rune = dir.join("gesture.rune");
    assert!(rune.exists());
}

#[test]
#[ignore = "The ACCEL capability isn't implemented yet"]
fn run_gesture() {
    let dir = gesture_dir();
    let runefile = dir.join("Runefile");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build").arg(&runefile).unwrap();

    let rune = dir.join("gesture.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run").arg(&rune);

    // FIXME: We should probably check the output for some well-known string
    // indicating success.
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::is_empty().not());
}

#[test]
fn identify_yes_microspeech() {
    let dir = microspeech_dir();
    let runefile = dir.join("Runefile");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build").arg(&runefile).unwrap();

    let rune = dir.join("microspeech.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run").arg(&rune).arg(format!(
        "--capability=sound:{}",
        dir.join("data").join("yes_01d22d03_nohash_0.wav").display()
    ));

    // FIXME: We should probably check the output for some well-known string
    // indicating success.
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("Serial -> \"yes\""));
}

#[test]
fn identify_no_microspeech() {
    let dir = microspeech_dir();
    let runefile = dir.join("Runefile");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build").arg(&runefile).unwrap();

    let rune = dir.join("microspeech.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run").arg(&rune).arg(format!(
        "--capability=sound:{}",
        dir.join("data").join("no_bf90a57a_nohash_1.wav").display()
    ));

    // FIXME: We should probably check the output for some well-known string
    // indicating success.
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("Serial -> \"no\""));
}
