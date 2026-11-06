use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// The first argument of `abi.encodeCall` is not an external function, eg.
/// `abi.encodeCall(f, ())` for an internal `f`, or `abi.encodeCall(1, ())`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidEncodedFunction;

impl DiagnosticExtensions for InvalidEncodedFunction {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/invalid-encoded-function"
    }

    fn message(&self) -> String {
        "The first argument to 'abi.encodeCall' must be an external function.".to_owned()
    }
}
