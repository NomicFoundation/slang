use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::kinds::compilation::CompilationDiagnosticKind;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the compilation pipeline is asked to read a file
/// that cannot be provided by the configured `read_file` callback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MissingFile {
    /// A human-readable description of why the file could not be read.
    pub reason: String,
}

impl DiagnosticExtensions for MissingFile {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "compilation/missing-file"
    }

    fn message(&self) -> String {
        format!("Missing file: {}", self.reason)
    }
}

impl From<MissingFile> for DiagnosticKind {
    fn from(d: MissingFile) -> Self {
        Self::Compilation(CompilationDiagnosticKind::MissingFile(d))
    }
}
