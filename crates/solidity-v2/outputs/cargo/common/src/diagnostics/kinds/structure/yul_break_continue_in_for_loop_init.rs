use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a Yul `break` or `continue` keyword appears in the
/// init block of a for-loop.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulBreakContinueInForLoopInit {
    /// The keyword that was misplaced (`break` or `continue`).
    pub keyword: String,
}

impl DiagnosticExtensions for YulBreakContinueInForLoopInit {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/yul-break-continue-in-for-loop-init"
    }

    fn message(&self) -> String {
        format!(
            "Keyword '{}' in for-loop init block is not allowed.",
            self.keyword
        )
    }
}
