use assert_cmd::Command;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn project_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();

    for ancestor in manifest_dir.ancestors() {
        if ancestor.join(".git").is_dir() {
            return ancestor.to_path_buf();
        }
    }

    unreachable!(
        "Unable to determine the project's root directory. Where is \".git/\"?"
    );
}

fn example_dir() -> PathBuf { project_root().join("examples") }

fn person_detection_dir() -> PathBuf { example_dir().join("person_detection") }

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
    let runefiles = WalkDir::new(example_dir())
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_name() == "Runefile"
                || entry.file_name() == "Runefile.yml"
        });

    for runefile in runefiles {
        let mut cmd = Command::cargo_bin("rune").unwrap();
        cmd.arg("build")
            .arg(runefile.path())
            .arg("--cache-dir")
            .arg(project_root().join("target").join(env!("CARGO_PKG_NAME")))
            .assert()
            .success();
    }
}
