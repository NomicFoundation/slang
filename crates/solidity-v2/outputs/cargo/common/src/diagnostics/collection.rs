use std::ops::Range;

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::kinds::DiagnosticKind;

/// An ordered collection of [`Diagnostic`] values produced during a single
/// parse or compilation run.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiagnosticCollection {
    contents: Vec<Diagnostic>,
}

impl DiagnosticCollection {
    /// Constructs a new diagnostic from its parts and appends it to the collection.
    pub fn push(
        &mut self,
        file_id: String,
        text_range: Range<usize>,
        kind: impl Into<DiagnosticKind>,
    ) {
        self.contents
            .push(Diagnostic::new(file_id, text_range, kind));
    }

    /// Returns `true` if the collection contains no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    /// Returns an iterator over the diagnostics in the collection.
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.contents.iter()
    }
}

impl<'a> IntoIterator for &'a DiagnosticCollection {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.iter()
    }
}

impl IntoIterator for DiagnosticCollection {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

impl Extend<Diagnostic> for DiagnosticCollection {
    fn extend<I: IntoIterator<Item = Diagnostic>>(&mut self, iter: I) {
        self.contents.extend(iter);
    }
}
