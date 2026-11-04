use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// An argument `abi.encodeCall` encodes a call with does not implicitly
/// convert to the type of its parameter in the encoded function, eg.
/// `abi.encodeCall(this.f, (true))` for `function f(uint)`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidEncodedArgumentType;

impl DiagnosticExtensions for InvalidEncodedArgumentType {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/invalid-encoded-argument-type"
    }

    fn message(&self) -> String {
        "Invalid type for encoded argument: it does not implicitly convert to the type of the encoded function's parameter."
            .to_owned()
    }
}
