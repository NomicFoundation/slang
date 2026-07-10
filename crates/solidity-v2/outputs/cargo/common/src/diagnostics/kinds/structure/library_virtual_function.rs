use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function declared in a library is marked `virtual`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LibraryVirtualFunction;

impl DiagnosticExtensions for LibraryVirtualFunction {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/library-virtual-function"
    }

    fn message(&self) -> String {
        "Library functions cannot be 'virtual'.".to_string()
    }
}
