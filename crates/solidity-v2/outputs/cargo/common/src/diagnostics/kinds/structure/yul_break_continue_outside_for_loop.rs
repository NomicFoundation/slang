use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a Yul `break` or `continue` keyword appears outside
/// of any for-loop body.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulBreakContinueOutsideForLoop {
    /// The keyword that was misplaced (`break` or `continue`).
    pub keyword: String,
}

impl DiagnosticExtensions for YulBreakContinueOutsideForLoop {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/yul-break-continue-outside-for-loop"
    }

    fn message(&self) -> String {
        format!(
            "Keyword '{}' needs to be inside a for-loop body.",
            self.keyword
        )
    }
}
