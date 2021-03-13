#![no_std]
// The WebAssembly bindings need to provide alloc error handling.
#![cfg_attr(
    target_arch = "wasm32",
    feature(core_intrinsics, lang_items, alloc_error_handler)
)]

#[cfg(target_arch = "wasm32")]
extern crate alloc;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;

mod buffer;
mod pipelines;
mod value;

pub use pipelines::{Sink, Source, Transform};
pub use buffer::Buffer;
pub use value::{Value, Type, AsType};

pub mod capabilities {
    pub const RAND: u32 = 1;
    pub const SOUND: u32 = 2;
    pub const ACCEL: u32 = 3;
    pub const IMAGE: u32 = 4;
    pub const RAW: u32 = 5;

    pub fn from_str(value: &str) -> Option<u32> {
        match value {
            "RAND" => Some(RAND),
            "SOUND" => Some(SOUND),
            "ACCEL" => Some(ACCEL),
            "IMAGE" => Some(IMAGE),
            "RAW" => Some(RAW),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum OUTPUT {
    SERIAL = 1,
    BLE = 2,
    PIN = 3,
    WIFI = 4,
}

impl OUTPUT {
    pub fn from_u32(value: u32) -> OUTPUT {
        match value {
            1 => OUTPUT::SERIAL,
            2 => OUTPUT::BLE,
            3 => OUTPUT::PIN,
            4 => OUTPUT::WIFI,
            _ => OUTPUT::SERIAL,
        }
    }
}
