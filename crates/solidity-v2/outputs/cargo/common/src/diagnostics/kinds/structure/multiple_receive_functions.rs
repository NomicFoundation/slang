use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a contract defines more than one receive function.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MultipleReceiveFunctions;

impl DiagnosticExtensions for MultipleReceiveFunctions {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/multiple-receive-functions"
    }

    fn message(&self) -> String {
        "Only one receive function is allowed.".to_string()
    }
}
