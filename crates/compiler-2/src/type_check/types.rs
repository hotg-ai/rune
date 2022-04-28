use std::num::NonZeroUsize;

use im::{OrdMap, Vector};

use crate::{lowering::NodeId, Text};

pub type Arguments = OrdMap<Text, Text>;

/// The fully resolved pipeline.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct Pipeline {
    pub arguments: OrdMap<NodeId, Arguments>,
    pub edges: Vector<Edge>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Edge {
    pub previous: Port,
    pub next: Vector<Port>,
    pub shape: TensorShape,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Port {
    pub node: NodeId,
    pub name: Text,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct TensorShape {
    pub element_type: ElementType,
    pub dimensions: Dimensions,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Dimensions {
    Dynamic,
    FixedRank(Vector<Dimension>),
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
pub enum Dimension {
    VariableLength,
    Fixed(NonZeroUsize),
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
pub enum ElementType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    Utf8,
}
