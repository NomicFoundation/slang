use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a free (file-level) function has no implementation
/// body.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FreeFunctionWithoutBody;

impl DiagnosticExtensions for FreeFunctionWithoutBody {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/free-function-without-body"
    }

    fn message(&self) -> String {
        "Free functions must be implemented.".to_string()
    }
}
