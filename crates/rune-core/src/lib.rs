#![cfg_attr(not(feature = "std"), no_std)]

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
    tensor::{Tensor, TensorView, TensorViewMut},
    value::{Value, Type, AsType, InvalidConversionError},
    pixel_format::{PixelFormat, PixelFormatConversionError},
    logging::SerializableRecord,
    shape::Shape,
    tensor_list::{TensorList, TensorListMut},
    resources::{InlineResource, decode_inline_resource},
    element_type::{AsElementType, ElementType, UnknownElementType},
};

/// The mimetype used for a TensorFlow Lite model.
pub const TFLITE_MIMETYPE: &str = "application/tflite-model";

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
    }
}

constants! {
    outputs {
        /// A serial device which consumes JSON-encoded data.
        SERIAL = 1,
        BLE = 2,
        PIN = 3,
        WIFI = 4,
    }
}
