use wasm_bindgen::prelude::*;

#[global_allocator]
pub static ALLOCATOR: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[wasm_bindgen(start)]
pub fn on_load() { console_error_panic_hook::set_once(); }

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() { alert("Hello, web-runtime!"); }
