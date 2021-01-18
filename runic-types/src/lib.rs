
#![no_std]
#[macro_use]
extern crate alloc;
pub mod marshall;
pub mod proc_block;


pub enum CAPABILITY
{
    RAND = 1,
    SOUND = 2,
    ACCEL = 3,
    IMAGE = 4,
    RAW = 5
}

pub enum PARAM_TYPE {
    INT = 1,
    FLOAT = 2,
    UTF8  = 3,
    BINARY = 4,
}

pub enum OUTPUT {
    SERIAL = 1,
    BLE = 2,
    PIN = 3,
    WIFI = 4
}


