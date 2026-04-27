/// Severity classification for a [`crate::diagnostics::Diagnostic`].
///
/// Explicitly compatible with the [LSP protocol](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#diagnosticSeverity).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    /// A correctness problem — input cannot be considered valid.
    Error,
}
