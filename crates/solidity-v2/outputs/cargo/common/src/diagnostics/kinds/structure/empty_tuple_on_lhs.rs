use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an empty tuple appears on the left hand side of an
/// assignment.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EmptyTupleOnLhs;

impl DiagnosticExtensions for EmptyTupleOnLhs {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/empty-tuple-on-lhs"
    }

    fn message(&self) -> String {
        "Empty tuple on the left hand side.".to_string()
    }
}
