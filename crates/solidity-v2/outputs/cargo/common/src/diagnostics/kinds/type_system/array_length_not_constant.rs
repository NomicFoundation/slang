use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an array length expression is not a compile-time
/// constant integer (for example a non-constant variable, or an expression the
/// constant evaluator cannot fold).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ArrayLengthNotConstant;

impl DiagnosticExtensions for ArrayLengthNotConstant {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/array-length-not-constant"
    }

    fn message(&self) -> String {
        "Invalid array length, expected integer literal or constant expression.".to_owned()
    }
}
