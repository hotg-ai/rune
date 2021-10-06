use core::{
    str::FromStr,
    fmt::{self, Display, Formatter},
    convert::TryFrom,
};

/// A type that is associated with an [`ElementType`].
pub trait AsElementType {
    const TYPE: ElementType;
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum ElementType {
    U8,
    I8,
    U16,
    I16,
    U32,
    F32,
    I32,
    U64,
    F64,
    I64,
    String,
}

impl ElementType {
    const BYTE: u32 = 5;
    const FLOAT: u32 = 2;
    const INTEGER: u32 = 1;
    const SHORT: u32 = 6;
    const SIGNED_BYTE: u32 = 7;

    /// The ID used when passing this [`ElementType`] to the Rune runtime.
    ///
    /// This is the inverse of [`ElementType::from_runtime_id()`].
    pub fn runtime_id(self) -> Option<u32> {
        // Note: The constants used here are important. We want to stay
        // compatible with PARAM_TYPE so the mobile runtime isn't broken.
        //
        // https://github.com/hotg-ai/runic_mobile/blob/94f9e72d6de8bd57c004952dc3ba31adc7603381/ios/Runner/hmr/hmr.hpp#L23-L29

        match self {
            ElementType::I32 => Some(ElementType::INTEGER),
            ElementType::F32 => Some(ElementType::FLOAT),
            ElementType::U8 => Some(ElementType::BYTE),
            ElementType::I16 => Some(ElementType::SHORT),
            ElementType::I8 => Some(ElementType::SIGNED_BYTE),
            _ => None,
        }
    }

    /// Try to get the [`ElementType`] which corresponds to this `id`.
    ///
    /// This is the inverse of [`ElementType::runtime_id()`].
    pub fn from_runtime_id(id: u32) -> Option<Self> {
        match id {
            ElementType::INTEGER => Some(ElementType::I32),
            ElementType::FLOAT => Some(ElementType::F32),
            ElementType::BYTE => Some(ElementType::U8),
            ElementType::SHORT => Some(ElementType::I16),
            ElementType::SIGNED_BYTE => Some(ElementType::I8),
            _ => None,
        }
    }

    pub fn size_of(self) -> Option<usize> {
        match self {
            ElementType::U8 => Some(core::mem::size_of::<u8>()),
            ElementType::I8 => Some(core::mem::size_of::<i8>()),
            ElementType::U16 => Some(core::mem::size_of::<u16>()),
            ElementType::I16 => Some(core::mem::size_of::<i16>()),
            ElementType::U32 => Some(core::mem::size_of::<u32>()),
            ElementType::F32 => Some(core::mem::size_of::<f32>()),
            ElementType::I32 => Some(core::mem::size_of::<i32>()),
            ElementType::U64 => Some(core::mem::size_of::<u64>()),
            ElementType::F64 => Some(core::mem::size_of::<f64>()),
            ElementType::I64 => Some(core::mem::size_of::<i64>()),
            ElementType::String => None,
        }
    }

    pub fn rune_name(self) -> &'static str {
        match self {
            ElementType::U8 => "u8",
            ElementType::I8 => "i8",
            ElementType::U16 => "u16",
            ElementType::I16 => "i16",
            ElementType::U32 => "u32",
            ElementType::I32 => "i32",
            ElementType::F32 => "f32",
            ElementType::U64 => "u64",
            ElementType::I64 => "i64",
            ElementType::F64 => "f64",
            ElementType::String => "utf8",
        }
    }

    pub fn from_rune_name(name: &str) -> Option<Self> {
        match name {
            "u8" => Some(ElementType::U8),
            "i8" => Some(ElementType::I8),
            "u16" => Some(ElementType::U16),
            "i16" => Some(ElementType::I16),
            "u32" => Some(ElementType::U32),
            "i32" => Some(ElementType::I32),
            "f32" => Some(ElementType::F32),
            "u64" => Some(ElementType::U64),
            "i64" => Some(ElementType::I64),
            "f64" => Some(ElementType::F64),
            "utf8" => Some(ElementType::String),
            _ => None,
        }
    }
}

impl Display for ElementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.rune_name())
    }
}

impl FromStr for ElementType {
    type Err = UnknownElementType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ElementType::from_rune_name(s).ok_or(UnknownElementType)
    }
}

impl<'a> TryFrom<&'a str> for ElementType {
    type Error = UnknownElementType;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        ElementType::from_rune_name(value).ok_or(UnknownElementType)
    }
}

impl AsElementType for u8 {
    const TYPE: ElementType = ElementType::U8;
}

impl AsElementType for i8 {
    const TYPE: ElementType = ElementType::I8;
}

impl AsElementType for u16 {
    const TYPE: ElementType = ElementType::U16;
}

impl AsElementType for i16 {
    const TYPE: ElementType = ElementType::I16;
}

impl AsElementType for u32 {
    const TYPE: ElementType = ElementType::U32;
}

impl AsElementType for f32 {
    const TYPE: ElementType = ElementType::F32;
}

impl AsElementType for i32 {
    const TYPE: ElementType = ElementType::I32;
}

impl AsElementType for u64 {
    const TYPE: ElementType = ElementType::U64;
}

impl AsElementType for f64 {
    const TYPE: ElementType = ElementType::F64;
}

impl AsElementType for i64 {
    const TYPE: ElementType = ElementType::I64;
}

impl AsElementType for alloc::borrow::Cow<'static, str> {
    const TYPE: ElementType = ElementType::String;
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct UnknownElementType;

impl Display for UnknownElementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Unknown element type")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UnknownElementType {}
