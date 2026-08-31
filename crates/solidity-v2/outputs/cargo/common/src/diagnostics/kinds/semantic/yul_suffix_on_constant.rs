use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly reference that accesses a constant
/// through a suffix like `.slot` or `.offset`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulSuffixOnConstant;

impl DiagnosticExtensions for YulSuffixOnConstant {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-suffix-on-constant"
    }

    fn message(&self) -> String {
        "Suffixes cannot be used on constant variables.".to_owned()
    }
}
