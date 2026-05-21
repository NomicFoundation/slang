use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an operator is applied to operand types that don't
/// support the operator (e.g. `bool < bool`, `struct == struct`, `mapping == mapping`,
/// `function < function`, comparisons involving event/error/super references).
///
/// It matches solc error code 2271
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OperatorNotApplicableToTypes {
    pub operator: String,
    pub left_type: String,
    pub right_type: String,
}

impl DiagnosticExtensions for OperatorNotApplicableToTypes {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/operator-not-applicable-to-types"
    }

    fn message(&self) -> String {
        format!(
            "Operator {op} cannot be applied to operands of type \"{left}\" and \"{right}\".",
            op = self.operator,
            left = self.left_type,
            right = self.right_type,
        )
    }
}
