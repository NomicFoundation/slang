use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly assignment to the `.offset` of a
/// storage reference variable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulAssignmentToOffset;

impl DiagnosticExtensions for YulAssignmentToOffset {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-assignment-to-offset"
    }

    fn message(&self) -> String {
        "Only \".slot\" can be assigned to.".to_owned()
    }
}
