#![no_std]
#![feature(alloc_error_handler)]
#![allow(warnings)]

use runic_types::{
    debug,
    wasm32::{intrinsics, Model},
};

#[no_mangle]
pub extern "C" fn _manifest() -> u32 {
    unsafe {
        let blob = include_bytes!("sine.tflite");
        let sine_model: Model<[f32; 1], [f32; 1]> = Model::load(blob);

        let ix = intrinsics::request_capability(
            runic_types::CAPABILITY::RAND as u32,
        );

        let key = "n";
        let value = u32::to_be_bytes(1);
        intrinsics::request_capability_set_param(
            ix,
            key.as_ptr(),
            key.len() as u32,
            value.as_ptr(),
            value.len() as u32,
            runic_types::PARAM_TYPE::INT as u32,
        );
        intrinsics::request_manifest_output(runic_types::OUTPUT::SERIAL as u32);
    }

    1
}

#[no_mangle]
pub extern "C" fn _call(
    capability_type: i32,
    input_type: i32,
    capability_idx: i32,
) -> i32 {
    static mut BUFFER: [u8; 512] = [0; 512];

    unsafe {
        let response_size = intrinsics::request_provider_response(
            BUFFER.as_ptr(),
            BUFFER.len() as u32,
            capability_idx as u32,
        );

        if response_size > 0 {
            // debug(b"Have a response\r\n");
            let response_size = response_size as usize;
            let buf: &[u8] = &BUFFER[..response_size];
            let proc_block_output = buf;

            if input_type == runic_types::PARAM_TYPE::FLOAT as i32 {
                if capability_type == runic_types::CAPABILITY::RAND as i32 {
                    intrinsics::tfm_model_invoke(
                        proc_block_output.as_ptr() as *const u8,
                        proc_block_output.len() as u32,
                    );
                    return proc_block_output.len() as i32;
                }
            }
        }

        response_size as i32
    }
}
