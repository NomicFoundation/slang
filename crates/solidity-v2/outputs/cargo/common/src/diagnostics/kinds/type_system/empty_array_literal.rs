use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an array literal has no elements, eg. `[]`, so there
/// is no element to deduce its type from.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EmptyArrayLiteral;

impl DiagnosticExtensions for EmptyArrayLiteral {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/empty-array-literal"
    }

    fn message(&self) -> String {
        "Unable to deduce the type of an empty array literal.".to_owned()
    }
}
