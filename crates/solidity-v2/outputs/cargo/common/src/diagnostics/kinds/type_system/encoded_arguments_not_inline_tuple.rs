use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// The second argument of `abi.encodeCall` has a tuple type without being a
/// tuple expression, eg. `abi.encodeCall(this.f, g())` where `g` returns two
/// values.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EncodedArgumentsNotInlineTuple;

impl DiagnosticExtensions for EncodedArgumentsNotInlineTuple {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/encoded-arguments-not-inline-tuple"
    }

    fn message(&self) -> String {
        "The arguments to 'abi.encodeCall' must be given as an inline tuple.".to_owned()
    }
}
