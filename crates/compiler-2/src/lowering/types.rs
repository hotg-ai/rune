use im::Vector;

use crate::{parse::ResourceType, Text};

intern_id! {
    pub struct ResourceId(salsa::InternId);
    pub struct ArgumentId(salsa::InternId);
    pub struct NodeId(salsa::InternId);
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
#[serde(rename_all = "kebab-case")]
pub enum HirId {
    Node(NodeId),
    Resource(ResourceId),
}

impl HirId {
    pub fn as_node(self) -> Option<NodeId> {
        match self {
            HirId::Node(id) => Some(id),
            _ => None,
        }
    }

    pub fn as_resource(self) -> Option<ResourceId> {
        match self {
            HirId::Resource(id) => Some(id),
            _ => None,
        }
    }
}

impl From<ResourceId> for HirId {
    fn from(v: ResourceId) -> Self { Self::Resource(v) }
}

impl From<NodeId> for HirId {
    fn from(v: NodeId) -> Self { Self::Node(v) }
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
#[serde(rename_all = "kebab-case")]
pub enum Abi {
    V0,
    V1,
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

/// A node in the ML pipeline.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct Node {
    pub kind: NodeKind,
    pub identifier: ResourceOrText,
    pub outputs: Vector<crate::parse::Type>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
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
#[serde(rename_all = "kebab-case")]
pub enum NodeKind {
    Input,
    ProcBlock,
    Model,
    Output,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct Argument {
    pub value: ResourceOrText,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
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
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct Port {}
