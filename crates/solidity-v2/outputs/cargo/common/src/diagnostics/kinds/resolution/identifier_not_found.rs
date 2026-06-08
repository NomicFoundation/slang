use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an identifier could not be found.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IdentifierNotFound;

impl DiagnosticExtensions for IdentifierNotFound {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/identifier-not-found"
    }

    fn message(&self) -> String {
        "Identifier not found.".to_owned()
    }
}
