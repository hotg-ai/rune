use std::{
    path::Path,
    process::{Command, Stdio},
};
use tempfile::TempDir;
use rune_syntax::Diagnostics;
use rune_codegen::{Compilation, RuneProject};

#[test]
fn execute_cpp_example() {
    let temp = TempDir::new().unwrap();
    let temp = temp.path();

    let ffi_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let rune_project_dir = ffi_dir.parent().unwrap().to_path_buf();
    let header = temp.join("rune.h");
    let executable = temp.join("main");
    let rune = temp.join("test.rune");
    let lib = rune_project_dir
        .join("target")
        .join("debug")
        .join("librune.a");

    // First, generate the header file for our bindings
    cbindgen::generate(ffi_dir).unwrap().write_to_file(&header);

    // make sure the library was built
    let status = Command::new("cargo")
        .arg("build")
        .arg("--manifest-path")
        .arg(ffi_dir.join("Cargo.toml"))
        .arg("--quiet")
        .status()
        .unwrap();
    assert!(status.success());

    // Note: We may want to use something like cmake so this is more likely
    // to work on other platforms.
    let compiler =
        std::env::var("CXX").unwrap_or_else(|_| String::from("clang++"));
    let status = Command::new(compiler)
        .arg("-g")
        .arg("-std=c++17")
        .arg("-Wall")
        .arg("-I")
        .arg(temp)
        .arg("-o")
        .arg(&executable)
        .arg(ffi_dir.join("examples").join("main.cpp"))
        .arg(&lib)
        .arg("-ldl")
        .arg("-lpthread")
        .status()
        .unwrap();
    assert!(status.success());

    // Compile a Runefile
    let runefile = r"
            FROM runicos/base
            CAPABILITY<u8[8]> random RAND
            OUT serial
            RUN random serial";
    let parsed = rune_syntax::parse(runefile).unwrap();
    let mut diags = Diagnostics::new();
    let analysed = rune_syntax::analyse(&parsed, &mut diags);
    assert!(!diags.has_errors());
    let c = Compilation {
        name: String::from("test"),
        rune: analysed,
        working_directory: temp.join("rust"),
        current_directory: ffi_dir.to_path_buf(),
        rune_project: RuneProject::Disk(rune_project_dir),
        optimized: false,
    };
    let compiled = rune_codegen::generate(c).unwrap();
    // and write it to the temporary directory
    std::fs::write(&rune, &compiled).unwrap();

    // Execute the rune and capture its output
    let output = Command::new(&executable)
        .arg(&rune)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);

    let logs = String::from_utf8(output.stdout).unwrap();
    let expected = "[0,0,40,66,0,0,40,66]"; // [42.0, 42.0] as LE bytes
    assert!(
        logs.contains(expected),
        "Unable to find {:?} in output: \n\n{}",
        expected,
        logs
    );
}
