use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a struct declares no members.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EmptyStruct;

impl DiagnosticExtensions for EmptyStruct {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/empty-struct"
    }

    fn message(&self) -> String {
        "Defining empty structs is disallowed.".to_string()
    }
}
