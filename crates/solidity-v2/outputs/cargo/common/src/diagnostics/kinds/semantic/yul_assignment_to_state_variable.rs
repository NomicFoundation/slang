use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly assignment to the `.slot` or `.offset`
/// of a state variable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulAssignmentToStateVariable;

impl DiagnosticExtensions for YulAssignmentToStateVariable {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-assignment-to-state-variable"
    }

    fn message(&self) -> String {
        "State variables cannot be assigned to in inline assembly. Use \"sstore()\" or \"tstore()\" instead.".to_owned()
    }
}
