use std::{
    num::NonZeroUsize,
    fmt::{Debug, Formatter, self},
};

#[derive(Clone, PartialEq)]
pub struct Tensor {
    element_type: ElementType,
    dimensions: Vec<NonZeroUsize>,
    buffer: Vec<u8>,
}

impl Debug for Tensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Tensor {
            element_type,
            dimensions,
            buffer: _,
        } = self;

        f.debug_struct("Tensor")
            .field("element_type", element_type)
            .field("dimensions", dimensions)
            .finish()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ElementType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
}
