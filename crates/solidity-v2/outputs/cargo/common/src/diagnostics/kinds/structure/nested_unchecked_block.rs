use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an `unchecked` block appears inside another
/// `unchecked` block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NestedUncheckedBlock;

impl DiagnosticExtensions for NestedUncheckedBlock {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/nested-unchecked-block"
    }

    fn message(&self) -> String {
        "\"unchecked\" blocks cannot be nested.".to_string()
    }
}
