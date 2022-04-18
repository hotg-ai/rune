//! The Runefile parser.

mod yaml;

use std::sync::Arc;

pub use self::yaml::*;
use crate::diagnostics::{
    AsDiagnostic, Diagnostic, DiagnosticMetadata, Severity,
};

/// Parse a `Runefile.yml`.
#[tracing::instrument(skip(src), err)]
pub fn parse_runefile(src: &str) -> Result<Document, ParseFailedDiagnostic> {
    Document::parse(src)
        .map_err(|e| ParseFailedDiagnostic { inner: Arc::new(e) })
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("Unable to parse the Runefile: {}", inner)]
pub struct ParseFailedDiagnostic {
    #[source]
    inner: Arc<serde_yaml::Error>,
}

impl AsDiagnostic for ParseFailedDiagnostic {
    fn meta() -> DiagnosticMetadata {
        DiagnosticMetadata::new("Parsing Failed")
    }

    fn as_diagnostic(&self) -> Diagnostic {
        Diagnostic::new(Severity::Error, self.to_string())
    }
}
