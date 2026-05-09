use serde::Serialize;
use slang_solidity_v2_common::diagnostics::{DiagnosticExtensions, DiagnosticSeverity};

use super::CompilationDiagnosticKind;

/// Diagnostic emitted when the compilation pipeline is asked to read a file
/// that cannot be provided by the configured `read_file` callback.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
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

// Probably not needed since we're not pushing this diagnostic into a collection
// of `Diagnostic<CompilationDiagnosticKind>`
impl From<MissingFile> for CompilationDiagnosticKind {
    fn from(d: MissingFile) -> Self {
        Self::MissingFile(d)
    }
}
