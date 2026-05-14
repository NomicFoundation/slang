use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a fallback function does not declare explicit `external` visibility.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidFallbackVisibility;

impl DiagnosticExtensions for InvalidFallbackVisibility {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/invalid-fallback-visibility"
    }

    fn message(&self) -> String {
        "Fallback function must have explicit `external` visibility.".to_string()
    }
}
