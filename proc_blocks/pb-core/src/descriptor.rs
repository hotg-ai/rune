use alloc::borrow::Cow;

/// A description of everything a particular proc block is capable of.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProcBlockDescriptor<'a> {
    /// The name for this proc block's type.
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

impl<'a> ProcBlockDescriptor<'a> {
    pub const CUSTOM_SECTION_NAME: &'static str = ".rune_proc_block";

    /// Get a parameter by name.
    pub fn get_parameter(
        &self,
        name: &str,
    ) -> Option<&ParameterDescriptor<'a>> {
        self.parameters.iter().find(|p| p.name == name)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ParameterDescriptor<'a> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub parameter_type: runic_types::reflect::Type,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransformDescriptor<'a> {
    pub input: TensorDescriptor<'a>,
    pub output: TensorDescriptor<'a>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TensorDescriptor<'a> {
    pub element_type: runic_types::reflect::Type,
    pub dimensions: Dimensions<'a>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Dimensions<'a> {
    Finite(Cow<'a, [Dimension]>),
    Arbitrary,
}

impl<'a, D: Into<Cow<'a, [Dimension]>>> From<D> for Dimensions<'a> {
    fn from(dims: D) -> Self { Dimensions::Finite(dims.into()) }
}

#[derive(
    Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub enum Dimension {
    Any,
    Value(usize),
}
