use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when compile-time evaluation of a constant expression
/// produces a value that does not fit the result type of an arithmetic or
/// bitwise operator.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstantArithmeticError;

impl DiagnosticExtensions for ConstantArithmeticError {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/constant-arithmetic-error"
    }

    fn message(&self) -> String {
        "Arithmetic error when computing constant value.".to_owned()
    }
}
