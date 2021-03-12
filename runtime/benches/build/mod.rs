use std::path::Path;
use anyhow::Error;
use once_cell::sync::Lazy;
use rune_codegen::Compilation;
use rune_runtime::{DefaultEnvironment, Runtime};
use rune_syntax::Diagnostics;

pub const SINE_RUNEFILE: &str = include_str!("../../../examples/sine/Runefile");
pub const GESTURE_RUNEFILE: &str =
    include_str!("../../../examples/gesture/Runefile");
pub const MICROSPEECH_RUNEFILE: &str =
    include_str!("../../../examples/microspeech/Runefile");

pub const WING: &str =
    include_str!("../../../examples/gesture/example_wing.csv");
pub const RING: &str =
    include_str!("../../../examples/gesture/example_ring.csv");
pub const SLOPE: &str =
    include_str!("../../../examples/gesture/example_slope.csv");

pub static SINE_DEBUG: Lazy<Vec<u8>> =
    Lazy::new(|| compile("sine", SINE_RUNEFILE, false));
pub static SINE_RELEASE: Lazy<Vec<u8>> =
    Lazy::new(|| compile("sine", SINE_RUNEFILE, true));
pub static GESTURE_DEBUG: Lazy<Vec<u8>> =
    Lazy::new(|| compile("gesture", GESTURE_RUNEFILE, false));
pub static GESTURE_RELEASE: Lazy<Vec<u8>> =
    Lazy::new(|| compile("gesture", GESTURE_RUNEFILE, true));
pub static MICROSPEECH_DEBUG: Lazy<Vec<u8>> =
    Lazy::new(|| compile("microspeech", MICROSPEECH_RUNEFILE, false));
pub static MICROSPEECH_RELEASE: Lazy<Vec<u8>> =
    Lazy::new(|| compile("microspeech", MICROSPEECH_RUNEFILE, true));

pub fn compile(name: &str, runefile: &str, optimized: bool) -> Vec<u8> {
    let parsed = rune_syntax::parse(runefile).unwrap();

    let mut diags = Diagnostics::new();
    let rune = rune_syntax::analyse(0, &parsed, &mut diags);

    assert!(!diags.has_errors());

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let sine_dir = repo_root.join("examples").join(name);

    let working_dir = tempfile::tempdir().unwrap();

    let compilation = Compilation {
        name: String::from(name),
        rune,
        rune_project_dir: repo_root.to_path_buf(),
        current_directory: sine_dir,
        working_directory: working_dir.path().to_path_buf(),
        optimized,
    };

    rune_codegen::generate(compilation).unwrap()
}

pub fn load_csv(csv: &str) -> Result<Vec<[f32; 3]>, Error> {
    let mut samples = Vec::new();

    for line in csv.lines() {
        let words: Vec<_> = line.split(",").map(|s| s.trim()).collect();

        match words.as_slice() {
            [a, b, c] => samples.push([a.parse()?, b.parse()?, c.parse()?]),
            [] => {},
            _ => anyhow::bail!("Expected 3 columns, found {}", words.len()),
        }
    }

    Ok(samples)
}

fn gesture_runtime(wasm: &[u8], accelerometer_samples: &str) -> Runtime {
    let mut env = DefaultEnvironment::default();
    env.set_accelerometer_data(load_csv(accelerometer_samples).unwrap());

    Runtime::load(wasm, env).unwrap()
}

pub fn wing_gesture_runtime() -> Runtime {
    gesture_runtime(&GESTURE_RELEASE, WING)
}

pub fn wing_gesture_runtime_debug() -> Runtime {
    gesture_runtime(&GESTURE_DEBUG, WING)
}

pub fn ring_gesture_runtime() -> Runtime {
    gesture_runtime(&GESTURE_RELEASE, RING)
}

pub fn ring_gesture_runtime_debug() -> Runtime {
    gesture_runtime(&GESTURE_DEBUG, RING)
}

pub fn slope_gesture_runtime() -> Runtime {
    gesture_runtime(&GESTURE_RELEASE, SLOPE)
}

pub fn slope_gesture_runtime_debug() -> Runtime {
    gesture_runtime(&GESTURE_DEBUG, SLOPE)
}
