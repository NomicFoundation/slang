use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a receive function does not declare explicit `external` visibility and `payable` modifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidReceiveAttributes;

impl DiagnosticExtensions for InvalidReceiveAttributes {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/invalid-receive-attributes"
    }

    fn message(&self) -> String {
        "Receive function must have explicit `external` visibility and `payable` modifier."
            .to_string()
    }
}
