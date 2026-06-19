/// Severity classification for a [`crate::diagnostics::Diagnostic`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticSeverity {
    /// A correctness problem — input cannot be considered valid.
    Error,
}
