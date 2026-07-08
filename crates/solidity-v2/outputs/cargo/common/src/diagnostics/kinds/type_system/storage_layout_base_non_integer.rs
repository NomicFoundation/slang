use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the base slot expression of a contract's storage
/// layout (`layout at <expr>`) evaluates to a non-integer (e.g. fractional)
/// value.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StorageLayoutBaseNonInteger;

impl DiagnosticExtensions for StorageLayoutBaseNonInteger {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/storage-layout-base-non-integer"
    }

    fn message(&self) -> String {
        "The base slot of the storage layout must evaluate to an integer.".to_owned()
    }
}
