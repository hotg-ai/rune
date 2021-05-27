use core::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
use crate::{InvalidConversionError, Value};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum PixelFormat {
    RGB = 0,
    BGR = 1,
    GrayScale = 2,
}

impl From<PixelFormat> for i32 {
    fn from(p: PixelFormat) -> i32 { p as i32 }
}

impl From<PixelFormat> for u32 {
    fn from(p: PixelFormat) -> u32 { p as u32 }
}

impl From<PixelFormat> for Value {
    fn from(p: PixelFormat) -> Value { Value::Integer(p.into()) }
}

impl TryFrom<i32> for PixelFormat {
    type Error = PixelFormatConversionError;

    fn try_from(i: i32) -> Result<PixelFormat, Self::Error> {
        match i {
            0 => Ok(PixelFormat::RGB),
            1 => Ok(PixelFormat::BGR),
            2 => Ok(PixelFormat::GrayScale),
            _ => Err(PixelFormatConversionError::InvalidConstant { value: i }),
        }
    }
}

impl TryFrom<Value> for PixelFormat {
    type Error = PixelFormatConversionError;

    fn try_from(value: Value) -> Result<PixelFormat, Self::Error> {
        let integer = i32::try_from(value)?;
        PixelFormat::try_from(integer)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PixelFormatConversionError {
    InvalidConstant { value: i32 },
    Value(InvalidConversionError),
}

impl From<InvalidConversionError> for PixelFormatConversionError {
    fn from(e: InvalidConversionError) -> Self {
        PixelFormatConversionError::Value(e)
    }
}

impl Display for PixelFormatConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PixelFormatConversionError::InvalidConstant { value } => {
                write!(f, "{} isn't a valid pixel format", value)
            },
            PixelFormatConversionError::Value(_) => {
                write!(f, "Unable to convert from a Value")
            },
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PixelFormatConversionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PixelFormatConversionError::InvalidConstant { .. } => None,
            PixelFormatConversionError::Value(e) => Some(e),
        }
    }
}
