use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function that requires an implementation body has
/// none. This covers free (file-level) functions and functions declared in a
/// library, both of which must be implemented.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FunctionMustBeImplemented;

impl DiagnosticExtensions for FunctionMustBeImplemented {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/function-must-be-implemented"
    }

    fn message(&self) -> String {
        "Function must be implemented".to_string()
    }
}
