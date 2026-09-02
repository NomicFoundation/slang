use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly assignment whose target is a declaration
/// that is not a variable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulAssignmentToNonVariable;

impl DiagnosticExtensions for YulAssignmentToNonVariable {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-assignment-to-non-variable"
    }

    fn message(&self) -> String {
        "Only local variables can be assigned to in inline assembly.".to_owned()
    }
}
