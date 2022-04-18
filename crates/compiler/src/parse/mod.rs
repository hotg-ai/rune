mod yaml;

use std::sync::Arc;

use codespan_reporting::diagnostic::{Diagnostic, Label};

pub use self::yaml::*;

#[tracing::instrument(skip(src), err)]
pub fn parse(src: &str) -> Result<Document, ParseFailedDiagnostic> {
    Document::parse(src)
        .map_err(|e| ParseFailedDiagnostic { inner: Arc::new(e) })
}

#[derive(Debug, Clone, thiserror::Error, miette::Diagnostic)]
#[error("Unable to parse the Runefile: {}", inner)]
#[diagnostic(code("P001"))]
pub struct ParseFailedDiagnostic {
    #[source]
    inner: Arc<serde_yaml::Error>,
}

impl ParseFailedDiagnostic {
    pub fn as_codespan_diagnostic(&self) -> Diagnostic<()> {
        let mut diag = Diagnostic::error().with_message(self.to_string());
        if let Some(location) = self.inner.location() {
            let ix = location.index();
            diag = diag.with_labels(vec![Label::primary((), ix..ix)]);
        }
        diag
    }
}
