use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a `constant` is declared without an initializer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UninitializedConstant;

impl DiagnosticExtensions for UninitializedConstant {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/uninitialized-constant"
    }

    fn message(&self) -> String {
        "Uninitialized \"constant\" variable.".to_string()
    }
}
