use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a contract defines more than one constructor.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MultipleConstructors;

impl DiagnosticExtensions for MultipleConstructors {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/multiple-constructors"
    }

    fn message(&self) -> String {
        "More than one constructor defined.".to_string()
    }
}
