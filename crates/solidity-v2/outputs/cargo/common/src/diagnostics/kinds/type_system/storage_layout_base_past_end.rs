use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a contract's state variables, laid out from the
/// base slot of its storage layout, extend past the end of storage.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StorageLayoutBasePastEnd;

impl DiagnosticExtensions for StorageLayoutBasePastEnd {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/storage-layout-base-past-end"
    }

    fn message(&self) -> String {
        "Contract extends past the end of storage from this base slot.".to_owned()
    }
}
