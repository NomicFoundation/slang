use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a `string` (or a slice of one) is indexed, eg.
/// `s[0]`, since strings have no per-character element type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StringIndexAccess;

impl DiagnosticExtensions for StringIndexAccess {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/string-index-access"
    }

    fn message(&self) -> String {
        "Index access is not possible on a string.".to_owned()
    }
}
