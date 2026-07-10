use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a constructor is defined in a library.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstructorInLibrary;

impl DiagnosticExtensions for ConstructorInLibrary {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/constructor-in-library"
    }

    fn message(&self) -> String {
        "Constructor cannot be defined in libraries.".to_string()
    }
}
