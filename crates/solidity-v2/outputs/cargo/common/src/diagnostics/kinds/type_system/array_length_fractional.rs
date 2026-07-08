use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an array length expression evaluates to a
/// fractional value.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ArrayLengthFractional;

impl DiagnosticExtensions for ArrayLengthFractional {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/array-length-fractional"
    }

    fn message(&self) -> String {
        "Array with fractional length specified.".to_owned()
    }
}
