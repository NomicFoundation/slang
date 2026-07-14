use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an `internal` or `private` function is marked `payable`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PayableInternalOrPrivateFunction;

impl DiagnosticExtensions for PayableInternalOrPrivateFunction {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/payable-internal-or-private-function"
    }

    fn message(&self) -> String {
        "\"internal\" and \"private\" functions cannot be payable.".to_string()
    }
}
