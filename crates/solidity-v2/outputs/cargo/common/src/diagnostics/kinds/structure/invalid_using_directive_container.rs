use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Using directives are only allowed at the file level, or inside contracts and libraries.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidUsingDirectiveContainer;

impl DiagnosticExtensions for InvalidUsingDirectiveContainer {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/invalid-using-directive-container"
    }

    fn message(&self) -> String {
        "Using directives are only allowed at the file level, or inside contracts and libraries."
            .to_string()
    }
}
