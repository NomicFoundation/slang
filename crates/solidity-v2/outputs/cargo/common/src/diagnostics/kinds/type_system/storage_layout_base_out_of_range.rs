use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the base slot expression of a contract's storage
/// layout (`layout at <expr>`) evaluates to an integer that is negative or
/// larger than `2**256 - 1`, i.e. outside the range of `uint256`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StorageLayoutBaseOutOfRange {
    /// The decimal representation of the offending value.
    pub value: String,
}

impl DiagnosticExtensions for StorageLayoutBaseOutOfRange {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/storage-layout-base-out-of-range"
    }

    fn message(&self) -> String {
        format!(
            "The base slot of the storage layout evaluates to {}, which is outside the range of type uint256.",
            self.value
        )
    }
}
