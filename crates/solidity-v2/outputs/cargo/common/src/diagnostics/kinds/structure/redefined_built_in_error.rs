use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an error is defined with a name reserved for a
/// built-in error (`Error` or `Panic`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RedefinedBuiltInError;

impl DiagnosticExtensions for RedefinedBuiltInError {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/redefined-built-in-error"
    }

    fn message(&self) -> String {
        "The built-in errors \"Error\" and \"Panic\" cannot be re-defined.".to_string()
    }
}
