use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an expression that names a type (eg. `uint`, or an
/// enum or contract name), a module (an import alias) or a function
/// declaration reached through a contract type name (eg. `C.f`) is used where
/// a value is required, eg. as a branch of a conditional: `c ? E : E.A`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExpressionNotAValue;

impl DiagnosticExtensions for ExpressionNotAValue {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/expression-not-a-value"
    }

    fn message(&self) -> String {
        "This expression denotes a type or declaration, not a value.".to_owned()
    }
}
