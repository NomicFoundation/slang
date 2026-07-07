use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a `continue` statement appears outside of any loop.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContinueOutsideLoop;

impl DiagnosticExtensions for ContinueOutsideLoop {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/continue-outside-loop"
    }

    fn message(&self) -> String {
        "\"continue\" has to be in a \"for\" or \"while\" loop.".to_string()
    }
}
