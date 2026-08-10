use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when two events visible under the same name declare
/// parameter lists that an external call cannot tell apart, so an `emit`
/// naming them could never be disambiguated.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicateEventDefinition;

impl DiagnosticExtensions for DuplicateEventDefinition {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/duplicate-event-definition"
    }

    fn message(&self) -> String {
        "Event with same name and parameter types defined twice.".to_owned()
    }
}
