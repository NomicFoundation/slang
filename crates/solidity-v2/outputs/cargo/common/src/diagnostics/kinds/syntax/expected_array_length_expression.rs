use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a range/slice index access (`[start:end]`) is used
/// where an array length expression is expected.
///
/// Mirrors solc's `ParserError 5464` ("Expected array length expression.").
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExpectedArrayLengthExpression;

impl DiagnosticExtensions for ExpectedArrayLengthExpression {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/expected-array-length-expression"
    }

    fn message(&self) -> String {
        "Expected array length expression.".to_string()
    }
}
