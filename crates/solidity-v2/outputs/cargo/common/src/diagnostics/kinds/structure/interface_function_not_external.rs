use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function declared in an interface is not `external`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InterfaceFunctionNotExternal;

impl DiagnosticExtensions for InterfaceFunctionNotExternal {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/interface-function-not-external"
    }

    fn message(&self) -> String {
        "Functions in interfaces must be declared external.".to_string()
    }
}
