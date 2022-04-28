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
    Clone,
    PartialEq,
    Eq,
    Hash,
    thiserror::Error,
    serde::Serialize,
    serde::Deserialize,
)]
#[error("There is no resource called \"{}\"", name)]
#[serde(rename_all = "kebab-case")]
pub struct NoSuchResource {
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
#[error("There is no proc-block called \"{}\"", name)]
#[serde(rename_all = "kebab-case")]
pub struct NoSuchProcBlock {
    pub name: Text,
}
