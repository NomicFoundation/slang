use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly reference that accesses a storage
/// variable without a suffix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulStorageVariableAccess;

impl DiagnosticExtensions for YulStorageVariableAccess {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-storage-variable-access"
    }

    fn message(&self) -> String {
        "Storage variables cannot be accessed directly in inline assembly. Use the \".slot\" and \".offset\" suffixes.".to_owned()
    }
}
