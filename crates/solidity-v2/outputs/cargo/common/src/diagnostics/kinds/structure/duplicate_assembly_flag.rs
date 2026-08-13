use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an assembly statement lists the same flag more than
/// once.
///
/// The kind and its code are deliberately general, but `memory-safe` is for now
/// the only flag the language defines.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicateAssemblyFlag;

impl DiagnosticExtensions for DuplicateAssemblyFlag {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/duplicate-assembly-flag"
    }

    fn message(&self) -> String {
        "This assembly flag has already been specified.".to_string()
    }
}
