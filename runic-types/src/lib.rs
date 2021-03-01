#![no_std]
#[macro_use]
extern crate alloc;

pub mod marshall;
mod pipelines;
pub mod proc_block;

pub use pipelines::{PipelineContext, Sink, Source, Transform};

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
pub enum PARAMTYPE {
    INT = 1,
    FLOAT = 2,
    UTF8 = 3,
    BINARY = 4,
}

impl PARAMTYPE {
    pub fn from_u32(value: u32) -> PARAMTYPE {
        match value {
            1 => PARAMTYPE::INT,
            2 => PARAMTYPE::FLOAT,
            3 => PARAMTYPE::UTF8,
            4 => PARAMTYPE::BINARY,
            _ => PARAMTYPE::BINARY,
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
