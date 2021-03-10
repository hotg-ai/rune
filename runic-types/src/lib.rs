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

#[derive(Copy, Clone, Debug)]
pub enum CAPABILITY {
    RAND = 1,
    SOUND = 2,
    ACCEL = 3,
    IMAGE = 4,
    RAW = 5,
}

impl CAPABILITY {
    pub fn from_u32(value: u32) -> CAPABILITY {
        match value {
            1 => CAPABILITY::RAND,
            2 => CAPABILITY::SOUND,
            3 => CAPABILITY::ACCEL,
            4 => CAPABILITY::IMAGE,
            5 => CAPABILITY::RAW,
            _ => CAPABILITY::RAW,
        }
    }

    pub fn from_str(value: &str) -> Option<CAPABILITY> {
        match value {
            "RAND" => Some(CAPABILITY::RAND),
            "SOUND" => Some(CAPABILITY::SOUND),
            "ACCEL" => Some(CAPABILITY::ACCEL),
            "IMAGE" => Some(CAPABILITY::IMAGE),
            "RAW" => Some(CAPABILITY::RAW),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum PARAM_TYPE {
    INT = 1,
    FLOAT = 2,
    UTF8 = 3,
    BINARY = 4,
}

impl PARAM_TYPE {
    pub fn from_u32(value: u32) -> PARAM_TYPE {
        match value {
            1 => PARAM_TYPE::INT,
            2 => PARAM_TYPE::FLOAT,
            3 => PARAM_TYPE::UTF8,
            4 => PARAM_TYPE::BINARY,
            _ => PARAM_TYPE::BINARY,
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

/// A helper trait that lets us go from a type to its [`PARAM_TYPE`] equivalent.
pub trait AsParamType: Sized {
    /// The corresponding [`PARAM_TYPE`] variant.
    const VALUE: PARAM_TYPE;
}

impl AsParamType for i32 {
    const VALUE: PARAM_TYPE = PARAM_TYPE::INT;
}

impl AsParamType for f32 {
    const VALUE: PARAM_TYPE = PARAM_TYPE::FLOAT;
}

impl AsParamType for i16 {
    const VALUE: PARAM_TYPE = PARAM_TYPE::BINARY;
}

impl AsParamType for u8 {
    const VALUE: PARAM_TYPE = PARAM_TYPE::BINARY;
}
