mod imports;
pub(crate) mod types;

pub use imports::Imports;

use wasm_bindgen::prelude::*;

#[global_allocator]
pub static ALLOCATOR: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[wasm_bindgen(start)]
pub fn on_load() { console_error_panic_hook::set_once(); }
