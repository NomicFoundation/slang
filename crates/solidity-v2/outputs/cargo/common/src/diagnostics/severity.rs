/// Severity classification for a [`crate::diagnostics::Diagnostic`].
///
/// Variants are declared from most to least severe.
/// See [`crate::diagnostics::DiagnosticCollection::highest_severity()`] for more details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticSeverity {
    /// A correctness problem — input cannot be considered valid.
    Error,
    /// Something worth the user's attention, but doesn't block the compilation pipeline.
    Warning,
}
