use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function is marked both `virtual` and `private`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VirtualPrivateFunction;

impl DiagnosticExtensions for VirtualPrivateFunction {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/virtual-private-function"
    }

    fn message(&self) -> String {
        "\"virtual\" and \"private\" cannot be used together.".to_string()
    }
}
