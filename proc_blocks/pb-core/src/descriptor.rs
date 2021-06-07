use alloc::borrow::Cow;

/// A description of everything a particular proc block is capable of.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcBlockDescriptor<'a> {
    /// The fully qualified name for this proc block's type (typically
    /// retrieved via [`core::any::type_name()`]).
    pub type_name: Cow<'a, str>,
    /// A human-friendly description of what this proc block does.
    ///
    /// Similar to how Rust types are normally documented, this should consist
    /// of a short one-line summary with more information in subsequent
    /// paragraphs.
    pub description: Cow<'a, str>,
    pub parameters: Cow<'a, [ParameterDescriptor<'a>]>,
    pub available_transforms: Cow<'a, [TransformDescriptor<'a>]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDescriptor<'a> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub parameter_type: runic_types::reflect::Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransformDescriptor<'a> {
    pub input: TensorDescriptor<'a>,
    pub output: TensorDescriptor<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TensorDescriptor<'a> {
    pub element_type: runic_types::reflect::Type,
    pub dimensions: Cow<'a, [Dimension]>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dimension {
    Any,
    Value(usize),
}
