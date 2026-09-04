use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly suffix that the referenced declaration
/// does not support.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulUnsupportedSuffix;

impl DiagnosticExtensions for YulUnsupportedSuffix {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-unsupported-suffix"
    }

    fn message(&self) -> String {
        "The suffix is not supported by this variable or type.".to_owned()
    }
}
