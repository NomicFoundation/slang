use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a binary operator in a compile-time constant
/// expression that has no result type for its operand types (e.g. a bitwise
/// operator on a fractional value).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IncompatibleConstantOperator {
    /// The operator symbol, e.g. `/`.
    pub operator: String,
    /// Display name of the left operand's type, e.g. `int_const -7`.
    pub left_type: String,
    /// Display name of the right operand's type, e.g. `uint256`.
    pub right_type: String,
}

impl DiagnosticExtensions for IncompatibleConstantOperator {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/incompatible-constant-operator"
    }

    fn message(&self) -> String {
        format!(
            "Operator {} not compatible with types {} and {}",
            self.operator, self.left_type, self.right_type,
        )
    }
}
