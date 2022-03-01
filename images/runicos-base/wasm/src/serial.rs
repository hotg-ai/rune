use alloc::vec::Vec;
use core::{cell::RefCell, fmt::Debug};

use hotg_rune_core::{outputs, AsElementType, ElementType, Tensor};
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

use crate::intrinsics;

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct Serial {
    id: u32,
    buffer: RefCell<Vec<u8>>,
}

impl Serial {
    const INITIAL_BUFFER_SIZE: usize = 1024;

    pub fn new() -> Self {
        unsafe {
            Serial {
                id: intrinsics::request_output(outputs::SERIAL),
                buffer: RefCell::new(
                    alloc::vec![0; Serial::INITIAL_BUFFER_SIZE],
                ),
            }
        }
    }

    fn log(&self, msg: &[u8]) {
        unsafe {
            intrinsics::consume_output(self.id, msg.as_ptr(), msg.len() as u32);
        }
    }

    fn consume_serializable(&self, msg: &Value) {
        let mut buffer = self.buffer.borrow_mut();

        // Keep resizing our internal buffer until it's big enough to hold the
        // full message. If we try to use more memory than the WebAssembly VM
        // wants to give us, this will OOM and we'll abort the Rune.
        //
        // In general that's the desired outcome because it means you've
        // designed a Rune that asks for more resources than its environment can
        // provide, and we should blow up loudly.
        loop {
            match serde_json_core::to_slice(msg, &mut buffer[..]) {
                Ok(bytes_written) => {
                    self.log(&buffer[..bytes_written]);
                    return;
                },
                Err(serde_json_core::ser::Error::BufferFull) => {
                    let new_len = buffer.len() * 2;
                    buffer.resize(new_len, 0);
                },
                Err(e) => panic!("Unable to serialize the data as JSON: {}", e),
            }
        }
    }

    pub fn consume<T>(&mut self, input: T)
    where
        T: IntoSerialMessage,
    {
        let msg = input.into_serial_message(self.id);
        self.consume_serializable(&msg);
    }
}

impl Default for Serial {
    fn default() -> Self { Serial::new() }
}

/// An intermediate trait which lets you convert from some input into a
/// serializable form suitable for sending back to the Rune runtime.
pub trait IntoSerialMessage {
    fn into_serial_message(self, channel: u32) -> Value;
}

impl<T: Serialize + AsElementType> IntoSerialMessage for Tensor<T> {
    fn into_serial_message(self, channel: u32) -> Value {
        let msg = TensorMessage {
            type_name: T::TYPE.rune_name(),
            channel,
            tensor: self,
        };

        serde_json::to_value(&msg).expect("Unable to serialize the message")
    }
}

impl IntoSerialMessage for &'static str {
    fn into_serial_message(self, channel: u32) -> Value {
        let msg = StringMessage {
            type_name: ElementType::String.rune_name(),
            string: self,
            channel,
        };
        serde_json::to_value(&msg).expect("Unable to serialize the message")
    }
}

pub struct TensorMessage<T> {
    type_name: &'static str,
    channel: u32,
    tensor: Tensor<T>,
}

impl<T: Serialize> Serialize for TensorMessage<T> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let mut map = ser.serialize_map(Some(4))?;

        map.serialize_key("type_name")?;
        map.serialize_value(&self.type_name)?;
        map.serialize_key("channel")?;
        map.serialize_value(&self.channel)?;
        map.serialize_key("elements")?;
        map.serialize_value(self.tensor.elements())?;
        map.serialize_key("dimensions")?;
        map.serialize_value(self.tensor.dimensions())?;

        map.end()
    }
}

#[derive(serde::Serialize)]
pub struct StringMessage<'a> {
    type_name: &'static str,
    channel: u32,
    string: &'a str,
}

macro_rules! tuple_serial_message {
    ($first:ident $(, $rest:ident)* $(,)?) => {
        impl<$first, $($rest),*> IntoSerialMessage for ($first, $($rest),*)
        where
            $first: IntoSerialMessage,
            $(
                $rest: IntoSerialMessage,
            )*
        {
            #[allow(non_snake_case)]
            fn into_serial_message(self, channel: u32) -> Value {
                let ($first, $($rest),*) = self;

                Value::Array(alloc::vec![
                    $first.into_serial_message(channel),
                    $($rest.into_serial_message(channel)),*
                ])
            }

        }

        tuple_serial_message!($($rest),*);
    };
    ($(,)?) => {};
}

tuple_serial_message!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16,
    T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30,
);
