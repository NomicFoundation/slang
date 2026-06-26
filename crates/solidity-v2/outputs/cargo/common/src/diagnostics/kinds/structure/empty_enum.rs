use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an enum declares no members.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EmptyEnum;

impl DiagnosticExtensions for EmptyEnum {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/empty-enum"
    }

    fn message(&self) -> String {
        "Enum with no members is not allowed.".to_string()
    }
}
