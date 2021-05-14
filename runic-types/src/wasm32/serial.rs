use crate::{wasm32::intrinsics, Sink, outputs, Tensor};
use serde::Serialize;
use core::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct Serial {
    id: u32,
}

impl Serial {
    // FIXME(Michael-F-Bryan): drop this back down to 8192 or figure out a way
    // to use a pre-allocated buffer.
    pub const MAX_MESSAGE_SIZE: usize = 8192 * 16;

    pub fn new() -> Self {
        unsafe {
            Serial {
                id: intrinsics::request_output(outputs::SERIAL),
            }
        }
    }

    fn consume_serializable<M>(&self, msg: &M)
    where
        M: Serialize,
    {
        let mut buffer = [0; Self::MAX_MESSAGE_SIZE];
        let bytes_written = serde_json_core::to_slice(&msg, &mut buffer)
            .expect("Unable to serialize the data as JSON");
        let msg = &buffer[..bytes_written];

        unsafe {
            intrinsics::consume_output(self.id, msg.as_ptr(), msg.len() as u32);
        }
    }
}

impl Default for Serial {
    fn default() -> Self { Serial::new() }
}

impl<T: Serialize> Sink<Tensor<T>> for Serial {
    fn consume(&mut self, input: Tensor<T>) {
        let msg = TensorMessage {
            type_name: core::any::type_name::<T>(),
            channel: self.id,
            elements: input.elements(),
            dimensions: input.dimensions(),
        };
        self.consume_serializable(&msg);
    }
}

impl<'a> Sink<&'a str> for Serial {
    fn consume(&mut self, input: &'a str) {
        let msg = StringMessage {
            type_name: "&str",
            channel: self.id,
            string: input,
        };
        self.consume_serializable(&msg);
    }
}

#[derive(serde::Serialize)]
struct TensorMessage<'a, T> {
    type_name: &'static str,
    channel: u32,
    elements: &'a [T],
    dimensions: &'a [usize],
}

#[derive(serde::Serialize)]
struct StringMessage<'a> {
    type_name: &'static str,
    channel: u32,
    string: &'a str,
}
