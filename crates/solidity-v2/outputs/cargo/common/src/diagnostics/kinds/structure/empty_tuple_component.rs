use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a tuple expression, used in a read position, has a
/// missing (empty) component.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EmptyTupleComponent;

impl DiagnosticExtensions for EmptyTupleComponent {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/empty-tuple-component"
    }

    fn message(&self) -> String {
        "Tuple component cannot be empty.".to_string()
    }
}
