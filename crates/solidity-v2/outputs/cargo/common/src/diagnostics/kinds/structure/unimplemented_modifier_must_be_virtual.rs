use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a modifier has no implementation body and is not
/// marked `virtual`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnimplementedModifierMustBeVirtual;

impl DiagnosticExtensions for UnimplementedModifierMustBeVirtual {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/unimplemented-modifier-must-be-virtual"
    }

    fn message(&self) -> String {
        "Modifiers without implementation must be marked virtual.".to_string()
    }
}
