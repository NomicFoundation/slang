use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a free function specifies a visibility modifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FreeFunctionVisibility;

impl DiagnosticExtensions for FreeFunctionVisibility {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/free-function-visibility"
    }

    fn message(&self) -> String {
        "Free functions cannot have visibility.".to_string()
    }
}
