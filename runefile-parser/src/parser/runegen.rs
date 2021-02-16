use codegen::Scope;
use std::collections::HashMap;

pub enum CodeChunk {
    Attributes,
    Header,
    PanicHandler,
    AllocErrorHandler,
    Debug,
    ManifestFn,
    ProviderResponsePtr,
    TfmModelInvoke,
    Call,
}

pub fn generate_code(code: CodeChunk, params: Option<HashMap<String, String>>) -> String {
    let mut scope = Scope::new();
    let parameters = params.unwrap_or_else(|| HashMap::new());
    match code {
        CodeChunk::Call => {
            scope
                .new_fn("_call")
                .attr("no_mangle")
                .attr("warn(unused_must_use)")
                .vis("pub extern \"C\"")
                .arg("capability_type", "i32")
                .arg("input_type", "i32")
                .arg("capability_idx", "i32")
                .ret("i32")
                // sets memory buffer
                .line("unsafe {")
                .line("    let response_size = request_provider_response(")
                .line("        PROVIDER_RESPONSE_BUFFER.as_ptr(),")
                .line("        PROVIDER_RESPONSE_BUFFER_SIZE as u32,")
                .line("        capability_idx as u32")
                .line("    );")
                .line("")
                .line("")
                .line("    if response_size > 0 {")
                .line("")
                .line("")
                .line("        return response_size as i32;")
                .line("    }")
                .line("")
                .line("    return 0 as i32;")
                .line("")
                .line("}");
            scope
                .raw("")
                .raw("");
        }
        CodeChunk::TfmModelInvoke => {
            scope.raw("extern \"C\" {")
            .raw("    fn tfm_model_invoke(\n        feature_idx: *const u8,\n        feature_len: u32,\n    ) -> u32;")
            .raw("    fn tfm_preload_model(\n        model_idx: *const u8,\n        model_len: u32,\n        inputs: u32,\n        outputs: u32\n    ) -> u32;")
            .raw("    fn _debug(str_ptr: *const u8) -> u32;")
            .raw("    fn request_capability(ct: u32) -> u32;")
            .raw("    fn request_capability_set_param(idx:u32, key_str_ptr:*const u8, key_str_len:u32, value_ptr:*const u8, value_len:u32, value_type:u32) -> u32;")
            .raw("    fn request_manifest_output(t:u32) -> u32;")
            .raw("    fn request_provider_response(\n        provider_response_idx: *const u8,\n        max_allowed_provider_response: u32,\n        capability_idx: u32\n    ) -> u32;")
            .raw("}")
            .raw("")
            .raw("");
        }
        CodeChunk::ProviderResponsePtr => {
            scope
            .raw("const PROVIDER_RESPONSE_BUFFER_SIZE: usize = 512;")
            .raw("static mut PROVIDER_RESPONSE_BUFFER: [u8; PROVIDER_RESPONSE_BUFFER_SIZE] = [0; PROVIDER_RESPONSE_BUFFER_SIZE];")
            .raw("static mut PRINT_BUF: [u8;512] = [0 as u8; 512];")
            .raw("mod sine_model;")
            .raw("");
        }
        CodeChunk::ManifestFn => {
            let function = scope
                .new_fn("_manifest")
                .attr("no_mangle")
                .vis("pub extern \"C\"")
                .ret("u32")
                .line("unsafe {")
                .line("")
                .line("    debug(b\"Requesting Rand Capability\r\n\");")
                .line("")
                .line("    let rand_capability_idx = request_capability(CAPABILITY::RAND as u32);")
                .line("")
                .line("");
            function.line("")
                .line("    let key = b\"n\";")
                .line("    let value: &[u8; 4] = &u32::to_be_bytes(1u32);")
                .line("    request_capability_set_param(")
                .line("        rand_capability_idx,")
                .line("        key.as_ptr(),")
                .line("        key.len() as u32,")
                .line("        value.as_ptr(),")
                .line("        value.len() as u32,")
                .line("        PARAM_TYPE::INT as u32,")
                .line("    request_manifest_output(OUTPUT::SERIAL as u32);")
                .line("")
                .line("}")
                .line("return 1;");
            scope
                .raw("")
                .raw("");
        }
        CodeChunk::Attributes => {
            scope.raw(
                concat!("#![no_std]\n",
                        "#![feature(alloc, core_intrinsics, lang_items, alloc_error_handler)]\n",
                        "extern crate alloc;\n",
                        "extern crate wee_alloc;\n"))
            .raw("");
        }
        CodeChunk::Header => {
            scope.raw(
                concat!("// Use `wee_alloc` as the global allocator.\n",
                        "#[global_allocator]\n",
                        "static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;\n",
                        "use core::fmt::Write;\n",
                        "use core::panic::PanicInfo;\n",
                        "use core::alloc::Layout;\n",
                        "mod wrapper;\n",
                        "use runic_types::{CAPABILITY, PARAM_TYPE, OUTPUT};\n",
                        "use runic_transform::{Transform, Transformable};\n",
                        "use crate::wrapper::Wrapper;\n",
                        "use alloc::vec::*;"))
                .raw("")
                .raw("");
        }
        CodeChunk::PanicHandler => {
            scope
                .new_fn("panic")
                .attr("panic_handler")
                .arg("info", "&PanicInfo")
                .ret("!")
                .line("unsafe {")
                .line("    let s = match info.payload().downcast_ref::<&str>() {")
                .line("        Some(s) => s,")
                .line("        _ => \"\"")
                .line("    };")
                .line("")
                .line("    write!(Wrapper::new(&mut PRINT_BUF), \"Panic {}\\r\\n\", s).expect(\"Can't write\");")
                .line("    debug(&PRINT_BUF);")
                .line("    }")
                .line("loop {}");
            scope
                .raw("")
                .raw("");
        }
        CodeChunk::AllocErrorHandler => {
            scope
                .new_fn("alloc_error_handler")
                .attr("alloc_error_handler")
                .arg("info", "Layout")
                .ret("!")
                .line("unsafe {")
                .line("    write!(Wrapper::new(&mut PRINT_BUF), \"{:?} \\r\\n\", info).expect(\"Can't write\");")
                .line("    debug(&PRINT_BUF);")
                .line("}")
                .line("loop {}");
            scope
                .raw("")
                .raw("");
        }
        CodeChunk::Debug => {
            scope.raw(
                concat!("fn debug(s: &[u8]) -> u32 {\n",
                "    unsafe { return _debug(s.as_ptr()) }\n}"))
            .raw("//Should be created during runefile-parser")
            .raw("");
        }
    }
    return scope.to_string();
}

//TODO: In Runefile add 2 capabilities and should see their variables:
/*
    ```
        CAPABILITY<_,I32> RAND randasas --n 1 
    ```

     Step 1 Check that `RAND` is part of runic_types::CAPABILITY ENUM
     if None from runic_types::CAPABILITY::from_str("asdasd") => log::fatal!("INVALID....")
     Step 2 Check that `randasas` is a variable name that is not already used in capabilities before
     Step 3 For each variable `--n 1` Write out the 

        /// SET RAND CAPABILITY
        debug(b"Requesting Rand Capability\r\n");

        let rand_capability_idx = request_capability(CAPABILITY::RAND as u32);
        /// SET RAND CAPABILITY PARAMS
        /// FOR LATER when we know all the parameters for the variables then we will enforce type for the value. 
        /// For now just assume u32
        let key = b"n"; <-- should remove the `--` from `--n` 
        let value: &[u8; 4] = &u32::to_be_bytes(1u32); <-- can change from --n 1 to --n 111

        request_capability_set_param(
            rand_capability_idx,
            key.as_ptr(),
            key.len() as u32,
            value.as_ptr(),
            value.len() as u32,
            PARAM_TYPE::INT as u32, <-- this part needs to change in the future 
        );
*/
pub fn generate_manifest_function(
    capability_manifest: HashMap<String, String>,
    models_manifest: HashMap<String, String>,
    outtype_manifest: String,
) -> String {
    let mut manifest_scope = Scope::new();

    let manifest = manifest_scope
        .new_fn("gen_manifest")
        .attr("no_mangle")
        .ret("Manifest")
        .line("let mut params = HashMap::new();");
    let mut capability_concat_string = String::from("");
    for cap in capability_manifest.keys() {
        match capability_manifest.get(cap) {
            Some(value) => {
                manifest.line(&format!("{}", value));
                if capability_concat_string.len() > 0 {
                    let new_cap_string = &format!(
                        "{}, {}_capability_request",
                        capability_concat_string,
                        cap.to_lowercase().to_string()
                    );
                    capability_concat_string = String::from(new_cap_string);
                } else {
                    let new_cap_string =
                        &format!("{}_capability_request", cap.to_lowercase().to_string());
                    capability_concat_string = String::from(new_cap_string);
                }
            }
            None => {}
        };
    }

    manifest.line(&format!(
        "\nlet capabilities = vec![{}];",
        capability_concat_string
    ));
    manifest.line("\nreturn Manifest {\n\tcapabilities,");
    manifest.line(&format!(
        "    out: OUTPUT::{},",
        outtype_manifest.to_uppercase()
    ));

    let mut models_concat_string = String::from("");
    for model in models_manifest.keys() {
        match models_manifest.get(model) {
            Some(value) => {
                if models_concat_string.len() > 0 {
                    let new_models_string =
                        &format!("{}, {}", capability_concat_string, value.to_string());
                    models_concat_string = String::from(new_models_string);
                } else {
                    let new_models_string = &format!("{}", value.to_string());
                    models_concat_string = String::from(new_models_string);
                }
            }
            None => {}
        };
    }
    manifest.line(&format!("\tmodels: vec![{}]\n}};", models_concat_string));
    manifest_scope.raw("");
    return manifest_scope.to_string();
}




pub fn wrapper() -> String {
    let mut scope = Scope::new();
    scope.raw("use alloc::fmt;");
    scope.new_struct("Wrapper")
        .vis("pub")
        .generic("'a")
        .field("buf", "&'a mut [u8]")
        .field("offset", "usize");
    scope.new_impl("Wrapper")
        .generic("'a")
        .target_generic("'a")
        .new_fn("new")
        .vis("pub")
        .arg("buf", "&'a mut [u8]")
        .ret("Self")
        .line("Wrapper {")
        .line("    buf: buf,")
        .line("    offset: 0,")
        .line("}");
    scope.new_impl("Wrapper")
        .impl_trait("fmt::Write")
        .generic("'a")
        .target_generic("'a")
        .new_fn("write_str")
        .arg_mut_self()
        .arg("s", "&str")
        .ret("fmt::Result")
        .line("    let bytes = s.as_bytes();")
        .line("")
        .line("    // Skip over already-copied data")
        .line("    let remainder = &mut self.buf[self.offset..];")
        .line("    // Check if there is space remaining (return error instead of panicking)")
        .line("    if remainder.len() < bytes.len() { return Err(core::fmt::Error); }")
        .line("    // Make the two slices the same length")
        .line("    let remainder = &mut remainder[..bytes.len()];")
        .line("    // Copy")
        .line("    remainder.copy_from_slice(bytes);")
        .line("")
        .line("    // Update offset to avoid overwriting")
        .line("    self.offset += bytes.len();")
        .line("")
        .line("    Ok(())");

    return scope.to_string();
}

//temp hack
pub fn sine_model() -> String {
    let mut scope = Scope::new();
    scope.raw("use alloc::vec::*;")
        .raw(
        concat!("pub const MODEL: &[u8; 2640] = &[\n",
                "                   0x18, 0x00, 0x00, 0x00, 0x54, 0x46, 0x4C, 0x33, 0x00, 0x00, 0x0E, 0x00, 0x18, 0x00, 0x04,\n",
                "                   0x00, 0x08, 0x00, 0x0C, 0x00, 0x10, 0x00, 0x14, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x03, 0x00,\n",
                "                   0x00, 0x00, 0x10, 0x0A, 0x00, 0x00, 0xB8, 0x05, 0x00, 0x00, 0xA0, 0x05, 0x00, 0x00, 0x04,\n",
                "                   0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x90, 0x05, 0x00, 0x00, 0x7C, 0x05, 0x00, 0x00,\n",
                "                   0x24, 0x05, 0x00, 0x00, 0xD4, 0x04, 0x00, 0x00, 0xC4, 0x00, 0x00, 0x00, 0x74, 0x00, 0x00,\n",
                "                   0x00, 0x24, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x0C, 0x00,\n",
                "                   0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x54, 0xF6, 0xFF, 0xFF, 0x58, 0xF6, 0xFF, 0xFF, 0x5C,\n",
                "                   0xF6, 0xFF, 0xFF, 0x60, 0xF6, 0xFF, 0xFF, 0xC2, 0xFA, 0xFF, 0xFF, 0x04, 0x00, 0x00, 0x00,\n",
                "                   0x40, 0x00, 0x00, 0x00, 0x7C, 0x19, 0xA7, 0x3E, 0x99, 0x81, 0xB9, 0x3E, 0x56, 0x8B, 0x9F,\n",
                "                   0x3E, 0x88, 0xD8, 0x12, 0xBF, 0x74, 0x10, 0x56, 0x3E, 0xFE, 0xC6, 0xDF, 0xBE, 0xF2, 0x10,\n",
                "                   0x5A, 0xBE, 0xF0, 0xE2, 0x0A, 0xBE, 0x10, 0x5A, 0x98, 0xBE, 0xB9, 0x36, 0xCE, 0x3D, 0x8F,\n",
                "                   0x7F, 0x87, 0x3E, 0x2C, 0xB1, 0xFD, 0xBD, 0xE6, 0xA6, 0x8A, 0xBE, 0xA5, 0x3E, 0xDA, 0x3E,\n",
                "                   0x50, 0x34, 0xED, 0xBD, 0x90, 0x91, 0x69, 0xBE, 0x0E, 0xFB, 0xFF, 0xFF, 0x04, 0x00, 0x00,\n",
                "                   0x00, 0x40, 0x00, 0x00, 0x00, 0x67, 0x41, 0x48, 0xBF, 0x24, 0xCD, 0xA0, 0xBE, 0xB7, 0x92,\n",
                "                   0x0C, 0xBF, 0x00, 0x00, 0x00, 0x00, 0x98, 0xFE, 0x3C, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00,\n",
                "                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4A, 0x17, 0x9A, 0xBE,\n",
                "                   0x41, 0xCB, 0xB6, 0xBE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0xD6, 0x1E,\n",
                "                   0x3E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5A, 0xFB, 0xFF, 0xFF, 0x04, 0x00,\n",
                "                   0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x4B, 0x98, 0xDD, 0xBD, 0x40, 0x6B, 0xCB, 0xBE, 0x36,\n",
                "                   0x0C, 0xD4, 0x3C, 0xBD, 0x44, 0xB5, 0x3E, 0x95, 0x70, 0xE3, 0x3E, 0xE7, 0xAC, 0x86, 0x3E,\n",
                "                   0x00, 0xC4, 0x4E, 0x3D, 0x7E, 0xA6, 0x1D, 0x3E, 0xBD, 0x87, 0xBB, 0x3E, 0xB4, 0xB8, 0x09,\n",
                "                   0xBF, 0xA1, 0x1F, 0xF8, 0xBE, 0x8D, 0x90, 0xDD, 0x3E, 0xDE, 0xFA, 0x6F, 0xBE, 0xB2, 0x75,\n",
                "                   0xE4, 0x3D, 0x6E, 0xFE, 0x36, 0x3E, 0x20, 0x18, 0xC2, 0xBE, 0x39, 0xC7, 0xFB, 0xBE, 0xFE,\n",
                "                   0xA4, 0x30, 0xBE, 0xF7, 0x91, 0xDE, 0xBE, 0xDE, 0xAB, 0x24, 0x3E, 0xFB, 0xBB, 0xCE, 0x3E,\n",
                "                   0xEB, 0x23, 0x80, 0xBE, 0x7B, 0x58, 0x73, 0xBE, 0x9A, 0x2E, 0x03, 0x3E, 0x10, 0x42, 0xA9,\n",
                "                   0xBC, 0x10, 0x12, 0x64, 0xBD, 0xE3, 0x8D, 0x0C, 0x3D, 0x9E, 0x48, 0x97, 0xBE, 0x34, 0x51,\n",
                "                   0xD4, 0xBE, 0x02, 0x3B, 0x0D, 0x3E, 0x62, 0x67, 0x89, 0xBE, 0x74, 0xDF, 0xA2, 0x3D, 0xF3,\n",
                "                   0x25, 0xB3, 0xBE, 0xEF, 0x34, 0x7B, 0x3D, 0x61, 0x70, 0xE3, 0x3D, 0xBA, 0x76, 0xC0, 0xBE,\n",
                "                   0x7D, 0xE9, 0xA7, 0x3E, 0xC3, 0xAB, 0xD0, 0xBE, 0xCF, 0x7C, 0xDB, 0xBE, 0x70, 0x27, 0x9A,\n",
                "                   0xBE, 0x98, 0xF5, 0x3C, 0xBD, 0xFF, 0x4B, 0x4B, 0x3E, 0x7E, 0xA0, 0xF8, 0xBD, 0xD4, 0x6E,\n",
                "                   0x86, 0x3D, 0x00, 0x4A, 0x07, 0x3A, 0x4C, 0x24, 0x61, 0xBE, 0x54, 0x68, 0xF7, 0xBD, 0x02,\n",
                "                   0x3F, 0x77, 0xBE, 0x23, 0x79, 0xB3, 0x3E, 0x1C, 0x83, 0xAD, 0xBD, 0xC8, 0x92, 0x8D, 0x3E,\n",
                "                   0xA8, 0xF3, 0x15, 0xBD, 0xE6, 0x4D, 0x6C, 0x3D, 0xAC, 0xE7, 0x98, 0xBE, 0x81, 0xEC, 0xBD,\n",
                "                   0x3E, 0xE2, 0x55, 0x73, 0x3E, 0xC1, 0x77, 0xC7, 0x3E, 0x6E, 0x1B, 0x5E, 0x3D, 0x27, 0x78,\n",
                "                   0x02, 0x3F, 0xD4, 0x21, 0x90, 0x3D, 0x52, 0xDC, 0x1F, 0x3E, 0xBF, 0xDA, 0x88, 0x3E, 0x80,\n",
                "                   0x79, 0xE3, 0xBD, 0x40, 0x6F, 0x10, 0xBE, 0x20, 0x43, 0x2E, 0xBD, 0xF0, 0x76, 0xC5, 0xBD,\n",
                "                   0xCC, 0xA0, 0x04, 0xBE, 0xF0, 0x69, 0xD7, 0xBE, 0xB1, 0xFE, 0x64, 0xBE, 0x20, 0x41, 0x84,\n",
                "                   0xBE, 0xB2, 0xC3, 0x26, 0xBE, 0xD8, 0xF4, 0x09, 0xBE, 0x64, 0x44, 0xD1, 0x3D, 0xD5, 0xE1,\n",
                "                   0xC8, 0xBE, 0x35, 0xBC, 0x3F, 0xBE, 0xC0, 0x94, 0x82, 0x3D, 0xDC, 0x2B, 0xB1, 0xBD, 0x02,\n",
                "                   0xDB, 0xBF, 0xBE, 0xA5, 0x7F, 0x8A, 0x3E, 0x21, 0xB4, 0xA2, 0x3E, 0xCD, 0x86, 0x56, 0xBF,\n",
                "                   0x9C, 0x3B, 0x76, 0xBC, 0x85, 0x6D, 0x60, 0xBF, 0x86, 0x00, 0x3C, 0xBE, 0xC1, 0x23, 0x7E,\n",
                "                   0x3E, 0x96, 0xCD, 0x3F, 0x3E, 0x86, 0x91, 0x2D, 0x3E, 0x55, 0xEF, 0x87, 0x3E, 0x7E, 0x97,\n",
                "                   0x03, 0xBE, 0x2A, 0xCD, 0x01, 0x3E, 0x32, 0xC9, 0x8E, 0xBE, 0x72, 0x77, 0x3B, 0xBE, 0xE0,\n",
                "                   0xA1, 0xBC, 0xBE, 0x8D, 0xB7, 0xA7, 0x3E, 0x1C, 0x05, 0x95, 0xBE, 0xF7, 0x1F, 0xBB, 0x3E,\n",
                "                   0xC9, 0x3E, 0xD6, 0x3E, 0x80, 0x42, 0xE9, 0xBD, 0x27, 0x0C, 0xD2, 0xBE, 0x5C, 0x32, 0x34,\n",
                "                   0xBE, 0x14, 0xCB, 0xCA, 0xBD, 0xDD, 0x3A, 0x67, 0xBE, 0x1C, 0xBB, 0x8D, 0xBE, 0x91, 0xAC,\n",
                "                   0x5C, 0xBE, 0x52, 0x40, 0x6F, 0xBE, 0xD7, 0x71, 0x94, 0x3E, 0x18, 0x71, 0x09, 0xBE, 0x9B,\n",
                "                   0x29, 0xD9, 0xBE, 0x7D, 0x66, 0xD2, 0xBE, 0x98, 0xD6, 0xB2, 0xBE, 0x00, 0xC9, 0x84, 0x3A,\n",
                "                   0xBC, 0xDA, 0xC2, 0xBD, 0x1D, 0xC2, 0x1B, 0xBF, 0xD4, 0xDD, 0x92, 0x3E, 0x07, 0x87, 0x6C,\n",
                "                   0xBE, 0x40, 0xC2, 0x3B, 0xBE, 0xBD, 0xE2, 0x9C, 0x3E, 0x0A, 0xB5, 0xA0, 0xBE, 0xE2, 0xD5,\n",
                "                   0x9C, 0xBE, 0x3E, 0xBB, 0x7C, 0x3E, 0x17, 0xB4, 0xCF, 0x3E, 0xD5, 0x8E, 0xC8, 0xBE, 0x7C,\n",
                "                   0xF9, 0x5C, 0x3E, 0x80, 0xFC, 0x0D, 0x3D, 0xC5, 0xD5, 0x8B, 0x3E, 0xF5, 0x17, 0xA2, 0x3E,\n",
                "                   0xC7, 0x60, 0x89, 0xBE, 0xEC, 0x95, 0x87, 0x3D, 0x7A, 0xC2, 0x5D, 0xBF, 0x77, 0x94, 0x98,\n",
                "                   0x3E, 0x77, 0x39, 0x07, 0xBC, 0x42, 0x29, 0x00, 0x3E, 0xAF, 0xD0, 0xA9, 0x3E, 0x31, 0x23,\n",
                "                   0xC4, 0xBE, 0x95, 0x36, 0x5B, 0xBE, 0xC7, 0xDC, 0x83, 0xBE, 0x1E, 0x6B, 0x47, 0x3E, 0x5B,\n",
                "                   0x24, 0x99, 0x3E, 0x99, 0x27, 0x54, 0x3E, 0xC8, 0x20, 0xDD, 0xBD, 0x5A, 0x86, 0x2F, 0x3E,\n",
                "                   0x80, 0xF0, 0x69, 0xBE, 0x44, 0xFC, 0x84, 0xBD, 0x82, 0xA0, 0x2A, 0xBE, 0x87, 0xE6, 0x2A,\n",
                "                   0x3E, 0xD8, 0x34, 0xAE, 0x3D, 0x50, 0xBD, 0xB5, 0x3E, 0xC4, 0x8C, 0x88, 0xBE, 0xE3, 0xBC,\n",
                "                   0xA5, 0x3E, 0xA9, 0xDA, 0x9E, 0x3E, 0x3E, 0xB8, 0x23, 0xBE, 0x80, 0x90, 0x15, 0x3D, 0x97,\n",
                "                   0x3F, 0xC3, 0x3E, 0xCA, 0x5C, 0x9D, 0x3E, 0x21, 0xE8, 0xE1, 0x3E, 0xC0, 0x49, 0x01, 0xBC,\n",
                "                   0x00, 0x0B, 0x88, 0xBD, 0x3F, 0xF7, 0xCA, 0x3C, 0xFB, 0x5A, 0xB1, 0x3E, 0x60, 0xD2, 0x0D,\n",
                "                   0x3C, 0xCE, 0x23, 0x78, 0xBF, 0x8F, 0x4F, 0xB9, 0xBE, 0x69, 0x6A, 0x34, 0xBF, 0x4B, 0x5E,\n",
                "                   0xA9, 0x3E, 0x64, 0x8C, 0xD9, 0x3E, 0x52, 0x77, 0x36, 0x3E, 0xEB, 0xAF, 0xBE, 0x3E, 0x40,\n",
                "                   0xBE, 0x36, 0x3C, 0x08, 0x65, 0x3B, 0xBD, 0x55, 0xE0, 0x66, 0xBD, 0xD2, 0xE8, 0x9B, 0xBE,\n",
                "                   0x86, 0xE3, 0x09, 0xBE, 0x93, 0x3D, 0xDD, 0x3E, 0x0F, 0x66, 0x18, 0x3F, 0x18, 0x05, 0x33,\n",
                "                   0xBD, 0xDE, 0x15, 0xD7, 0xBE, 0xAA, 0xCF, 0x49, 0xBE, 0xA2, 0xA5, 0x64, 0x3E, 0xE6, 0x9C,\n",
                "                   0x42, 0xBE, 0x54, 0x42, 0xCC, 0x3D, 0xA0, 0xBD, 0x9D, 0xBE, 0xC2, 0x69, 0x48, 0x3E, 0x5B,\n",
                "                   0x8B, 0xA2, 0xBE, 0xC0, 0x13, 0x87, 0x3D, 0x36, 0xFD, 0x69, 0x3E, 0x05, 0x86, 0x40, 0xBE,\n",
                "                   0x1E, 0x7A, 0xCE, 0xBE, 0x46, 0x13, 0xA7, 0xBE, 0x68, 0x52, 0x86, 0xBE, 0x04, 0x9E, 0x86,\n",
                "                   0xBD, 0x8C, 0x54, 0xC1, 0x3D, 0xE0, 0x3B, 0xAD, 0x3C, 0x42, 0x67, 0x85, 0xBD, 0xEA, 0x97,\n",
                "                   0x42, 0x3E, 0x6E, 0x13, 0x3B, 0xBF, 0x56, 0x5B, 0x16, 0x3E, 0xAA, 0xAB, 0xDF, 0x3E, 0xC8,\n",
                "                   0x41, 0x36, 0x3D, 0x24, 0x2D, 0x47, 0xBE, 0x77, 0xA5, 0xAE, 0x3E, 0xC0, 0xC2, 0x5B, 0x3C,\n",
                "                   0xAC, 0xAC, 0x4E, 0x3E, 0x99, 0xEC, 0x13, 0xBE, 0xF2, 0xAB, 0x73, 0x3E, 0xAA, 0xA1, 0x48,\n",
                "                   0xBE, 0xE8, 0xD3, 0x01, 0xBE, 0x60, 0xB7, 0xC7, 0xBD, 0x64, 0x72, 0xD3, 0x3D, 0x83, 0xD3,\n",
                "                   0x99, 0x3E, 0x0C, 0x76, 0x34, 0xBE, 0x42, 0xDA, 0x0D, 0x3E, 0xFB, 0x47, 0x9A, 0x3E, 0x8B,\n",
                "                   0xDC, 0x92, 0xBE, 0x56, 0x7F, 0x6B, 0x3E, 0x04, 0xD4, 0x88, 0xBD, 0x11, 0x9E, 0x80, 0x3E,\n",
                "                   0x3C, 0x89, 0xFF, 0x3D, 0xB3, 0x3E, 0x88, 0x3E, 0xF7, 0xF0, 0x88, 0x3E, 0x28, 0xFB, 0xC9,\n",
                "                   0xBE, 0x53, 0x3E, 0xCF, 0x3E, 0xAC, 0x75, 0xDC, 0xBE, 0xDD, 0xCA, 0xD7, 0x3E, 0x01, 0x58,\n",
                "                   0xA7, 0x3E, 0x29, 0xB8, 0x13, 0xBF, 0x76, 0x81, 0x12, 0xBC, 0x28, 0x8B, 0x16, 0xBF, 0x0E,\n",
                "                   0xEC, 0x0E, 0x3E, 0x40, 0x0A, 0xDB, 0xBD, 0x98, 0xEC, 0xBF, 0xBD, 0x32, 0x55, 0x0C, 0xBE,\n",
                "                   0xFB, 0xF9, 0xC9, 0x3E, 0x83, 0x4A, 0x6D, 0xBE, 0x76, 0x59, 0xE2, 0xBE, 0x54, 0x7D, 0x9F,\n",
                "                   0xBB, 0x9D, 0xE8, 0x95, 0x3E, 0x5C, 0xD3, 0xD0, 0x3D, 0x19, 0x8A, 0xB0, 0x3E, 0xDE, 0x6F,\n",
                "                   0x2E, 0xBE, 0xD0, 0x16, 0x83, 0x3D, 0x9C, 0x7D, 0x11, 0xBF, 0x2B, 0xCC, 0x25, 0x3C, 0x2A,\n",
                "                   0xA5, 0x27, 0xBE, 0x22, 0x14, 0xC7, 0xBE, 0x5E, 0x7A, 0xAC, 0x3E, 0x4E, 0x41, 0x94, 0xBE,\n",
                "                   0x5A, 0x68, 0x7B, 0x3E, 0x86, 0xFD, 0x4E, 0x3E, 0xA2, 0x56, 0x6A, 0xBE, 0xCA, 0xFE, 0x81,\n",
                "                   0xBE, 0x43, 0xC3, 0xB1, 0xBD, 0xC5, 0xB8, 0xA7, 0x3E, 0x55, 0x23, 0xCD, 0x3E, 0xAF, 0x2E,\n",
                "                   0x76, 0x3E, 0x69, 0xA8, 0x90, 0xBE, 0x0D, 0xBA, 0xB9, 0x3E, 0x66, 0xFF, 0xFF, 0xFF, 0x04,\n",
                "                   0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x53, 0xD6, 0xE2, 0x3D, 0x66, 0xB6, 0xCC, 0x3E,\n",
                "                   0x03, 0xE7, 0xF6, 0x3E, 0xE0, 0x28, 0x10, 0xBF, 0x00, 0x00, 0x00, 0x00, 0x3E, 0x3D, 0xB0,\n",
                "                   0x3E, 0x00, 0x00, 0x00, 0x00, 0x62, 0xF0, 0x77, 0x3E, 0xA6, 0x9D, 0xA4, 0x3E, 0x3A, 0x4B,\n",
                "                   0xF3, 0xBE, 0x71, 0x9E, 0xA7, 0x3E, 0x00, 0x00, 0x00, 0x00, 0x34, 0x39, 0xA2, 0x3E, 0x00,\n",
                "                   0x00, 0x00, 0x00, 0xCC, 0x9C, 0x4A, 0x3E, 0xAB, 0x40, 0xA3, 0x3E, 0xB2, 0xFF, 0xFF, 0xFF,\n",
                "                   0x04, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0xB3, 0x71, 0x67, 0x3F, 0x9A, 0x7A, 0x95,\n",
                "                   0xBF, 0xE1, 0x48, 0xE8, 0xBE, 0x8A, 0x72, 0x96, 0x3E, 0x00, 0xD2, 0xD3, 0xBB, 0x1A, 0xC5,\n",
                "                   0xD7, 0x3F, 0xAC, 0x7E, 0xC8, 0xBE, 0x90, 0xA7, 0x95, 0xBE, 0x3B, 0xD7, 0xDC, 0xBE, 0x41,\n",
                "                   0xA8, 0x16, 0x3F, 0x50, 0x5B, 0xCB, 0x3F, 0x52, 0xB9, 0xED, 0xBE, 0x2E, 0xA7, 0xC6, 0xBE,\n",
                "                   0xAF, 0x0F, 0x14, 0xBF, 0xB3, 0xDA, 0x59, 0x3F, 0x02, 0xEC, 0xD7, 0xBE, 0x00, 0x00, 0x06,\n",
                "                   0x00, 0x08, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00,\n",
                "                   0x00, 0x00, 0x66, 0x11, 0x1F, 0xBF, 0xB8, 0xFB, 0xFF, 0xFF, 0x0F, 0x00, 0x00, 0x00, 0x54,\n",
                "                   0x4F, 0x43, 0x4F, 0x20, 0x43, 0x6F, 0x6E, 0x76, 0x65, 0x72, 0x74, 0x65, 0x64, 0x2E, 0x00,\n",
                "                   0x01, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x14, 0x00, 0x04, 0x00, 0x08,\n",
                "                   0x00, 0x0C, 0x00, 0x10, 0x00, 0x0C, 0x00, 0x00, 0x00, 0xF0, 0x00, 0x00, 0x00, 0xE4, 0x00,\n",
                "                   0x00, 0x00, 0xD8, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x90,\n",
                "                   0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xCE, 0xFF, 0xFF, 0xFF,\n",
                "                   0x00, 0x00, 0x00, 0x08, 0x18, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,\n",
                "                   0x00, 0x1C, 0xFC, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00,\n",
                "                   0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00,\n",
                "                   0x00, 0x0E, 0x00, 0x14, 0x00, 0x00, 0x00, 0x08, 0x00, 0x0C, 0x00, 0x07, 0x00, 0x10, 0x00,\n",
                "                   0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x1C, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,\n",
                "                   0x00, 0x04, 0x00, 0x00, 0x00, 0xBA, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00,\n",
                "                   0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x05,\n",
                "                   0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x16, 0x00, 0x00, 0x00,\n",
                "                   0x08, 0x00, 0x0C, 0x00, 0x07, 0x00, 0x10, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,\n",
                "                   0x08, 0x24, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00,\n",
                "                   0x06, 0x00, 0x08, 0x00, 0x07, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01,\n",
                "                   0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,\n",
                "                   0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,\n",
                "                   0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x10, 0x03,\n",
                "                   0x00, 0x00, 0xA4, 0x02, 0x00, 0x00, 0x40, 0x02, 0x00, 0x00, 0xF4, 0x01, 0x00, 0x00, 0xAC,\n",
                "                   0x01, 0x00, 0x00, 0x48, 0x01, 0x00, 0x00, 0xFC, 0x00, 0x00, 0x00, 0xB4, 0x00, 0x00, 0x00,\n",
                "                   0x50, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x26, 0xFD, 0xFF, 0xFF, 0x3C, 0x00, 0x00,\n",
                "                   0x00, 0x01, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x18, 0xFD,\n",
                "                   0xFF, 0xFF, 0x20, 0x00, 0x00, 0x00, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6E, 0x74, 0x69, 0x61,\n",
                "                   0x6C, 0x5F, 0x31, 0x2F, 0x64, 0x65, 0x6E, 0x73, 0x65, 0x5F, 0x34, 0x2F, 0x4D, 0x61, 0x74,\n",
                "                   0x4D, 0x75, 0x6C, 0x5F, 0x62, 0x69, 0x61, 0x73, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,\n",
                "                   0x00, 0x01, 0x00, 0x00, 0x00, 0x6E, 0xFD, 0xFF, 0xFF, 0x50, 0x00, 0x00, 0x00, 0x02, 0x00,\n",
                "                   0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x60, 0xFD, 0xFF, 0xFF, 0x34,\n",
                "                   0x00, 0x00, 0x00, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6E, 0x74, 0x69, 0x61, 0x6C, 0x5F, 0x31,\n",
                "                   0x2F, 0x64, 0x65, 0x6E, 0x73, 0x65, 0x5F, 0x34, 0x2F, 0x4D, 0x61, 0x74, 0x4D, 0x75, 0x6C,\n",
                "                   0x2F, 0x52, 0x65, 0x61, 0x64, 0x56, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6C, 0x65, 0x4F, 0x70,\n",
                "                   0x2F, 0x74, 0x72, 0x61, 0x6E, 0x73, 0x70, 0x6F, 0x73, 0x65, 0x00, 0x00, 0x00, 0x00, 0x02,\n",
                "                   0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0xCE, 0xFD, 0xFF, 0xFF,\n",
                "                   0x34, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,\n",
                "                   0x00, 0xC0, 0xFD, 0xFF, 0xFF, 0x19, 0x00, 0x00, 0x00, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6E,\n",
                "                   0x74, 0x69, 0x61, 0x6C, 0x5F, 0x31, 0x2F, 0x64, 0x65, 0x6E, 0x73, 0x65, 0x5F, 0x33, 0x2F,\n",
                "                   0x52, 0x65, 0x6C, 0x75, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,\n",
                "                   0x10, 0x00, 0x00, 0x00, 0x12, 0xFE, 0xFF, 0xFF, 0x3C, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00,\n",
                "                   0x00, 0x0C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0xFE, 0xFF, 0xFF, 0x20, 0x00,\n",
                "                   0x00, 0x00, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6E, 0x74, 0x69, 0x61, 0x6C, 0x5F, 0x31, 0x2F,\n",
                "                   0x64, 0x65, 0x6E, 0x73, 0x65, 0x5F, 0x33, 0x2F, 0x4D, 0x61, 0x74, 0x4D, 0x75, 0x6C, 0x5F,\n",
                "                   0x62, 0x69, 0x61, 0x73, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,\n",
                "                   0x00, 0x5A, 0xFE, 0xFF, 0xFF, 0x50, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0C, 0x00,\n",
                "                   0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x4C, 0xFE, 0xFF, 0xFF, 0x34, 0x00, 0x00, 0x00, 0x73,\n",
                "                   0x65, 0x71, 0x75, 0x65, 0x6E, 0x74, 0x69, 0x61, 0x6C, 0x5F, 0x31, 0x2F, 0x64, 0x65, 0x6E,\n",
                "                   0x73, 0x65, 0x5F, 0x33, 0x2F, 0x4D, 0x61, 0x74, 0x4D, 0x75, 0x6C, 0x2F, 0x52, 0x65, 0x61,\n",
                "                   0x64, 0x56, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6C, 0x65, 0x4F, 0x70, 0x2F, 0x74, 0x72, 0x61,\n",
                "                   0x6E, 0x73, 0x70, 0x6F, 0x73, 0x65, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x10,\n",
                "                   0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0xBA, 0xFE, 0xFF, 0xFF, 0x34, 0x00, 0x00, 0x00,\n",
                "                   0x0A, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xAC, 0xFE, 0xFF,\n",
                "                   0xFF, 0x19, 0x00, 0x00, 0x00, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6E, 0x74, 0x69, 0x61, 0x6C,\n",
                "                   0x5F, 0x31, 0x2F, 0x64, 0x65, 0x6E, 0x73, 0x65, 0x5F, 0x32, 0x2F, 0x52, 0x65, 0x6C, 0x75,\n",
                "                   0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,\n",
                "                   0xFE, 0xFE, 0xFF, 0xFF, 0x3C, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00,\n",
                "                   0x00, 0x04, 0x00, 0x00, 0x00, 0xF0, 0xFE, 0xFF, 0xFF, 0x20, 0x00, 0x00, 0x00, 0x73, 0x65,\n",
                "                   0x71, 0x75, 0x65, 0x6E, 0x74, 0x69, 0x61, 0x6C, 0x5F, 0x31, 0x2F, 0x64, 0x65, 0x6E, 0x73,\n",
                "                   0x65, 0x5F, 0x32, 0x2F, 0x4D, 0x61, 0x74, 0x4D, 0x75, 0x6C, 0x5F, 0x62, 0x69, 0x61, 0x73,\n",
                "                   0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x46, 0xFF, 0xFF,\n",
                "                   0xFF, 0x50, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x04, 0x00,\n",
                "                   0x00, 0x00, 0x38, 0xFF, 0xFF, 0xFF, 0x34, 0x00, 0x00, 0x00, 0x73, 0x65, 0x71, 0x75, 0x65,\n",
                "                   0x6E, 0x74, 0x69, 0x61, 0x6C, 0x5F, 0x31, 0x2F, 0x64, 0x65, 0x6E, 0x73, 0x65, 0x5F, 0x32,\n",
                "                   0x2F, 0x4D, 0x61, 0x74, 0x4D, 0x75, 0x6C, 0x2F, 0x52, 0x65, 0x61, 0x64, 0x56, 0x61, 0x72,\n",
                "                   0x69, 0x61, 0x62, 0x6C, 0x65, 0x4F, 0x70, 0x2F, 0x74, 0x72, 0x61, 0x6E, 0x73, 0x70, 0x6F,\n",
                "                   0x73, 0x65, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01,\n",
                "                   0x00, 0x00, 0x00, 0xA6, 0xFF, 0xFF, 0xFF, 0x48, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00,\n",
                "                   0x2C, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x08, 0x00, 0x0C, 0x00, 0x04, 0x00, 0x08,\n",
                "                   0x00, 0x08, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00,\n",
                "                   0x00, 0x00, 0x00, 0x00, 0x7F, 0x43, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0D,\n",
                "                   0x00, 0x00, 0x00, 0x64, 0x65, 0x6E, 0x73, 0x65, 0x5F, 0x32, 0x5F, 0x69, 0x6E, 0x70, 0x75,\n",
                "                   0x74, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,\n",
                "                   0x00, 0x00, 0x00, 0x0E, 0x00, 0x14, 0x00, 0x04, 0x00, 0x00, 0x00, 0x08, 0x00, 0x0C, 0x00,\n",
                "                   0x10, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x10,\n",
                "                   0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x00, 0x00,\n",
                "                   0x08, 0x00, 0x00, 0x00, 0x49, 0x64, 0x65, 0x6E, 0x74, 0x69, 0x74, 0x79, 0x00, 0x00, 0x00,\n",
                "                   0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,\n",
                "                   0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x0C, 0x00, 0x07, 0x00, 0x00,\n",
                "                   0x00, 0x08, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x03, 0x00, 0x00, 0x00,\n",
                "];"));
    return scope.to_string();
}
