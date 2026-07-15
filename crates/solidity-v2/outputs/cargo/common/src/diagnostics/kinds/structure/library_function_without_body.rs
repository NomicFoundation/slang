use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function declared in a library has no
/// implementation body.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LibraryFunctionWithoutBody;

impl DiagnosticExtensions for LibraryFunctionWithoutBody {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/library-function-without-body"
    }

    fn message(&self) -> String {
        "Library functions must be implemented if declared.".to_string()
    }
}
