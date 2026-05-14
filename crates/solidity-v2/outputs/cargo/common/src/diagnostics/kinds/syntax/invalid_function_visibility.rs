/// Diagnostic emitted when a function does not declare explicit visibility.
use crate::diagnostics::DiagnosticExtensions;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct InvalidFunctionVisibility;

impl DiagnosticExtensions for InvalidFunctionVisibility {
    fn severity(&self) -> crate::diagnostics::severity::DiagnosticSeverity {
        crate::diagnostics::severity::DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/invalid-function-visibility"
    }

    fn message(&self) -> String {
        "Function must have explicit visibility (public, internal, external, or private)."
            .to_string()
    }
}
