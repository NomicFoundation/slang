/// Severity classification for a [`crate::diagnostics::Diagnostic`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    /// A correctness problem — input cannot be considered valid.
    Error,
}
