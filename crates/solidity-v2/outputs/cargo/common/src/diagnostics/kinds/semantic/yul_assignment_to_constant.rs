use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly assignment whose target is a constant.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulAssignmentToConstant;

impl DiagnosticExtensions for YulAssignmentToConstant {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-assignment-to-constant"
    }

    fn message(&self) -> String {
        "Constant variables cannot be assigned to.".to_owned()
    }
}
