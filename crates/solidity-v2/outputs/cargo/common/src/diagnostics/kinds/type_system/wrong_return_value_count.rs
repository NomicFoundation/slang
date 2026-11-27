use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A `return` statement gives a different number of values than its function
/// declares, eg. `return (1, 2);` in a function that `returns (uint)`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WrongReturnValueCount {
    pub given: usize,
    pub expected: usize,
}

impl DiagnosticExtensions for WrongReturnValueCount {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/wrong-return-value-count"
    }

    fn message(&self) -> String {
        let s = if self.given == 1 { "" } else { "s" };
        format!(
            "Wrong return value count: {} value{s} given but expected {}.",
            self.given, self.expected
        )
    }
}
