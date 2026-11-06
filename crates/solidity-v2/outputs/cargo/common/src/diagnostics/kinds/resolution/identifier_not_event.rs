use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// The target of an `emit` resolved to a declaration that is not an event.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IdentifierNotEvent;

impl DiagnosticExtensions for IdentifierNotEvent {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/identifier-not-event"
    }

    fn message(&self) -> String {
        "Identifier does not name an event.".to_owned()
    }
}
