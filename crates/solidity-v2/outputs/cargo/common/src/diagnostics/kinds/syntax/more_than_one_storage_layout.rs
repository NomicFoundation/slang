use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a contract declares more than one storage layout
/// (`layout at`) specifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MoreThanOneStorageLayout;

impl DiagnosticExtensions for MoreThanOneStorageLayout {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/more-than-one-storage-layout"
    }

    fn message(&self) -> String {
        "More than one storage layout definition.".to_string()
    }
}
