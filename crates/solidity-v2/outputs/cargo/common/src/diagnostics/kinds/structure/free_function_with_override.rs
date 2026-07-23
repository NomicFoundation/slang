use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a free (file-level) function carries an `override`
/// specifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FreeFunctionWithOverride;

impl DiagnosticExtensions for FreeFunctionWithOverride {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/free-function-with-override"
    }

    fn message(&self) -> String {
        "Free functions cannot override.".to_string()
    }
}
