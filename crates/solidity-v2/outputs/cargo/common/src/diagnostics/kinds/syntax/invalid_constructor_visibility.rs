use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a constructor declares a visibility other than
/// `public`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidConstructorVisibility;

impl DiagnosticExtensions for InvalidConstructorVisibility {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/invalid-constructor-visibility"
    }

    fn message(&self) -> String {
        "Constructor visibility must be `public` or omitted.".to_string()
    }
}
