use serde::Serialize;
use slang_solidity_v2_common::diagnostics::{DiagnosticExtensions, DiagnosticSeverity};

use super::CompilationDiagnosticKind;

/// Diagnostic emitted when the compilation pipeline cannot resolve an
/// `import` directive to a concrete file identifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnresolvedImport {
    /// A human-readable description of why resolution failed, typically the
    /// eagerly-formatted error produced by the configured resolver.
    pub reason: String,
}

impl DiagnosticExtensions for UnresolvedImport {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "compilation/unresolved-import"
    }

    fn message(&self) -> String {
        format!("Unresolved import: {}", self.reason)
    }
}

// Probably not needed since we're not pushing this diagnostic into a collection
// of `Diagnostic<CompilationDiagnosticKind>`
impl From<UnresolvedImport> for CompilationDiagnosticKind {
    fn from(d: UnresolvedImport) -> Self {
        Self::UnresolvedImport(d)
    }
}
