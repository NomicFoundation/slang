use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a Yul `leave` keyword appears outside of a function.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulLeaveOutsideFunction;

impl DiagnosticExtensions for YulLeaveOutsideFunction {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/yul-leave-outside-function"
    }

    fn message(&self) -> String {
        "Keyword 'leave' can only be used inside a function.".to_string()
    }
}
