use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a free (file-level) function has one or more
/// modifier invocations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FreeFunctionWithModifiers;

impl DiagnosticExtensions for FreeFunctionWithModifiers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/free-function-with-modifiers"
    }

    fn message(&self) -> String {
        "Free functions cannot have modifiers.".to_string()
    }
}
