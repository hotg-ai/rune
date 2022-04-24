use crate::{
    diagnostics::{AsDiagnostic, DiagnosticMetadata, Diagnostic, Severity},
    lowering::{HirId, ResourceId},
    Text,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
)]
#[error("The name \"{}\" is defined multiple times", name)]
#[serde(rename_all = "kebab-case")]
pub struct DuplicateName {
    pub original: HirId,
    pub duplicate: HirId,
    pub name: Text,
}

impl DuplicateName {
    pub fn new(original: HirId, duplicate: HirId, name: Text) -> Self {
        Self {
            original,
            duplicate,
            name,
        }
    }
}

impl AsDiagnostic for DuplicateName {
    fn meta() -> DiagnosticMetadata {
        DiagnosticMetadata::new("Duplicate Name")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
)]
#[error(
    "The \"{}\" resource defines both a \"path\" and \"inline\" default value",
    name
)]
#[serde(rename_all = "kebab-case")]
pub struct PathAndInlineNotAllowed {
    pub name: Text,
    pub id: ResourceId,
}

impl PathAndInlineNotAllowed {
    pub fn new(name: impl Into<Text>, id: ResourceId) -> Self {
        Self {
            name: name.into(),
            id,
        }
    }
}

impl AsDiagnostic for PathAndInlineNotAllowed {
    fn meta() -> DiagnosticMetadata {
        DiagnosticMetadata::new("Path and Inline Resources Not Allowed")
    }

    fn as_diagnostic(&self) -> Diagnostic {
        Diagnostic::from_impl(Severity::Warning, self)
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
)]
#[error("There is no resource called {}", name)]
#[serde(rename_all = "kebab-case")]
pub struct UnknownResource {
    pub name: crate::parse::ResourceName,
}

impl AsDiagnostic for UnknownResource {
    fn meta() -> DiagnosticMetadata {
        DiagnosticMetadata::new("Unknown Resource")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
)]
#[error("There is no node called \"{}\"", input.name)]
#[serde(rename_all = "kebab-case")]
pub struct UnknownInput {
    pub input: crate::parse::Input,
}

impl UnknownInput {
    pub fn new(input: crate::parse::Input) -> Self { Self { input } }
}

impl AsDiagnostic for UnknownInput {
    fn meta() -> DiagnosticMetadata { DiagnosticMetadata::new("Unknown Input") }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
)]
#[error("Unknown ABI, \"{}\"", image)]
#[serde(rename_all = "kebab-case")]
pub struct UnknownAbi {
    pub image: crate::parse::Image,
}

impl AsDiagnostic for UnknownAbi {
    fn meta() -> DiagnosticMetadata { DiagnosticMetadata::new("Unknown ABI") }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
)]
#[error("\"{}\" is not a resource", name)]
#[serde(rename_all = "kebab-case")]
pub struct NotAResource {
    pub name: crate::parse::ResourceName,
}

impl AsDiagnostic for NotAResource {
    fn meta() -> DiagnosticMetadata {
        DiagnosticMetadata::new("Not a Resource")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
)]
#[error(
    "The \"{}\" node uses \"{}\" as an input, but it is a resource",
    node,
    input
)]
#[serde(rename_all = "kebab-case")]
pub struct ResourceUsedAsInput {
    pub input: crate::parse::Input,
    pub node: Text,
    pub resource: ResourceId,
}

impl ResourceUsedAsInput {
    pub fn new(
        input: crate::parse::Input,
        node: Text,
        resource: ResourceId,
    ) -> Self {
        Self {
            input,
            node,
            resource,
        }
    }
}

impl AsDiagnostic for ResourceUsedAsInput {
    fn meta() -> DiagnosticMetadata {
        DiagnosticMetadata::new("Resource Used As Node Input")
    }
}
