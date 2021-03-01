use crate::{wasm32::intrinsics, Sink, OUTPUT};
use core::fmt::Debug;

#[derive(Debug, Default, PartialEq, Clone)]
#[non_exhaustive]
pub struct Serial {}

impl Serial {
    pub fn new() -> Self {
        unsafe {
            intrinsics::request_manifest_output(OUTPUT::SERIAL as u32);
        }

        Serial {}
    }
}

impl<T: Debug> Sink<T> for Serial {
    fn consume(&mut self, input: T) {
        crate::debug!("Serial -> {:?}", input);
    }
}
