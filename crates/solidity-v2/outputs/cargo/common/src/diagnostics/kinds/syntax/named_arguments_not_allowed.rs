use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when named arguments are used in a position that only
/// accepts positional arguments.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NamedArgumentsNotAllowed;

impl DiagnosticExtensions for NamedArgumentsNotAllowed {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/named-arguments-not-allowed"
    }

    fn message(&self) -> String {
        "Named arguments are not allowed here.".to_string()
    }
}
