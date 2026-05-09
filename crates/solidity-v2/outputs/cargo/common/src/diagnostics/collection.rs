use std::ops::Range;

use super::DiagnosticExtensions;
use crate::diagnostics::diagnostic::Diagnostic;

/// An ordered collection of [`Diagnostic`] values produced during a single
/// parse or compilation run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticCollection<K: DiagnosticExtensions> {
    contents: Vec<Diagnostic<K>>,
}

impl<K: DiagnosticExtensions> Default for DiagnosticCollection<K> {
    fn default() -> Self {
        Self {
            contents: Vec::default(),
        }
    }
}

impl<K: DiagnosticExtensions> DiagnosticCollection<K> {
    /// Constructs a new diagnostic from its parts and appends it to the collection.
    pub fn push(&mut self, file_id: String, text_range: Range<usize>, kind: impl Into<K>) {
        self.contents
            .push(Diagnostic::new(file_id, text_range, kind));
    }

    /// Returns `true` if the collection contains no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    /// Returns an iterator over the diagnostics in the collection.
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic<K>> {
        self.contents.iter()
    }
}

impl<'a, K: DiagnosticExtensions> IntoIterator for &'a DiagnosticCollection<K> {
    type Item = &'a Diagnostic<K>;
    type IntoIter = std::slice::Iter<'a, Diagnostic<K>>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.iter()
    }
}

impl<K: DiagnosticExtensions> IntoIterator for DiagnosticCollection<K> {
    type Item = Diagnostic<K>;
    type IntoIter = std::vec::IntoIter<Diagnostic<K>>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

// impl<K: DiagnosticExtensions> Extend<Diagnostic<K>> for DiagnosticCollection<K> {
//     fn extend<I: IntoIterator<Item = Diagnostic<K>>>(&mut self, iter: I) {
//         self.contents.extend(iter);
//     }
// }

impl<K: DiagnosticExtensions, L: DiagnosticExtensions> Extend<Diagnostic<L>>
    for DiagnosticCollection<K>
where
    K: From<L>,
{
    fn extend<T: IntoIterator<Item = Diagnostic<L>>>(&mut self, iter: T) {
        self.contents.extend(
            iter.into_iter()
                .map(|diag| Diagnostic::from_diagnostic(diag)),
        );
    }
}
