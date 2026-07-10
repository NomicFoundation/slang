use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a `break` statement appears outside of any loop.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BreakOutsideLoop;

impl DiagnosticExtensions for BreakOutsideLoop {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/break-outside-loop"
    }

    fn message(&self) -> String {
        "\"break\" has to be in a \"for\" or \"while\" loop.".to_string()
    }
}
