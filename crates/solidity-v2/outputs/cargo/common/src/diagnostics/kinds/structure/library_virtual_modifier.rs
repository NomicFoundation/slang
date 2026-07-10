use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a modifier declared in a library is marked `virtual`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LibraryVirtualModifier;

impl DiagnosticExtensions for LibraryVirtualModifier {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/library-virtual-modifier"
    }

    fn message(&self) -> String {
        "Modifiers in a library cannot be 'virtual'.".to_string()
    }
}
