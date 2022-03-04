use std::{
    ffi::CStr,
    os::raw::c_int,
    path::Path,
    process::Command,
    ptr::{self, NonNull},
};

use once_cell::sync::Lazy;
use rune_native::*;

static SINE_RUNE: Lazy<Vec<u8>> = Lazy::new(|| {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
    let sine_dir = workspace_root.join("examples").join("sine");
    let runefile = sine_dir.join("Runefile.yml");

    Command::new(env!("CARGO"))
        .arg("rune")
        .arg("build")
        .arg(&runefile);

    let rune = sine_dir.join("sine.rune");

    std::fs::read(&rune).unwrap()
});

#[test]
fn load_the_sine_rune() {
    unsafe {
        let mut runtime: *mut Runtime = ptr::null_mut();
        let cfg = Config {
            wasm: SINE_RUNE.as_ptr(),
            wasm_len: SINE_RUNE.len() as c_int,
            engine: Engine::Wasm3,
        };

        let error = rune_runtime_load(&cfg, &mut runtime);
        assert!(error.is_null());

        assert!(!runtime.is_null());
        rune_runtime_free(runtime);
    }
}

#[test]
fn inspect_inputs() {
    unsafe {
        let mut runtime: *mut Runtime = ptr::null_mut();
        let cfg = Config {
            wasm: SINE_RUNE.as_ptr(),
            wasm_len: SINE_RUNE.len() as c_int,
            engine: Engine::Wasm3,
        };

        let error = rune_runtime_load(&cfg, &mut runtime);
        assert!(error.is_null());

        let mut inputs: *mut Metadata = ptr::null_mut();

        let error = rune_runtime_inputs(runtime, &mut inputs);
        assert!(error.is_null());
        let inputs = NonNull::new(inputs);

        let num_inputs = rune_metadata_node_count(inputs);
        assert_eq!(1, num_inputs);

        let node = rune_metadata_get_node(inputs, 0);
        assert!(node.is_some());

        assert_eq!(1, rune_node_metadata_id(node));
        let kind = rune_node_metadata_kind(node);
        assert_eq!(CStr::from_ptr(kind).to_string_lossy(), "RAW");

        assert_eq!(1, rune_node_metadata_num_arguments(node));
        let arg_name = rune_node_metadata_get_argument_name(node, 0);
        assert_eq!(CStr::from_ptr(arg_name).to_string_lossy(), "length");
        let arg_value = rune_node_metadata_get_argument_value(node, 0);
        assert_eq!(CStr::from_ptr(arg_value).to_string_lossy(), "4");

        rune_metadata_free(inputs.unwrap().as_ptr());
        rune_runtime_free(runtime);
    }
}

#[test]
fn set_inputs() {
    unsafe {
        let mut runtime: *mut Runtime = ptr::null_mut();
        let cfg = Config {
            wasm: SINE_RUNE.as_ptr(),
            wasm_len: SINE_RUNE.len() as c_int,
            engine: Engine::Wasm3,
        };

        let error = rune_runtime_load(&cfg, &mut runtime);
        assert!(error.is_null());

        rune_runtime_free(runtime);
    }
}
