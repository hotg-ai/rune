#![no_std]
// The WebAssembly bindings need to provide alloc error handling.
#![cfg_attr(
    target_arch = "wasm32",
    feature(core_intrinsics, lang_items, alloc_error_handler)
)]

extern crate alloc;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;

mod buffer;
mod pipelines;
mod value;

use alloc::borrow::Cow;
use log::{Level, Record};
pub use pipelines::{Sink, Source, Transform};
pub use buffer::Buffer;
pub use value::{Value, Type, AsType, InvalidConversionError};

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

pub mod outputs {
    /// A serial device which consumes JSON-encoded data.
    pub const SERIAL: u32 = 1;
    pub const BLE: u32 = 2;
    pub const PIN: u32 = 3;
    pub const WIFI: u32 = 4;
}

/// A serializable version of [`log::Record`].
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SerializableRecord<'a> {
    pub level: Level,
    pub message: Cow<'a, str>,
    pub target: Cow<'a, str>,
    pub module_path: Option<Cow<'a, str>>,
    pub file: Option<Cow<'a, str>>,
    pub line: Option<u32>,
}

impl<'a> SerializableRecord<'a> {
    pub fn with_record<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Record<'_>) -> R,
    {
        f(&Record::builder()
            .level(self.level)
            .args(format_args!("{}", self.message.as_ref()))
            .target(self.target.as_ref())
            .module_path(self.module_path.as_deref())
            .file(self.file.as_deref())
            .line(self.line)
            .build())
    }
}
