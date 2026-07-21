use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a variable is declared in an interface.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VariableInInterface;

impl DiagnosticExtensions for VariableInInterface {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/variable-in-interface"
    }

    fn message(&self) -> String {
        "Variables cannot be declared in interfaces.".to_string()
    }
}
