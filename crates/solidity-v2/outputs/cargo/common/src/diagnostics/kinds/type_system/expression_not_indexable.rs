use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the operand of an index access is neither an array,
/// a mapping, nor a type name, eg. `x[0]` on an integer, a contract or a
/// function.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExpressionNotIndexable;

impl DiagnosticExtensions for ExpressionNotIndexable {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/expression-not-indexable"
    }

    fn message(&self) -> String {
        "This expression is not indexable.".to_owned()
    }
}
