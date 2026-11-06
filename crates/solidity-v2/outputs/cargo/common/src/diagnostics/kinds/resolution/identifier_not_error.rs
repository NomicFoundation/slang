use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// The target of a `revert` resolved to a declaration that is not an error.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IdentifierNotError;

impl DiagnosticExtensions for IdentifierNotError {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/identifier-not-error"
    }

    fn message(&self) -> String {
        "Identifier does not name an error.".to_owned()
    }
}
