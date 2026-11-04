use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A call passes a different number of arguments than the callee declares
/// parameters, eg. `f(1, 2)` for `function f(uint)`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WrongArgumentCount {
    pub given: usize,
    pub expected: usize,
}

impl DiagnosticExtensions for WrongArgumentCount {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/wrong-argument-count"
    }

    fn message(&self) -> String {
        format!(
            "Wrong argument count: {} arguments given but expected {}.",
            self.given, self.expected
        )
    }
}
