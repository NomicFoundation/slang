use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly reference that accesses an external
/// function pointer without a suffix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulExternalFunctionAccess;

impl DiagnosticExtensions for YulExternalFunctionAccess {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-external-function-access"
    }

    fn message(&self) -> String {
        "Variables of external function type cannot be accessed directly in inline assembly."
            .to_owned()
    }
}
