use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a library declares a fallback function.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LibraryFallbackFunction;

impl DiagnosticExtensions for LibraryFallbackFunction {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/library-fallback-function"
    }

    fn message(&self) -> String {
        "Libraries cannot have fallback functions.".to_string()
    }
}
