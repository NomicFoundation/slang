use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a free (file-level) function is marked `payable`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FreeFunctionPayable;

impl DiagnosticExtensions for FreeFunctionPayable {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/free-function-payable"
    }

    fn message(&self) -> String {
        "Free functions cannot be \"payable\".".to_string()
    }
}
