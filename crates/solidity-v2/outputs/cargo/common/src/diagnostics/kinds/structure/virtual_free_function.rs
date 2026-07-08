use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a free function is marked `virtual`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VirtualFreeFunction;

impl DiagnosticExtensions for VirtualFreeFunction {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/virtual-free-function"
    }

    fn message(&self) -> String {
        "Free functions cannot be virtual.".to_string()
    }
}
