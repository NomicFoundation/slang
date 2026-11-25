use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A call to a callee that takes any number of arguments after some leading
/// ones passes fewer than those, eg. `abi.encodeWithSelector()`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TooFewArguments {
    pub given: usize,
    pub minimum: usize,
}

impl DiagnosticExtensions for TooFewArguments {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/too-few-arguments"
    }

    fn message(&self) -> String {
        let s = if self.given == 1 { "" } else { "s" };
        format!(
            "Too few arguments: {} argument{s} given but expected at least {}.",
            self.given, self.minimum
        )
    }
}
