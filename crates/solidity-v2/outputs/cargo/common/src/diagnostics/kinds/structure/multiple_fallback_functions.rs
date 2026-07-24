use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a contract defines more than one fallback function.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MultipleFallbackFunctions;

impl DiagnosticExtensions for MultipleFallbackFunctions {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/multiple-fallback-functions"
    }

    fn message(&self) -> String {
        "Only one fallback function is allowed.".to_string()
    }
}
