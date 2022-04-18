mod yaml;

use std::sync::Arc;

pub use self::yaml::*;

#[tracing::instrument(skip(src), err)]
pub fn parse(src: &str) -> Result<Document, ParseFailedDiagnostic> {
    Document::parse(src)
        .map_err(|e| ParseFailedDiagnostic { inner: Arc::new(e) })
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("Unable to parse the Runefile: {}", inner)]
pub struct ParseFailedDiagnostic {
    #[source]
    inner: Arc<serde_yaml::Error>,
}
