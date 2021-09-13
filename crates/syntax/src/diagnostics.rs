use codespan_reporting::diagnostic::{Diagnostic, Severity};

type FileId = ();

/// A collection of [`Diagnostic`]s.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Diagnostics(Vec<Diagnostic<FileId>>);

impl Diagnostics {
    pub fn new() -> Self { Diagnostics(Vec::new()) }

    pub fn iter(&self) -> impl Iterator<Item = &'_ Diagnostic<FileId>> + '_ {
        self.0.iter()
    }

    /// Get an iterator over all the [`Diagnostic`]s that are at least as severe
    /// as a certain [`Severity`].
    pub fn iter_severity(
        &self,
        severity: Severity,
    ) -> impl Iterator<Item = &'_ Diagnostic<FileId>> + '_ {
        self.iter().filter(move |diag| diag.severity >= severity)
    }

    /// Are there any diagnostics which are at least as severe as a certain
    /// [`Severity`] level?
    pub fn has_severity(&self, severity: Severity) -> bool {
        self.iter_severity(severity).next().is_some()
    }

    /// Does this set of [`Diagnostics`] contain any [`Diagnostic`]s which are
    /// at least as bad as an error?
    pub fn has_errors(&self) -> bool { self.has_severity(Severity::Error) }

    /// Does this set of [`Diagnostics`] contain any [`Diagnostic`]s which are
    /// at least as bad as a warning?
    pub fn has_warnings(&self) -> bool { self.has_severity(Severity::Warning) }

    /// Add a new [`Diagnostic`] to the collection.
    pub fn push(&mut self, diag: Diagnostic<FileId>) { self.0.push(diag); }

    /// Is this collection of [`Diagnostic`]s empty?
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl<'a> IntoIterator for &'a Diagnostics {
    type IntoIter = <&'a Vec<Diagnostic<FileId>> as IntoIterator>::IntoIter;
    type Item = &'a Diagnostic<FileId>;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

impl IntoIterator for Diagnostics {
    type IntoIter = <Vec<Diagnostic<FileId>> as IntoIterator>::IntoIter;
    type Item = Diagnostic<FileId>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl Extend<Diagnostic<FileId>> for Diagnostics {
    fn extend<T: IntoIterator<Item = Diagnostic<FileId>>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}
