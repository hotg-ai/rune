//! The YAML frontend for the Rune compiler.
//!
//! You are probably here for the [`parse_runefile()`] function.

mod yaml;

use std::sync::Arc;

pub use self::yaml::*;

/// Parse a `Runefile.yml`.
#[tracing::instrument(skip(src), err)]
pub fn parse_runefile(src: &str) -> Result<Document, ParseFailed> {
    Document::parse(src).map_err(|e| ParseFailed { inner: Arc::new(e) })
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("Unable to parse the Runefile: {}", inner)]
pub struct ParseFailed {
    #[source]
    inner: Arc<serde_yaml::Error>,
}
