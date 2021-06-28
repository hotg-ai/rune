#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;

mod buf_writer;
mod logging;
mod pipelines;
mod pixel_format;
pub mod reflect;
mod tensor;
mod value;

pub use crate::{
    buf_writer::BufWriter,
    pipelines::{Sink, Source, HasOutputs},
    tensor::{Tensor, TensorView, TensorViewMut},
    value::{Value, Type, AsType, InvalidConversionError},
    pixel_format::{PixelFormat, PixelFormatConversionError},
    logging::SerializableRecord,
};

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
