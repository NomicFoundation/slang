use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a placeholder statement (`_`) appears inside an
/// `unchecked` block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PlaceholderInUncheckedBlock;

impl DiagnosticExtensions for PlaceholderInUncheckedBlock {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/placeholder-in-unchecked-block"
    }

    fn message(&self) -> String {
        "The placeholder statement \"_\" cannot be used inside an \"unchecked\" block.".to_string()
    }
}
