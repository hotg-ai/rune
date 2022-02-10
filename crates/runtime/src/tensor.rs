use std::{
    num::NonZeroUsize,
    fmt::{Debug, Formatter, self, Display},
};

#[derive(Clone, PartialEq)]
pub struct Tensor {
    element_type: ElementType,
    dimensions: Vec<NonZeroUsize>,
    buffer: Vec<u8>,
}

impl Tensor {
    /// The tensor's element type.
    pub fn element_type(&self) -> ElementType { self.element_type }

    /// Get a reference to the tensor's dimensions.
    pub fn dimensions(&self) -> &[NonZeroUsize] { self.dimensions.as_ref() }

    /// Get a reference to the tensor's buffer.
    pub fn buffer(&self) -> &[u8] { &self.buffer }

    /// Get a mutable reference to the tensor's buffer.
    pub fn buffer_mut(&mut self) -> &mut [u8] { &mut self.buffer }

    pub fn shape(&self) -> impl Display + '_ {
        struct Shape<'a>(&'a Tensor);
        impl Display for Shape<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}[", self.0.element_type)?;

                for (i, dimension) in self.0.dimensions.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", dimension)?;
                }

                Ok(())
            }
        }

        Shape(self)
    }
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

impl Display for ElementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ElementType::U8 => write!(f, "u8"),
            ElementType::I8 => write!(f, "i8"),
            ElementType::U16 => write!(f, "u16"),
            ElementType::I16 => write!(f, "i16"),
            ElementType::U32 => write!(f, "u32"),
            ElementType::I32 => write!(f, "i32"),
            ElementType::F32 => write!(f, "f32"),
            ElementType::U64 => write!(f, "u64"),
            ElementType::I64 => write!(f, "i64"),
            ElementType::F64 => write!(f, "f64"),
        }
    }
}
