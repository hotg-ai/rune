use alloc::{string::ToString, vec::Vec};

use hotg_rune_core::{outputs, AsElementType, Tensor};

pub struct TensorOutput {
    id: u32,
    buffer: Vec<u8>,
}

impl TensorOutput {
    pub fn new() -> Self {
        unsafe {
            TensorOutput {
                id: crate::intrinsics::request_output(outputs::TENSOR),
                buffer: Vec::new(),
            }
        }
    }

    pub fn consume<'a>(&mut self, inputs: impl Writable) {
        self.buffer.clear();
        inputs.encode(&mut self.buffer);

        unsafe {
            crate::intrinsics::consume_output(
                self.id,
                self.buffer.as_ptr(),
                self.buffer.len() as u32,
            );
        }
    }
}

impl Default for TensorOutput {
    fn default() -> Self { TensorOutput::new() }
}

pub trait Writable {
    fn encode(&self, buffer: &mut Vec<u8>);
}

impl<E> Writable for Tensor<E>
where
    E: AsElementType,
{
    fn encode(&self, buffer: &mut Vec<u8>) {
        let shape = self.shape().to_string();
        let shape_len = (shape.len() as u32).to_le_bytes();
        buffer.extend(&shape_len);
        buffer.extend(shape.as_bytes());

        let (ptr, len) = self.as_ptr_and_byte_length();
        let data = unsafe { core::slice::from_raw_parts(ptr, len) };
        buffer.extend(data);
    }
}

macro_rules! tuple_writable {
    ($first:ident $(, $rest:ident)* $(,)?) => {
        impl<$first, $($rest),*> Writable for ($first, $($rest),*)
        where
            $first: Writable,
            $(
                $rest: Writable,
            )*
        {
            #[allow(non_snake_case)]
            fn encode(&self, buffer: &mut Vec<u8>) {
                let ($first, $($rest),*) = self;

                $first.encode(buffer);
                $(
                    $rest.encode(buffer);
                )*
            }
        }

        tuple_writable!($($rest),*);
    };
    ($(,)?) => {};
}

tuple_writable!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16,
    T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
);
