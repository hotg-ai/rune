//! A simple reflection system.

use core::any::TypeId;
use alloc::borrow::Cow;

/// A type known to the reflection system.
pub trait ReflectionType {
    const TYPE: Type;
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Type {
    /// An integer type.
    Integer { signed: bool, bit_width: usize },
    /// An IEEE floating point number.
    Float { bit_width: usize },
    /// A `&'static str`.
    String,
    /// An opaque type.
    Opaque { type_name: Cow<'static, str> },
}

macro_rules! declare_type {
        ($( $name:ident => $value:expr),* $(,)?) => {
            #[allow(non_upper_case_globals)]
            impl Type {
                $(
                    pub const $name: Self = $value;
                )*
                pub const str: Self = Type::String;

                /// Try to get the [`Type`] for some type `T`, falling back to
                /// [`Type::Opaque`] if unknown.
                pub fn of<T>() -> Self
                where
                    T: Sized + 'static,
                {
                    let type_id = TypeId::of::<T>();

                    $(
                        if type_id == TypeId::of::<$name>() {
                            return Type::$name;
                        }
                    )*

                    if type_id == TypeId::of::<&'static str>() {
                        return Type::String;
                    }

                    Type::Opaque {
                        type_name: core::any::type_name::<T>().into(),
                    }
                }

                pub fn from_rust_name(name: &str) -> Option<Type> {
                    match name {
                        "str" | "&str" => Some(Type::str),
                        $(
                            stringify!($name) => Some(Type::$name),
                        )*
                        _ => None,
                    }
                }

                /// Get the common name for this type.
                pub fn rust_name(&self) -> Option<&str> {
                    match *self {
                        Type::String => Some("str"),
                        $(
                          Type::$name => Some(stringify!($name)),
                        )*
                        Type::Opaque { ref type_name } => Some(type_name.as_ref()),
                        _ => None,
                    }
                }
            }

            $(
                impl ReflectionType for $name {
                    const TYPE: Type = Type::$name;
                }
            )*

            impl ReflectionType  for &'static str {
                    const TYPE: Type = Type::str;
            }
        };
    }

declare_type! {
    u8 => Type::Integer { signed: false, bit_width: 8 },
    i8 => Type::Integer { signed: true, bit_width: 8 },
    u16 => Type::Integer { signed: false, bit_width: 16 },
    i16 => Type::Integer { signed: true, bit_width: 16 },
    u32 => Type::Integer { signed: false, bit_width: 32 },
    i32 => Type::Integer { signed: true, bit_width: 32 },
    u64 => Type::Integer { signed: false, bit_width: 64 },
    i64 => Type::Integer { signed: true, bit_width: 64 },
    u128 => Type::Integer { signed: false, bit_width: 128 },
    i128 => Type::Integer { signed: true, bit_width: 128 },
    f32 => Type::Float { bit_width: 32 },
    f64 => Type::Float { bit_width: 64 },
}

impl Type {
    const BYTE: u32 = 5;
    const FLOAT: u32 = 2;
    const INTEGER: u32 = 1;
    const SHORT: u32 = 6;
    const SIGNED_BYTE: u32 = 7;

    /// The ID used when passing this [`Type`] to the Rune runtime.
    ///
    /// This is the inverse of [`Type::from_runtime_id()`].
    pub fn runtime_id(self) -> Option<u32> {
        // Note: The constants used here are important. We want to stay
        // compatible with PARAM_TYPE so the mobile runtime isn't broken.
        //
        // https://github.com/hotg-ai/runic_mobile/blob/94f9e72d6de8bd57c004952dc3ba31adc7603381/ios/Runner/hmr/hmr.hpp#L23-L29

        match self {
            Type::i32 => Some(Type::INTEGER),
            Type::f32 => Some(Type::FLOAT),
            Type::u8 => Some(Type::BYTE),
            Type::i16 => Some(Type::SHORT),
            Type::i8 => Some(Type::SIGNED_BYTE),
            _ => None,
        }
    }

    /// Try to get the [`Type`] which corresponds to this `id`.
    ///
    /// This is the inverse of [`Type::runtime_id()`].
    pub fn from_runtime_id(id: u32) -> Option<Self> {
        match id {
            Type::INTEGER => Some(Type::i32),
            Type::FLOAT => Some(Type::f32),
            Type::BYTE => Some(Type::u8),
            Type::SHORT => Some(Type::i16),
            Type::SIGNED_BYTE => Some(Type::i8),
            _ => None,
        }
    }
}
