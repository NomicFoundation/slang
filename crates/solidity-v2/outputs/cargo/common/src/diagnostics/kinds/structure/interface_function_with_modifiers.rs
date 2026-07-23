use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function declared in an interface has one or more
/// modifier invocations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InterfaceFunctionWithModifiers;

impl DiagnosticExtensions for InterfaceFunctionWithModifiers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/interface-function-with-modifiers"
    }

    fn message(&self) -> String {
        "Functions in interfaces cannot have modifiers.".to_string()
    }
}
