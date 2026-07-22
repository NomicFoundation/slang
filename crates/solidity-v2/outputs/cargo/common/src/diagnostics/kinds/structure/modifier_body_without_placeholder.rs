use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an implemented modifier's body does not contain a
/// placeholder statement (`_`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ModifierBodyWithoutPlaceholder;

impl DiagnosticExtensions for ModifierBodyWithoutPlaceholder {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/modifier-body-without-placeholder"
    }

    fn message(&self) -> String {
        "Modifier body does not contain '_'.".to_string()
    }
}
