//! Diagnostics that may be shown to a user.

use im::Vector;

use crate::Text;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
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
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(transparent)]
#[must_use = "You should always check for errors before proceeding"]
pub struct Diagnostics(Vector<Diagnostic>);

impl Diagnostics {
    pub fn new() -> Self { Diagnostics(Vector::new()) }

    pub fn one(d: Diagnostic) -> Self {
        let mut diags = Diagnostics::new();
        diags.push(d);
        diags
    }

    pub fn push(&mut self, diag: Diagnostic) { self.0.push_back(diag); }

    pub fn iter(&self) -> impl Iterator<Item = &'_ Diagnostic> + '_ {
        self.0.iter()
    }

    pub fn len(&self) -> usize { self.0.len() }

    pub fn is_empty(&self) -> bool { self.len() == 0 }

    pub fn has_severity(&self, severity: Severity) -> bool {
        self.iter().any(|diag| diag.severity > severity)
    }

    pub fn has_errors(&self) -> bool { self.has_severity(Severity::Error) }

    pub fn has_warnings(&self) -> bool { self.has_severity(Severity::Warning) }
}

impl FromIterator<Diagnostic> for Diagnostics {
    fn from_iter<T: IntoIterator<Item = Diagnostic>>(iter: T) -> Self {
        Diagnostics(iter.into_iter().collect())
    }
}

impl Extend<Diagnostic> for Diagnostics {
    fn extend<T: IntoIterator<Item = Diagnostic>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl IntoIterator for Diagnostics {
    type IntoIter = <Vector<Diagnostic> as IntoIterator>::IntoIter;
    type Item = Diagnostic;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a Diagnostics {
    type IntoIter = <&'a Vector<Diagnostic> as IntoIterator>::IntoIter;
    type Item = &'a Diagnostic;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

/// Something that can be used to create a [`Diagnostic`].
pub trait AsDiagnostic: std::error::Error {
    fn meta() -> DiagnosticMetadata;

    /// Create a [`Diagnostic`] from this value.
    ///
    /// For convenience, this method has a default implementation which creates
    /// a [`Severity::Error`] diagnostic with the metadata from
    /// [`AsDiagnostic::meta()`].
    fn as_diagnostic(&self) -> Diagnostic {
        Diagnostic::from_impl(Severity::Error, self)
    }
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
    pub meta: Option<DiagnosticMetadata>,
}

impl Diagnostic {
    pub fn new(severity: Severity, message: impl Into<Text>) -> Self {
        Self {
            severity,
            message: message.into(),
            primary_label: None,
            labels: Vector::new(),
            help: None,
            meta: None,
        }
    }

    pub fn from_impl<T>(severity: Severity, diag: &T) -> Self
    where
        T: AsDiagnostic + ?Sized,
    {
        Diagnostic::new(severity, diag.to_string()).with_meta(T::meta())
    }

    pub fn with_primary_label(self, primary_label: Label) -> Self {
        Diagnostic {
            primary_label: Some(primary_label),
            ..self
        }
    }

    pub fn with_label(mut self, label: Label) -> Self {
        self.labels.push_back(label);
        self
    }

    pub fn with_help(self, help: Text) -> Self {
        Diagnostic {
            help: Some(help),
            ..self
        }
    }

    pub fn with_meta(self, meta: DiagnosticMetadata) -> Self {
        Diagnostic {
            meta: Some(meta),
            ..self
        }
    }
}

impl From<Diagnostic> for Diagnostics {
    fn from(d: Diagnostic) -> Self { Diagnostics::one(d) }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Label {
    /// A number that refers to the item being labeled.
    pub target: Id,
    pub message: Option<Text>,
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
#[serde(rename_all = "kebab-case", tag = "type", content = "index")]
pub enum Id {
    NodeId(crate::lowering::NodeId),
    ArgumentId(crate::lowering::ArgumentId),
    ResourceId(crate::lowering::ResourceId),
    PortId(crate::lowering::ResourceId),
}
