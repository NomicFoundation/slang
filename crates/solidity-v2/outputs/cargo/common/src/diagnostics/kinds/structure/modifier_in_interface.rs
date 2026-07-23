use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a modifier is defined or declared in an interface.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ModifierInInterface;

impl DiagnosticExtensions for ModifierInInterface {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/modifier-in-interface"
    }

    fn message(&self) -> String {
        "Modifiers cannot be defined or declared in interfaces.".to_string()
    }
}
