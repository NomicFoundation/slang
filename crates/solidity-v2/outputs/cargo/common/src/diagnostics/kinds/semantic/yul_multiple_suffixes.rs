use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly path with more than one suffix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulMultipleSuffixes;

impl DiagnosticExtensions for YulMultipleSuffixes {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-multiple-suffixes"
    }

    fn message(&self) -> String {
        "Only one suffix is allowed.".to_owned()
    }
}
