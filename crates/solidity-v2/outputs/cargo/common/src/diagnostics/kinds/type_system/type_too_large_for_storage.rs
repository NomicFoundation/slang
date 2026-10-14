use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a state variable's type, or a type it stores
/// through an array, mapping or struct member, extends past the end of
/// storage.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypeTooLargeForStorage;

impl DiagnosticExtensions for TypeTooLargeForStorage {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/type-too-large-for-storage"
    }

    fn message(&self) -> String {
        "Type too large for storage.".to_owned()
    }
}
