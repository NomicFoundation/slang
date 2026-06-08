use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a base in an inheritance list resolves to something
/// that is not a contract or interface (for example a library, struct, or free
/// function), and therefore cannot be used as a base.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidBase;

impl DiagnosticExtensions for InvalidBase {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/invalid-base"
    }

    fn message(&self) -> String {
        "Only a contract or interface can be used as a base.".to_owned()
    }
}
