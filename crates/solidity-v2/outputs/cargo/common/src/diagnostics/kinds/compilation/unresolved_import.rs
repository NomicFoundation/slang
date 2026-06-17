use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

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
        self.reason.clone()
    }
}
