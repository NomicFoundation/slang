use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function declared in an interface has an
/// implementation body.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InterfaceFunctionCannotBeImplemented;

impl DiagnosticExtensions for InterfaceFunctionCannotBeImplemented {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/interface-function-cannot-be-implemented"
    }

    fn message(&self) -> String {
        "Functions in interfaces cannot have an implementation.".to_string()
    }
}
