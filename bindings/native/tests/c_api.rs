use std::{
    ffi::CStr,
    os::raw::c_int,
    path::Path,
    process::Command,
    ptr::{self, NonNull},
    slice,
};

use hotg_rune_runtime::ElementType;
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
fn run_prediction_with_missing_input() {
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

        let error = rune_runtime_predict(runtime);
        assert!(!error.is_null());

        let msg = rune_error_to_string(error);
        assert_eq!(
            CStr::from_ptr(msg).to_str(),
            Ok("Unable to read the input")
        );
        libc::free(msg.cast());

        rune_error_free(error);
        rune_runtime_free(runtime);
    }
}

#[test]
fn inspect_input_metadata() {
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

        assert_eq!(1, rune_node_id(node));
        let kind = rune_node_kind(node);
        assert_eq!(CStr::from_ptr(kind).to_string_lossy(), "RAW");

        assert_eq!(1, rune_node_argument_count(node));
        let arg_name = rune_node_get_argument_name(node, 0);
        assert_eq!(CStr::from_ptr(arg_name).to_string_lossy(), "length");
        let arg_value = rune_node_get_argument_value(node, 0);
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

        let mut tensors: *mut InputTensors = ptr::null_mut();
        let error = rune_runtime_input_tensors(runtime, &mut tensors);
        assert!(error.is_null());
        assert!(!tensors.is_null());
        let tensors = NonNull::new(tensors);

        assert_eq!(rune_input_tensor_count(tensors), 0);

        let dims = [1, 4];
        let tensor = rune_input_tensors_insert(
            tensors,
            1,
            ElementType::U8,
            dims.as_ptr(),
            2,
        );
        assert!(!tensor.is_null());
        let tensor = NonNull::new(tensor);
        assert_eq!(rune_tensor_rank(tensor), 2);
        assert_eq!(rune_tensor_element_type(tensor), ElementType::U8);
        assert_eq!(rune_tensor_buffer_len(tensor), 4);

        let buffer = rune_tensor_buffer(tensor);
        assert!(!buffer.is_null());
        let buffer = slice::from_raw_parts_mut(
            buffer,
            rune_tensor_buffer_len(tensor) as usize,
        );
        buffer.fill(42);

        rune_input_tensors_free(
            tensors.map(|p| p.as_ptr()).unwrap_or(ptr::null_mut()),
        );
        rune_runtime_free(runtime);
    }
}

#[test]
fn inspect_output_metadata() {
    unsafe {
        let mut runtime: *mut Runtime = ptr::null_mut();
        let cfg = Config {
            wasm: SINE_RUNE.as_ptr(),
            wasm_len: SINE_RUNE.len() as c_int,
            engine: Engine::Wasm3,
        };

        let error = rune_runtime_load(&cfg, &mut runtime);
        assert!(error.is_null());

        let mut outputs: *mut Metadata = ptr::null_mut();

        let error = rune_runtime_outputs(runtime, &mut outputs);
        assert!(error.is_null());
        let outputs = NonNull::new(outputs);

        let num_outputs = rune_metadata_node_count(outputs);
        assert_eq!(1, num_outputs);

        let node = rune_metadata_get_node(outputs, 0);
        assert!(node.is_some());

        assert_eq!(3, rune_node_id(node));
        let kind = rune_node_kind(node);
        assert_eq!(CStr::from_ptr(kind).to_string_lossy(), "SERIAL");

        assert_eq!(0, rune_node_argument_count(node));

        rune_metadata_free(outputs.unwrap().as_ptr());
        rune_runtime_free(runtime);
    }
}

#[test]
fn run_the_sine_rune() {
    unsafe {
        let mut runtime: *mut Runtime = ptr::null_mut();
        let cfg = Config {
            wasm: SINE_RUNE.as_ptr(),
            wasm_len: SINE_RUNE.len() as c_int,
            engine: Engine::Wasm3,
        };

        let _ = rune_runtime_load(&cfg, &mut runtime);

        let mut tensors = ptr::null_mut();
        let _ = rune_runtime_input_tensors(runtime, &mut tensors);

        let dims = [1, 4];
        let tensor = rune_input_tensors_insert(
            NonNull::new(tensors),
            1,
            ElementType::U8,
            dims.as_ptr(),
            2,
        );
        let data = [1_u8, 2, 3, 4];
        std::ptr::copy_nonoverlapping(
            data.as_ptr(),
            rune_tensor_buffer(NonNull::new(tensor)),
            data.len(),
        );
        rune_input_tensors_free(tensors);

        let error = rune_runtime_predict(runtime);
        assert!(error.is_null());

        rune_error_free(error);
        rune_runtime_free(runtime);
    }
}
