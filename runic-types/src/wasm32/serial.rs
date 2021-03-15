use crate::{wasm32::intrinsics, Sink, outputs};
use serde::Serialize;
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
                id: intrinsics::request_output(outputs::SERIAL),
            }
        }
    }
}

impl<T: Serialize> Sink<T> for Serial {
    fn consume(&mut self, input: T) {
        let msg = serde_json::to_string(&input)
            .expect("Unable to serialize the data as JSON");

        unsafe {
            intrinsics::consume_output(self.id, msg.as_ptr(), msg.len() as u32);
        }
    }
}
