use core::convert::TryFrom;

/// A dynamically typed value that may be passed back and forth across the
/// runtime.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Value {
    Byte(u8),
    Short(i16),
    Integer(i32),
    Float(f32),
}

impl Value {
    /// Get a buffer big enough to be used with [`Value::to_le_bytes()`].
    pub const fn buffer() -> [u8; core::mem::size_of::<Value>()] {
        [0; core::mem::size_of::<Value>()]
    }

    pub fn from_le_bytes(ty: Type, bytes: &[u8]) -> Self {
        match ty {
            Type::Byte => {
                assert!(bytes.len() >= 1);
                Value::Byte(bytes[0])
            },
            Type::Short => {
                let mut buffer = [0; core::mem::size_of::<i16>()];
                let len = buffer.len();
                buffer.copy_from_slice(&bytes[..len]);
                Value::Short(i16::from_le_bytes(buffer))
            },
            Type::Integer => {
                let mut buffer = [0; core::mem::size_of::<i32>()];
                let len = buffer.len();
                buffer.copy_from_slice(&bytes[..len]);
                Value::Integer(i32::from_le_bytes(buffer))
            },
            Type::Float => {
                let mut buffer = [0; core::mem::size_of::<f32>()];
                let len = buffer.len();
                buffer.copy_from_slice(&bytes[..len]);
                Value::Float(f32::from_le_bytes(buffer))
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
            Value::Short(_) => Type::Short,
            Value::Integer(_) => Type::Integer,
            Value::Float(_) => Type::Float,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum Type {
    Byte = 0,
    Short = 1,
    Integer = 2,
    Float = 3,
    /* Note: don't forget to update TryFrom if you add new variants!
     *
     * We *could* use #[derive(FromPrimitive)] to automate things, but I'd
     * prefer not to add a proc-macro dependency to the crate that every
     * single rune or proc block will depend on.
     */
}

impl From<Type> for u32 {
    fn from(t: Type) -> Self { t as u32 }
}

impl TryFrom<u32> for Type {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Type::Byte),
            1 => Ok(Type::Short),
            2 => Ok(Type::Integer),
            3 => Ok(Type::Float),
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
                type Error = ();

                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    match value {
                        Value::$variant(v) => Ok(v),
                        _ => Err(())
                    }
                }
            }
        )*
    }
}

impl_as_type!(u8 => Byte, i16 => Short, i32 => Integer, f32 => Float);
