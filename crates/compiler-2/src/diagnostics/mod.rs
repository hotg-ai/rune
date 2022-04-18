use im::Vector;

use crate::Text;

pub struct DiagnosticMetadata {
    pub title: Text,
    pub code: Option<Text>,
    pub description: Option<Text>,
}

impl DiagnosticMetadata {
    pub fn new(title: impl Into<Text>) -> Self {
        DiagnosticMetadata {
            title: title.into(),
            code: None,
            description: None,
        }
    }

    pub fn with_code(self, code: impl Into<Text>) -> Self {
        DiagnosticMetadata {
            code: Some(code.into()),
            ..self
        }
    }

    pub fn with_description(self, description: impl Into<Text>) -> Self {
        DiagnosticMetadata {
            description: Some(description.into()),
            ..self
        }
    }
}

/// A severity level for diagnostic messages.
///
/// These are ordered in the following way:
///
/// ```rust
/// use hotg_rune_compiler_2::diagnostics::Severity;
///
/// assert!(Severity::Bug > Severity::Error);
/// assert!(Severity::Error > Severity::Warning);
/// assert!(Severity::Warning > Severity::Note);
/// assert!(Severity::Note > Severity::Help);
/// ```
#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Severity {
    /// A help message.
    Help,
    /// A note.
    Note,
    /// A warning.
    Warning,
    /// An error.
    Error,
    /// An unexpected bug.
    Bug,
}

/// A collection of [`Diagnostic`]s.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct Diagnostics(Vec<Diagnostic>);

/// Something that can be used to create a [`Diagnostic`].
pub trait AsDiagnostic: std::error::Error {
    fn meta() -> DiagnosticMetadata;
    fn as_diagnostic(&self) -> Diagnostic;
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: Text,
    pub primary_label: Option<Label>,
    pub labels: Vector<Label>,
    pub help: Option<Text>,
}

impl Diagnostic {
    pub fn new(severity: Severity, message: impl Into<Text>) -> Self {
        Self {
            severity,
            message: message.into(),
            primary_label: None,
            labels: Vector::new(),
            help: None,
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Label {
    /// A number that refers to the item being labeled.
    pub target_id: u32,
    pub message: Option<Text>,
}
