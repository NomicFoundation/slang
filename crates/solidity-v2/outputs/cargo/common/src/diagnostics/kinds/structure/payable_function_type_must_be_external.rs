use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function type is marked `payable` but is not `external`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PayableFunctionTypeMustBeExternal;

impl DiagnosticExtensions for PayableFunctionTypeMustBeExternal {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/payable-function-type-must-be-external"
    }

    fn message(&self) -> String {
        "Only \"external\" function types can be \"payable\".".to_string()
    }
}
