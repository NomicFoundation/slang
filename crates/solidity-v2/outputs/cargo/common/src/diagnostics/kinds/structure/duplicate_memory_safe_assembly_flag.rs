use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an assembly statement repeats the `"memory-safe"` flag.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicateMemorySafeAssemblyFlag;

impl DiagnosticExtensions for DuplicateMemorySafeAssemblyFlag {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/duplicate-memory-safe-assembly-flag"
    }

    fn message(&self) -> String {
        "Inline assembly already marked as 'memory-safe'.".to_string()
    }
}
