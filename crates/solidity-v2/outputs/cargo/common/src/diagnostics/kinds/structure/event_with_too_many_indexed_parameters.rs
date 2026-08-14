use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a non-anonymous event declares more than 3 `indexed` parameters.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EventWithTooManyIndexedParameters;

impl DiagnosticExtensions for EventWithTooManyIndexedParameters {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/event-with-too-many-indexed-parameters"
    }

    fn message(&self) -> String {
        "More than 3 indexed arguments for event.".to_string()
    }
}
