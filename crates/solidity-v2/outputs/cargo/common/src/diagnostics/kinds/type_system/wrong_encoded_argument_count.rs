use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// The arguments `abi.encodeCall` encodes a call with are a different number
/// than the encoded function declares parameters, eg.
/// `abi.encodeCall(this.f, (1, 2))` for `function f(uint)`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WrongEncodedArgumentCount {
    pub given: usize,
    pub expected: usize,
}

impl DiagnosticExtensions for WrongEncodedArgumentCount {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/wrong-encoded-argument-count"
    }

    fn message(&self) -> String {
        let s = if self.given == 1 { "" } else { "s" };
        format!(
            "Wrong encoded argument count: {} argument{s} given but the encoded function expects {}.",
            self.given, self.expected
        )
    }
}
