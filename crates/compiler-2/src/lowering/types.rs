use std::num::NonZeroU32;

use im::Vector;

use crate::{
    diagnostics::{AsDiagnostic, Diagnostic, DiagnosticMetadata, Severity},
    parse::ResourceType,
    Text,
};

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
pub enum Abi {
    V0,
    V1,
}

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
#[repr(transparent)]
pub struct ResourceId(Option<NonZeroU32>);

impl ResourceId {
    pub const ERROR: ResourceId = ResourceId(None);

    pub fn is_error(self) -> bool { self == Self::ERROR }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct Resource {
    /// Where to read the [`Resource`]'s default value from.
    pub default_value: Option<ResourceSource>,
    pub ty: ResourceType,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum ResourceSource {
    /// The value is specified in-line as a string.
    Inline(Text),
    /// The value should be read from a file.
    FromDisk { filename: Text },
}

impl ResourceSource {
    pub fn inline(value: impl Into<Text>) -> Self {
        ResourceSource::Inline(value.into())
    }

    pub fn from_disk(filename: impl Into<Text>) -> Self {
        ResourceSource::FromDisk {
            filename: filename.into(),
        }
    }
}

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
#[repr(transparent)]
pub struct NodeId(Option<NonZeroU32>);

impl NodeId {
    pub const ERROR: NodeId = NodeId(None);

    pub fn is_error(self) -> bool { self == Self::ERROR }
}

/// A node in the ML pipeline.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Node {
    pub kind: NodeKind,
    pub identifier: ResourceOrText,
    pub inputs: Vector<Input>,
    pub outputs: Vector<crate::parse::Type>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Input {
    pub node: NodeId,
    pub index: usize,
}

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
pub enum NodeKind {
    Input,
    ProcBlock,
    Model,
    Output,
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct ArgumentId(Option<NonZeroU32>);

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Argument {
    pub value: ResourceOrText,
}

/// A monotonic counter that can be used to generate unique HIR identifiers.
#[derive(Debug)]
pub struct Identifiers {
    next_id: u32,
}

impl Identifiers {
    pub const fn new() -> Self { Identifiers { next_id: 0 } }

    pub fn node(&mut self) -> NodeId { NodeId(Some(self.next())) }

    pub fn resource(&mut self) -> ResourceId { ResourceId(Some(self.next())) }

    pub fn argument(&mut self) -> ArgumentId { ArgumentId(Some(self.next())) }

    pub fn next(&mut self) -> NonZeroU32 {
        self.next_id += 1;
        NonZeroU32::new(self.next_id).expect("Unreachable")
    }
}

impl Default for Identifiers {
    fn default() -> Self { Identifiers::new() }
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
#[error("The name \"{}\" is used as both a resource and a node", name)]
pub struct DuplicateName {
    pub resource_id: ResourceId,
    pub node_id: NodeId,
    pub name: Text,
}

impl AsDiagnostic for DuplicateName {
    fn meta() -> DiagnosticMetadata {
        DiagnosticMetadata::new("Duplicate Name")
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ResourceOrText {
    Text(Text),
    Resource(ResourceId),
    Error,
}

impl ResourceOrText {
    pub fn text(value: impl Into<Text>) -> Self {
        ResourceOrText::Text(value.into())
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
pub struct UnknownInput {
    pub input: crate::parse::Input,
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
pub struct UnknownAbi {
    pub image: crate::parse::Image,
}

impl AsDiagnostic for UnknownAbi {
    fn meta() -> DiagnosticMetadata { DiagnosticMetadata::new("Unknown ABI") }
}
