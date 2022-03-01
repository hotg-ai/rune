use core::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// A dynamically typed value that may be passed back and forth across the
/// runtime.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Value {
    Byte(u8),
    Short(i16),
    Integer(i32),
    Float(f32),
    SignedByte(i8),
}

impl Value {
    /// Get a buffer big enough to be used with [`Value::to_le_bytes()`].
    pub const fn buffer() -> [u8; core::mem::size_of::<Value>()] {
        [0; core::mem::size_of::<Value>()]
    }

    pub fn from_le_bytes(ty: Type, bytes: &[u8]) -> Option<Self> {
        match ty {
            Type::Byte => bytes.get(0).copied().map(Value::Byte),
            Type::SignedByte => {
                if let [byte, ..] = bytes {
                    Some(Value::SignedByte(i8::from_le_bytes([*byte])))
                } else {
                    None
                }
            },
            Type::Short => {
                const LEN: usize = core::mem::size_of::<i16>();

                bytes.get(..LEN).map(|bytes| {
                    let mut buffer = [0; LEN];
                    buffer.copy_from_slice(bytes);
                    Value::Short(i16::from_le_bytes(buffer))
                })
            },
            Type::Integer => {
                const LEN: usize = core::mem::size_of::<i32>();

                bytes.get(..LEN).map(|bytes| {
                    let mut buffer = [0; LEN];
                    buffer.copy_from_slice(bytes);
                    Value::Integer(i32::from_le_bytes(buffer))
                })
            },
            Type::Float => {
                const LEN: usize = core::mem::size_of::<f32>();

                bytes.get(..LEN).map(|bytes| {
                    let mut buffer = [0; LEN];
                    buffer.copy_from_slice(bytes);
                    Value::Float(f32::from_le_bytes(buffer))
                })
            },
        }
    }

    /// Write this [`Value`]'s underlying value to the start of the provided
    /// buffer, returning the number of bytes written.
    ///
    /// The buffer should have at least `core::mem::size_of::<Value>()` bytes.
    /// You can use the [`Value::buffer()`] helper for creating an adequately
    /// sized buffer.
    pub fn to_le_bytes(self, buffer: &mut [u8]) -> usize {
        match self {
            Value::Byte(b) => {
                buffer[0] = b;
                1
            },
            Value::SignedByte(b) => {
                buffer[..1].copy_from_slice(&b.to_le_bytes());
                1
            },
            Value::Short(short) => {
                let bytes = short.to_le_bytes();
                buffer[..bytes.len()].copy_from_slice(&bytes);
                bytes.len()
            },
            Value::Integer(int) => {
                let bytes = int.to_le_bytes();
                buffer[..bytes.len()].copy_from_slice(&bytes);
                bytes.len()
            },
            Value::Float(float) => {
                let bytes = float.to_le_bytes();
                buffer[..bytes.len()].copy_from_slice(&bytes);
                bytes.len()
            },
        }
    }

    pub fn ty(&self) -> Type {
        match self {
            Value::Byte(_) => Type::Byte,
            Value::SignedByte(_) => Type::SignedByte,
            Value::Short(_) => Type::Short,
            Value::Integer(_) => Type::Integer,
            Value::Float(_) => Type::Float,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Byte(b) => write!(f, "{}_u8", b),
            Value::SignedByte(b) => write!(f, "{}_i8", b),
            Value::Short(s) => write!(f, "{}_i16", s),
            Value::Integer(i) => write!(f, "{}_i32", i),
            Value::Float(float) => write!(f, "{:.1}", float),
        }
    }
}

impl FromStr for Value {
    type Err = core::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(integer) = s.parse() {
            Ok(Value::Integer(integer))
        } else {
            s.parse().map(Value::Float)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum Type {
    /// A 32-bit signed integer.
    Integer = 1,
    /// A 32-bit floating point number.
    Float = 2,
    /// An 8-bit unsigned integer.
    Byte = 5,
    /// A 16-bit signed integer.
    Short = 6,
    // Note: Enum discriminant are important here. We want to stay
    // compatible with PARAM_TYPE so the mobile runtime isn't broken.
    //
    // https://github.com/hotg-ai/runic_mobile/blob/94f9e72d6de8bd57c004952dc3ba31adc7603381/ios/Runner/hmr/hmr.hpp#L23-L29
    //
    // Don't forget to update TryFrom if you add new variants!
    //
    // We *could* use #[derive(FromPrimitive)] to automate things, but I'd
    // prefer not to add a proc-macro dependency to the crate that every
    // single rune or proc block will depend on.
    SignedByte = 7,
}

impl From<Type> for u32 {
    fn from(t: Type) -> Self { t as u32 }
}

impl TryFrom<u32> for Type {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Type::Integer),
            2 => Ok(Type::Float),
            5 => Ok(Type::Byte),
            6 => Ok(Type::Short),
            _ => Err(()),
        }
    }
}

/// A Rust primitive which has a corresponding [`Type`] and can be converted to
/// or from a [`Value`].
pub trait AsType
where
    Self: Into<Value>,
    Self: TryFrom<Value>,
{
    /// The corresponding [`Type`] variant.
    const TYPE: Type;
}

macro_rules! impl_as_type {
    ($($type:ty => $variant:ident),* $(,)?) => {
        $(
            impl AsType for $type {
                const TYPE: Type = Type::$variant;
            }

            impl From<$type> for Value {
                fn from(other: $type) -> Value {
                    Value::$variant(other)
                }
            }

            impl TryFrom<Value> for $type {
                type Error = InvalidConversionError;

                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    match value {
                        Value::$variant(v) => Ok(v),
                        _ => Err(InvalidConversionError {
                            value,
                            target_type: Type::$variant,
                        }),
                    }
                }
            }
        )*
    }
}

impl_as_type!(u8 => Byte, i16 => Short, i32 => Integer, f32 => Float, i8 => SignedByte);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InvalidConversionError {
    pub value: Value,
    pub target_type: Type,
}

impl Display for InvalidConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unable to convert {} ({:?}) to a {:?}",
            self.value,
            self.value.ty(),
            self.target_type
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidConversionError {}
