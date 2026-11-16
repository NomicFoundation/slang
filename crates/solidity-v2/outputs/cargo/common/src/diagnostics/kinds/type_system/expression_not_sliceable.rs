use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the operand of a slice is not a dynamically-sized
/// calldata array with fixed-size elements (or a slice of one), eg. `m[1:]` on a
/// mapping, `uint[1:]` on a type name, `a[1:]` on a `memory` or fixed-size
/// array, or on a `bytes[] calldata`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExpressionNotSliceable;

impl DiagnosticExtensions for ExpressionNotSliceable {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/expression-not-sliceable"
    }

    fn message(&self) -> String {
        "Only dynamically-sized calldata arrays with fixed-size elements can be sliced.".to_owned()
    }
}
