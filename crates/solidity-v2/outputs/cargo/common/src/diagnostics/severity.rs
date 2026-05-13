/// Severity classification for a [`crate::diagnostics::Diagnostic`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    /// A correctness problem — input cannot be considered valid.
    Error,
}
