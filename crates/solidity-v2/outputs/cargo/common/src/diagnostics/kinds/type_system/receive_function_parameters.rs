use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a receive function declares parameters.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReceiveFunctionParameters;

impl DiagnosticExtensions for ReceiveFunctionParameters {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/receive-function-parameters"
    }

    fn message(&self) -> String {
        "Receive ether function cannot take parameters.".to_string()
    }
}
