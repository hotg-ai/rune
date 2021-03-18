use assert_cmd::Command;
use std::path::{Path, PathBuf};
use tempfile::{NamedTempFile, TempDir};
use std::io::Write;

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
fn sine() {
    let build_dir = TempDir::new().unwrap();
    let runefile = sine_dir().join("Runefile");
    let rune = build_dir.path().join("sine.rune");

    // compile like normal
    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build")
        .arg(&runefile)
        .arg("--output")
        .arg(&rune)
        .arg("--cache-dir")
        .arg(build_dir.path())
        .unwrap();

    assert!(rune.exists());

    // This is the value we want to take the sine of
    let input: f32 = 0.8;

    let mut tempfile = NamedTempFile::new().unwrap();
    let data = input.to_le_bytes();
    tempfile.write(&data).unwrap();

    // then run the rune, making sure the RNG yields out value.
    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run")
        .arg(&rune)
        .arg("--capability")
        .arg(format!("random:{}", tempfile.path().display()));

    cmd.assert()
        .success()
        .code(0)
        // Note: sin(0.8) = 0.7173560909, but our model is kinda inaccurate so
        // we hard-code the value it actually yields.
        .stderr(predicates::str::contains("Serial: [6.972786e-1]"));
}

#[test]
fn gesture() {
    let gesture_dir = gesture_dir();
    let runefile = gesture_dir.join("Runefile");
    let build_dir = TempDir::new().unwrap();
    let rune = build_dir.path().join("gesture.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build")
        .arg(&runefile)
        .arg("--output")
        .arg(&rune)
        .arg("--cache-dir")
        .arg(build_dir.path())
        .unwrap();

    let example_wing = gesture_dir.join("example_ring.csv");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run")
        .arg(&rune)
        .arg("--capability")
        .arg(format!("accelerometer:{}", example_wing.display()));

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("ring"));
}

#[test]
fn yes_microspeech() {
    let microspeech_dir = microspeech_dir();
    let runefile = microspeech_dir.join("Runefile");
    let build_dir = TempDir::new().unwrap();
    let rune = build_dir.path().join("microspeech.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build")
        .arg(&runefile)
        .arg("--output")
        .arg(&rune)
        .arg("--cache-dir")
        .arg(build_dir.path())
        .unwrap();

    let wav = microspeech_dir
        .join("data")
        .join("yes_01d22d03_nohash_0.wav");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run")
        .arg(&rune)
        .arg(format!("--capability=sound:{}", wav.display()));

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("Serial: \"yes\""));
}

#[test]
fn no_microspeech() {
    let microspeech_dir = microspeech_dir();
    let runefile = microspeech_dir.join("Runefile");
    let build_dir = TempDir::new().unwrap();
    let rune = build_dir.path().join("microspeech.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build")
        .arg(&runefile)
        .arg("--output")
        .arg(&rune)
        .arg("--cache-dir")
        .arg(build_dir.path())
        .unwrap();

    let wav = microspeech_dir
        .join("data")
        .join("no_bf90a57a_nohash_1.wav");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run")
        .arg(&rune)
        .arg(format!("--capability=sound:{}", wav.display()));

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("Serial: \"no\""));
}
