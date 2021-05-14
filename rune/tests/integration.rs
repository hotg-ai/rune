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

fn noop_dir() -> PathBuf { example_dir().join("noop") }
fn sine_dir() -> PathBuf { example_dir().join("sine") }
fn gesture_dir() -> PathBuf { example_dir().join("gesture") }
fn microspeech_dir() -> PathBuf { example_dir().join("microspeech") }
fn person_detection_dir() -> PathBuf { example_dir().join("person_detection") }

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
        .stderr(predicates::str::contains(r#"Serial: {"type_name":"f32","channel":2,"elements":[6.972786e-1],"dimensions":[1]}"#));
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
fn up_microspeech() {
    let microspeech_dir = microspeech_dir();
    let runefile = microspeech_dir.join("Runefile");
    let build_dir = TempDir::new().unwrap();
    let rune = build_dir.path().join("microspeech.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build")
        .arg(&runefile)
        .arg("--output")
        .arg(&rune)
        .unwrap();

    let wav = microspeech_dir
        .join("data")
        .join("up")
        .join("84d1e469_nohash_0.wav");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run")
        .arg(&rune)
        .arg(format!("--capability=sound:{}", wav.display()));

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains(
            r#"Serial: {"type_name":"&str","channel":2,"elements":["up"],"dimensions":[1]}"#,
        ));
}

#[test]
fn down_microspeech() {
    let microspeech_dir = microspeech_dir();
    let runefile = microspeech_dir.join("Runefile");
    let build_dir = TempDir::new().unwrap();
    let rune = build_dir.path().join("microspeech.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build")
        .arg(&runefile)
        .arg("--output")
        .arg(&rune)
        .unwrap();

    let wav = microspeech_dir
        .join("data")
        .join("down")
        .join("cd85758f_nohash_0.wav");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run")
        .arg(&rune)
        .arg(format!("--capability=sound:{}", wav.display()));

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains(
            r#"Serial: {"type_name":"&str","channel":2,"elements":["down"],"dimensions":[1]}"#,
        ));
}

#[test]
fn noop() {
    let build_dir = TempDir::new().unwrap();
    let runefile = noop_dir().join("Runefile");
    let rune = build_dir.path().join("noop.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build")
        .arg(&runefile)
        .arg("--output")
        .arg(&rune)
        .unwrap();

    assert!(rune.exists());

    let mut bytes = Vec::new();
    for number in &[0_i32, 1, 2, 3] {
        bytes.extend_from_slice(&number.to_ne_bytes());
    }
    let mut tempfile = NamedTempFile::new().unwrap();
    tempfile.write(&bytes).unwrap();

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run")
        .arg(&rune)
        .arg("--capability")
        .arg(format!("raw:{}", tempfile.path().display()));

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains(r#"Serial: {"type_name":"i32","channel":1,"elements":[0,1,2,3],"dimensions":[4]}"#));
}

#[cfg(target_os = "linux")] // See https://github.com/hotg-ai/rune/issues/131
#[test]
fn person_detection() {
    let person_detection_dir = person_detection_dir();
    let runefile = person_detection_dir.join("Runefile");
    let build_dir = TempDir::new().unwrap();
    let rune = build_dir.path().join("person_detection.rune");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("build")
        .arg(&runefile)
        .arg("--output")
        .arg(&rune)
        .unwrap();

    let image = person_detection_dir.join("image_grayscale.png");

    let mut cmd = Command::cargo_bin("rune").unwrap();
    cmd.arg("run")
        .arg(&rune)
        .arg("--capability")
        .arg(format!("image:{}", image.display()));

    cmd.assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("\"person_prob\""));
}

#[test]
fn build_all_examples() {
    for entry in example_dir().read_dir().unwrap() {
        let entry = entry.unwrap();
        let runefile = entry.path().join("Runefile");

        if runefile.exists() {
            let mut cmd = Command::cargo_bin("rune").unwrap();

            cmd.arg("build").arg(&runefile).assert().success();
        }
    }
}
