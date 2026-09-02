use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at a `.selector` or `.address` suffix on an internal
/// function pointer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulInternalFunctionPointerSuffix;

impl DiagnosticExtensions for YulInternalFunctionPointerSuffix {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-internal-function-pointer-suffix"
    }

    fn message(&self) -> String {
        "Only external function pointers support \".selector\" and \".address\".".to_owned()
    }
}
