use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function declared in a library is marked `payable`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LibraryPayableFunction;

impl DiagnosticExtensions for LibraryPayableFunction {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/library-payable-function"
    }

    fn message(&self) -> String {
        "Library functions cannot be \"payable\".".to_string()
    }
}
