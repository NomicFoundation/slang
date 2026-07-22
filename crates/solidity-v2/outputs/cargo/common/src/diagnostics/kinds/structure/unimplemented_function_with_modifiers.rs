use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function without an implementation body (e.g. an
/// abstract function in a contract) has one or more modifier invocations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnimplementedFunctionWithModifiers;

impl DiagnosticExtensions for UnimplementedFunctionWithModifiers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/unimplemented-function-with-modifiers"
    }

    fn message(&self) -> String {
        "Function without an implementation body cannot have modifiers".to_string()
    }
}
