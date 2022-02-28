//! Core types and abstractions used by the Rune ecosystem.
//!
//! # Feature Flags
//!
//! This crate has the following cargo feature flags:
//!
//! - `std` - enables functionality that requires the standard library
//!   (typically implementations of `std::error::Error`)

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "unstable_doc_cfg", feature(doc_cfg))]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;

mod element_type;
mod logging;
mod pixel_format;
mod resources;
mod shape;
mod tensor;
mod tensor_list;
mod value;

pub use crate::{
    element_type::{AsElementType, ElementType, UnknownElementType},
    logging::SerializableRecord,
    pixel_format::{PixelFormat, PixelFormatConversionError},
    resources::{decode_inline_resource, InlineResource},
    shape::Shape,
    tensor::{Tensor, TensorView, TensorViewMut},
    tensor_list::{TensorList, TensorListMut},
    value::{AsType, InvalidConversionError, Type, Value},
};

/// The mimetype used for a TensorFlow Lite model.
pub const TFLITE_MIMETYPE: &str = "application/tflite-model";
/// The mimetype used for a TensorFlow model.
pub const TF_MIMETYPE: &str = "application/tf-model";
/// The mimetype used for a ONNX model.
pub const ONNX_MIMETYPE: &str = "application/onnx-model";
/// The mimetype used for a TensorFlow JS model.
pub const TFJS_MIMETYPE: &str = "application/tfjs-model";

/// The version number for this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! constants {
    ($name:ident { $(
        $(#[$constant_meta:meta])*
        $constant:ident = $value:expr
    ),* $(,)* }) => {
        pub mod $name {
            $(
                $( #[$constant_meta] )*
                pub const $constant: u32 = $value;
            )*

            pub const fn all() -> &'static [(&'static str, u32)] {
                &[
                    $(
                        (stringify!($constant), $value)
                    ),*
                ]
            }


            pub fn from_name(name: &str) -> Option<u32> {
                for (candidate, id) in all() {
                    if *candidate == name {
                        return Some(*id);
                    }
                }

                None
            }

            pub fn name(value: u32) -> Option<&'static str> {
                for (name, candidate) in all().iter() {
                    if *candidate == value {
                        return Some(*name);
                    }
                }

                None
            }
        }
    };
}

constants! {
    capabilities {
        RAND = 1,
        SOUND = 2,
        ACCEL = 3,
        IMAGE = 4,
        RAW = 5,
        FLOAT_IMAGE = 6,
    }
}

constants! {
    outputs {
        /// A serial device which consumes JSON-encoded data.
        SERIAL = 1,
        BLE = 2,
        PIN = 3,
        WIFI = 4,
        /// A raw tensor output.
        ///
        /// The buffer passed from the Rune to the runtime will be laid out
        /// as:
        ///
        /// | Field     | Length   | Description                                                                   |
        /// | --------- | -------- | ----------------------------------------------------------------------------- |
        /// | shape_len | 4        | A little-endian u32 containing the shape field's length                       |
        /// | shape     | variable | A UTF-8 string encoding the tensor's shape (i.e. element type and dimensions) |
        /// | elements  | variable | The tensor data itself, in little-endian format                               |
        ///
        /// This pattern may be repeated an arbitrary number of times, depending
        /// on how many tensors are being outputted.
        TENSOR = 5,
    }
}
