use crate::{wasm32::intrinsics, Sink, outputs, Tensor};
use serde::ser::{Serialize, Serializer, SerializeMap};
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

impl<T> Sink<T> for Serial
where
    T: IntoSerialMessage,
{
    fn consume(&mut self, input: T) {
        let msg = input.into_serial_message(self.id);
        self.consume_serializable(&msg);
    }
}

/// An intermediate trait which lets you convert from some input into a
/// serializable form suitable for sending back to the Rune runtime.
pub trait IntoSerialMessage {
    type Message: Serialize;

    fn into_serial_message(self, channel: u32) -> Self::Message;
}

impl<T: Serialize> IntoSerialMessage for Tensor<T> {
    type Message = TensorMessage<T>;

    fn into_serial_message(self, channel: u32) -> Self::Message {
        TensorMessage {
            type_name: core::any::type_name::<T>(),
            channel,
            tensor: self,
        }
    }
}

impl IntoSerialMessage for &'static str {
    type Message = StringMessage<'static>;

    fn into_serial_message(self, channel: u32) -> Self::Message {
        StringMessage {
            type_name: "&str",
            string: self,
            channel,
        }
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
            type Message = (
                <$first as IntoSerialMessage>::Message,
                $( <$rest as IntoSerialMessage>::Message),*
            );

            #[allow(non_snake_case)]
            fn into_serial_message(self, channel: u32) -> Self::Message {
                let ($first, $($rest),*) = self;

                (
                    $first.into_serial_message(channel),
                    $($rest.into_serial_message(channel)),*
                )
            }

        }

        tuple_serial_message!($($rest),*);
    };
    ($(,)?) => {};
}

tuple_serial_message!(A, B, C, D, E, F, G, H, I, J, K);
