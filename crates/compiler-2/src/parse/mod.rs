//! The YAML frontend for the Rune compiler.
//!
//! You are probably here for either the [`Frontend`] trait or the [`Document`]
//! type.

mod query;
mod yaml;

use std::sync::Arc;

use crate::Text;

pub use self::{
    query::{Frontend, FrontendStorage},
    yaml::*,
};

#[derive(Debug, Clone, thiserror::Error)]
#[error("Unable to parse the Runefile")]
pub struct ParseFailed {
    #[from]
    pub error: Arc<serde_yaml::Error>,
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
pub enum ItemType {
    Input,
    Model,
    ProcBlock,
    Output,
    Resource,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    thiserror::Error,
    serde::Serialize,
    serde::Deserialize,
)]
#[error("There is no model called \"{}\"", name)]
#[serde(rename_all = "kebab-case")]
pub struct NotFound {
    pub item_type: ItemType,
    pub name: Text,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    thiserror::Error,
    serde::Serialize,
    serde::Deserialize,
)]
#[error(
    "Expected \"{}\" to be a {:?}, but it is actually a {:?}",
    name,
    expected,
    actual
)]
#[serde(rename_all = "kebab-case")]
pub struct WrongItemType {
    pub expected: ItemType,
    pub actual: ItemType,
    pub name: Text,
}
