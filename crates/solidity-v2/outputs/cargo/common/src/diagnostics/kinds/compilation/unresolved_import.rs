use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::kinds::compilation::CompilationDiagnosticKind;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the compilation pipeline cannot resolve an
/// `import` directive to a concrete file identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl From<UnresolvedImport> for DiagnosticKind {
    fn from(d: UnresolvedImport) -> Self {
        Self::Compilation(CompilationDiagnosticKind::UnresolvedImport(d))
    }
}
