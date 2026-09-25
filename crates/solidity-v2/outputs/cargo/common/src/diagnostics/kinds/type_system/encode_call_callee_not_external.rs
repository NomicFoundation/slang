use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the callee of `abi.encodeCall` is not an externally
/// callable function (eg. an `internal` function, a `public` one reached
/// without `this.`, or an event). Only an externally callable function has a
/// selector, which is what the call is encoded against.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EncodeCallCalleeNotExternal;

impl DiagnosticExtensions for EncodeCallCalleeNotExternal {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/encode-call-callee-not-external"
    }

    fn message(&self) -> String {
        "Expected regular external function type, or external view on public function.".to_owned()
    }
}
