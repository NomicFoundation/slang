use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a constructor is defined in an interface.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstructorInInterface;

impl DiagnosticExtensions for ConstructorInInterface {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/constructor-in-interface"
    }

    fn message(&self) -> String {
        "Constructor cannot be defined in interfaces.".to_string()
    }
}
