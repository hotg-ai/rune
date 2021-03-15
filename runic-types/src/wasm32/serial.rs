use crate::{wasm32::intrinsics, Sink, OUTPUT, outputs};
use core::fmt::Debug;

#[derive(Debug, Default, PartialEq, Clone)]
#[non_exhaustive]
pub struct Serial {
    id: u32,
}

impl Serial {
    pub fn new() -> Self {
        unsafe {
            Serial {
                id: intrinsics::request_manifest_output(outputs::SERIAL),
            }
        }
    }
}

impl<T: Serialize> Sink<T> for Serial {
    fn consume(&mut self, input: T) {
        let msg = serde_json::to_string(&input);

        unsafe {

        }
    }
}
