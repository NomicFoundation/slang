use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a library declares a receive function.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LibraryReceiveFunction;

impl DiagnosticExtensions for LibraryReceiveFunction {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/library-receive-function"
    }

    fn message(&self) -> String {
        "Libraries cannot have receive ether functions.".to_string()
    }
}
