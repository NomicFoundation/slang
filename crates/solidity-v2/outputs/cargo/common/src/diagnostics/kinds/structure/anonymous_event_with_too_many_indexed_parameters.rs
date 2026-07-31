use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an anonymous event declares more than 4 `indexed` parameters.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AnonymousEventWithTooManyIndexedParameters;

impl DiagnosticExtensions for AnonymousEventWithTooManyIndexedParameters {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/anonymous-event-with-too-many-indexed-parameters"
    }

    fn message(&self) -> String {
        "More than 4 indexed arguments for anonymous event.".to_string()
    }
}
