use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a `return` statement in a modifier's body carries a
/// value. A modifier has no return values, so only a bare `return;` is allowed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReturnValueInModifier;

impl DiagnosticExtensions for ReturnValueInModifier {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/return-value-in-modifier"
    }

    fn message(&self) -> String {
        "Modifiers cannot return values.".to_string()
    }
}
