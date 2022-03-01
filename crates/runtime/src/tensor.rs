use std::{
    fmt::{self, Debug, Display, Formatter},
    num::NonZeroUsize,
};

/// A n-dimension array of numbers.
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tensor {
    element_type: ElementType,
    dimensions: Vec<NonZeroUsize>,
    buffer: Vec<u8>,
}

impl Tensor {
    pub fn new<E>(elements: &[E], dimensions: &[usize]) -> Self
    where
        E: TensorElement,
    {
        let buffer = E::to_bytes(elements).to_vec();
        let dimensions = dimensions
            .iter()
            .map(|&d| {
                NonZeroUsize::new(d).expect("All dimensions must be nonzero")
            })
            .collect();

        Tensor {
            element_type: E::ELEMENT_TYPE,
            dimensions,
            buffer,
        }
    }

    pub fn new_raw(
        element_type: ElementType,
        dimensions: Vec<NonZeroUsize>,
        buffer: Vec<u8>,
    ) -> Self {
        let num_elements: usize = dimensions.iter().map(|d| d.get()).product();
        let expected_length = num_elements * element_type.byte_size();

        assert_eq!(
            expected_length,
            buffer.len(),
            "A {} tensor should take up {} bytes, but {} bytes were provided",
            Shape::new(element_type, &dimensions),
            expected_length,
            buffer.len()
        );

        Tensor {
            element_type,
            dimensions,
            buffer,
        }
    }

    /// The tensor's element type.
    pub fn element_type(&self) -> ElementType { self.element_type }

    /// Get a reference to the tensor's dimensions.
    pub fn dimensions(&self) -> &[NonZeroUsize] { self.dimensions.as_ref() }

    /// Get a reference to the tensor's buffer.
    pub fn buffer(&self) -> &[u8] { &self.buffer }

    /// Get a mutable reference to the tensor's buffer.
    pub fn buffer_mut(&mut self) -> &mut [u8] { &mut self.buffer }

    pub fn shape(&self) -> impl Display + '_ {
        Shape::new(self.element_type(), self.dimensions())
    }
}

#[derive(Debug)]
struct Shape<'a> {
    element_type: ElementType,
    dimensions: &'a [NonZeroUsize],
}

impl<'a> Shape<'a> {
    fn new(element_type: ElementType, dimensions: &'a [NonZeroUsize]) -> Self {
        Self {
            element_type,
            dimensions,
        }
    }
}

impl Display for Shape<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Shape {
            element_type,
            dimensions,
        } = self;
        write!(f, "{}[", element_type)?;

        for (i, dimension) in dimensions.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", dimension)?;
        }

        write!(f, "]")?;

        Ok(())
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

/// The type of value that may be stored in a [`Tensor`].
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(u32)]
#[serde(rename_all = "kebab-case")]
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

impl ElementType {
    pub fn byte_size(self) -> usize {
        match self {
            ElementType::U8 => std::mem::size_of::<u8>(),
            ElementType::I8 => std::mem::size_of::<i8>(),
            ElementType::U16 => std::mem::size_of::<u16>(),
            ElementType::I16 => std::mem::size_of::<i16>(),
            ElementType::U32 => std::mem::size_of::<u32>(),
            ElementType::I32 => std::mem::size_of::<i32>(),
            ElementType::F32 => std::mem::size_of::<f32>(),
            ElementType::U64 => std::mem::size_of::<u64>(),
            ElementType::I64 => std::mem::size_of::<i64>(),
            ElementType::F64 => std::mem::size_of::<f64>(),
        }
    }
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

/// A numeric type that can be stored in a [`Tensor`].
pub trait TensorElement: sealed::Sealed + Copy + 'static {
    const ELEMENT_TYPE: ElementType;

    fn to_bytes(slice: &[Self]) -> &[u8];
    fn from_bytes(bytes: &[u8]) -> Option<&[Self]>;
}

mod sealed {
    pub trait Sealed {}
}

macro_rules! impl_tensor_element {
    ($type:ty => $element_type:expr) => {
        impl TensorElement for $type {
            const ELEMENT_TYPE: ElementType = $element_type;

            fn to_bytes(slice: &[Self]) -> &[u8] {
                // Safey: Always valid because primitive integers have no
                // padding or references to other parts of memory.
                unsafe {
                    std::slice::from_raw_parts(
                        slice.as_ptr().cast(),
                        std::mem::size_of_val(slice),
                    )
                }
            }

            fn from_bytes(bytes: &[u8]) -> Option<&[Self]> {
                // Safety: We know it will always be valid to transmute from
                // bytes to our type because TensorElement will only be
                // implemented for integer primitives
                unsafe {
                    let (head, elements, tail) = bytes.align_to();

                    if head.is_empty() && tail.is_empty() {
                        Some(elements)
                    } else {
                        None
                    }
                }
            }
        }

        impl sealed::Sealed for $type {}
    };
}

impl_tensor_element!(u8 => ElementType::U8);
impl_tensor_element!(i8 => ElementType::I8);
impl_tensor_element!(u16 => ElementType::U16);
impl_tensor_element!(i16 => ElementType::I16);
impl_tensor_element!(u32 => ElementType::U32);
impl_tensor_element!(i32 => ElementType::I32);
impl_tensor_element!(f32 => ElementType::F32);
impl_tensor_element!(u64 => ElementType::U64);
impl_tensor_element!(i64 => ElementType::I64);
impl_tensor_element!(f64 => ElementType::F64);
