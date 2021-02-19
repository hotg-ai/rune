use std::path::{Path, PathBuf};

use rune_codegen::Compilation;
use rune_syntax::Diagnostics;
use tempfile::TempDir;

#[test]
fn we_can_compile_the_sine_example() {
    let runefile = include_str!("../../examples/sine/Runefile");
    let parsed = rune_syntax::parse(runefile).unwrap();

    let mut diags = Diagnostics::new();
    let rune = rune_syntax::analyse(0, &parsed, &mut diags);
    assert!(!diags.has_errors());

    let temp = TempDir::new().unwrap();
    let sine_dir = project_root().join("examples").join("sine");

    let _blob = rune_codegen::generate(Compilation {
        rune,
        current_directory: sine_dir,
        working_directory: temp.path().to_path_buf(),
    })
    .unwrap();
}

fn project_root() -> PathBuf {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));

    for parent in path.ancestors() {
        if parent.join(".git").exists() {
            return parent.to_path_buf();
        }
    }

    panic!("Unable to find the project root");
}
