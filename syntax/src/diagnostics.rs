use codespan::FileId;
use codespan_reporting::diagnostic::{Diagnostic, Severity};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Diagnostics(Vec<Diagnostic<FileId>>);

impl Diagnostics {
    pub fn new() -> Self { Diagnostics(Vec::new()) }

    pub fn iter(&self) -> impl Iterator<Item = &'_ Diagnostic<FileId>> + '_ {
        self.0.iter()
    }

    pub fn has_severity(&self, severity: Severity) -> bool {
        self.iter().any(|diag| diag.severity >= severity)
    }

    pub fn has_errors(&self) -> bool { self.has_severity(Severity::Error) }

    pub fn has_warnings(&self) -> bool { self.has_severity(Severity::Warning) }

    pub fn push(&mut self, diag: Diagnostic<FileId>) { self.0.push(diag); }
}
