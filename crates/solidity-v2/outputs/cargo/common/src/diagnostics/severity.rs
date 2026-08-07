/// Severity classification for a [`crate::diagnostics::Diagnostic`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticSeverity {
    /// A correctness problem — input cannot be considered valid.
    Error,
    // TODO: when a `Warning` (or other non-`Error`) variant is added, revisit
    // the `diagnostics_output` test runner — it counts only error-severity
    // diagnostics as failures, so solc warnings currently pass as a success.
}
