use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an array length expression evaluates to a value
/// larger than `2**256 - 1`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ArrayLengthTooLarge;

impl DiagnosticExtensions for ArrayLengthTooLarge {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/array-length-too-large"
    }

    fn message(&self) -> String {
        "Array length too large, maximum is 2**256 - 1.".to_owned()
    }
}
