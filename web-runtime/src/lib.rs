extern crate alloc;

pub(crate) mod external_types;
mod hacks;
mod imports;
mod runtime;

pub use crate::{imports::Imports, runtime::Runtime};

use anyhow::Error;
use wasm_bindgen::{JsCast, prelude::*};

#[global_allocator]
pub static ALLOCATOR: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[wasm_bindgen(start)]
pub fn on_load() { console_error_panic_hook::set_once(); }

pub(crate) fn rust_error(js: JsValue) -> Error {
    if let Some(e) = js.dyn_ref::<js_sys::Error>() {
        Error::msg(String::from(e.message()))
    } else if let Some(s) = js.dyn_ref::<js_sys::JsString>() {
        Error::msg(String::from(s))
    } else {
        Error::msg(format!("{:?}", js))
    }
}
