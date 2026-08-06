use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a Yul `switch` statement declares more than one case
/// with the same value.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicateYulSwitchCase {
    /// The formatted value of the duplicated case.
    pub value: String,
}

impl DiagnosticExtensions for DuplicateYulSwitchCase {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/duplicate-yul-switch-case"
    }

    fn message(&self) -> String {
        format!("Duplicate case '{}' defined.", self.value)
    }
}
