use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function has the same name as its enclosing
/// contract, library, or interface.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FunctionNameMatchesContainer;

impl DiagnosticExtensions for FunctionNameMatchesContainer {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/function-name-matches-container"
    }

    fn message(&self) -> String {
        "Functions are not allowed to have the same name as the enclosing container.".to_string()
    }
}
