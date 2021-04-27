//! Checks to make sure our Rust implementation is identical to the original.

use std::{
    ffi::OsStr,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use noise_reduction::NoiseReduction;
use once_cell::sync::Lazy;
use quickcheck::TestResult;
use runic_types::{Tensor, Transform};

#[test]
fn check_for_perfect_parity() {
    quickcheck::quickcheck(perfect_parity as fn(_, _, _, _, _) -> _);
}

fn perfect_parity(
    smoothing_bits: u32,
    even_smoothing: u16,
    odd_smoothing: u16,
    min_signal_remaining: u16,
    input: Vec<u32>,
) -> TestResult {
    if input.len() <= 1 {
        return TestResult::discard();
    }
    if smoothing_bits as usize >= std::mem::size_of::<u32>() * 8 {
        return TestResult::discard();
    }
    // if input.iter().any(|i| *i >= (1 << 30)) {
    //     return TestResult::discard();
    // }

    let num_channels = input.len();

    let input = Tensor::new_vector(input);

    let c_result = transform(
        &ORIGINAL_IMPLEMENTATION,
        smoothing_bits,
        even_smoothing,
        odd_smoothing,
        min_signal_remaining,
        &input,
    );

    let mut proc_block = NoiseReduction::default()
        .with_smoothing_bits(smoothing_bits)
        .with_even_smoothing(even_smoothing)
        .with_odd_smoothing(odd_smoothing)
        .with_min_signal_remaining(min_signal_remaining)
        .with_num_channels(num_channels);

    let rust_result = proc_block.transform(input);

    assert_eq!(c_result, rust_result);

    TestResult::passed()
}

#[test]
fn bad_test_cases() {
    let inputs =
        vec![(0, 0, 0, 0, &[0, 0]), (0, 0, 0, 16904, &[4162845728, 0])];

    for args in inputs {
        let (
            smoothing_bits,
            even_smoothing,
            odd_smoothing,
            min_signal_remaining,
            input,
        ) = args;

        let result = perfect_parity(
            smoothing_bits,
            even_smoothing,
            odd_smoothing,
            min_signal_remaining,
            input.to_vec(),
        );

        assert!(!result.is_error(), "{:?}", args);
        assert!(!result.is_failure(), "{:?}", args);
    }
}

static ORIGINAL_IMPLEMENTATION: Lazy<PathBuf> = Lazy::new(|| {
    // we want to always compile the native binary on the first run
    let test_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");

    let noise_reduction_c = find_file_by_extension(&test_dir, "c".as_ref())
        .unwrap()
        .unwrap();
    let main_c = test_dir.join("main.cpp");

    let binary = test_dir.join("main");

    let status = Command::new("c++")
        .arg(&noise_reduction_c)
        .arg(&main_c)
        .arg("-I")
        .arg(&test_dir)
        .arg("-o")
        .arg(&binary)
        .status()
        .unwrap();

    assert!(status.success());

    binary
});

fn transform(
    binary: &Path,
    smoothing_bits: u32,
    even_smoothing: u16,
    odd_smoothing: u16,
    min_signal_remaining: u16,
    input: &Tensor<u32>,
) -> Tensor<u32> {
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();

    writeln!(stdin, "{}", smoothing_bits).unwrap();
    writeln!(stdin, "{}", even_smoothing).unwrap();
    writeln!(stdin, "{}", odd_smoothing).unwrap();
    writeln!(stdin, "{}", min_signal_remaining).unwrap();

    let elements = input.elements();
    writeln!(stdin, "{}", elements.len()).unwrap();

    for element in elements {
        writeln!(stdin, "{}", element).unwrap();
    }

    stdin.flush().unwrap();
    drop(stdin);

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let mut values: Vec<u32> = Vec::new();
    for line in std::str::from_utf8(&output.stdout).unwrap().lines() {
        let line = line.trim();
        if !line.is_empty() {
            values.push(line.parse().unwrap());
        }
    }

    Tensor::new_vector(values)
}

fn find_file_by_extension(
    dir: &Path,
    extension: &OsStr,
) -> io::Result<Option<PathBuf>> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(ret) = find_file_by_extension(&path, extension)? {
                return Ok(Some(ret));
            }
        } else if path.extension() == Some(extension) {
            return Ok(Some(path));
        }
    }

    Ok(None)
}
