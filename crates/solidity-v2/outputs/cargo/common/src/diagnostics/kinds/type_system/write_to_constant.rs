use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a `constant` declaration is written to. A constant
/// holds a value, but the value is fixed at compile time and there is nothing
/// to write to at run time.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WriteToConstant;

impl DiagnosticExtensions for WriteToConstant {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/write-to-constant"
    }

    fn message(&self) -> String {
        "Cannot write to a constant variable.".to_owned()
    }
}
