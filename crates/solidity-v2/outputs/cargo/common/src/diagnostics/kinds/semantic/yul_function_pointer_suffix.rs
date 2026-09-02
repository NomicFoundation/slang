use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly suffix on a function pointer that is
/// neither `.selector` nor `.address`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulFunctionPointerSuffix;

impl DiagnosticExtensions for YulFunctionPointerSuffix {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-function-pointer-suffix"
    }

    fn message(&self) -> String {
        "Function pointers only support \".selector\" and \".address\".".to_owned()
    }
}
