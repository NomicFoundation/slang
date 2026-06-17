use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an array length expression evaluates to zero.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ArrayLengthZero;

impl DiagnosticExtensions for ArrayLengthZero {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/array-length-zero"
    }

    fn message(&self) -> String {
        "Array with zero length specified.".to_owned()
    }
}
