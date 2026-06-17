use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an array length expression evaluates to a negative
/// value.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ArrayLengthNegative;

impl DiagnosticExtensions for ArrayLengthNegative {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/array-length-negative"
    }

    fn message(&self) -> String {
        "Array with negative length specified.".to_owned()
    }
}
