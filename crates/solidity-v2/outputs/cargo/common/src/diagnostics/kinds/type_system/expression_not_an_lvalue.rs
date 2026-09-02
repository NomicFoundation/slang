use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an expression is written to (assigned to, deleted,
/// incremented or decremented) but it does not denote a storage location that
/// can be written: it names a function, or is a computed value such as a call
/// result, a literal or an operator application.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExpressionNotAnLValue;

impl DiagnosticExtensions for ExpressionNotAnLValue {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/expression-not-an-lvalue"
    }

    fn message(&self) -> String {
        "Expression has to be an lvalue.".to_owned()
    }
}
