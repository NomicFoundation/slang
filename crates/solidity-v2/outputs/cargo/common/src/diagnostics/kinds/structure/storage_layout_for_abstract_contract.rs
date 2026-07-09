use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an abstract contract declares a storage layout
/// (`layout at`) specifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StorageLayoutForAbstractContract;

impl DiagnosticExtensions for StorageLayoutForAbstractContract {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/storage-layout-for-abstract-contract"
    }

    fn message(&self) -> String {
        "Storage layout cannot be specified for abstract contracts.".to_string()
    }
}
