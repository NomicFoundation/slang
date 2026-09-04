use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly reference that accesses a dynamic
/// calldata array without a suffix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulCalldataArrayAccess;

impl DiagnosticExtensions for YulCalldataArrayAccess {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-calldata-array-access"
    }

    fn message(&self) -> String {
        "Call data elements cannot be accessed directly. Use \".offset\" and \".length\" to access the calldata offset and length of this array and then use \"calldatacopy\".".to_owned()
    }
}
