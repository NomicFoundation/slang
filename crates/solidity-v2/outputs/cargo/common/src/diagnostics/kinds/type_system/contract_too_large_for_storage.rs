use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a contract's state variables, including those
/// inherited from its bases, extend past the end of storage.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractTooLargeForStorage;

impl DiagnosticExtensions for ContractTooLargeForStorage {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/contract-too-large-for-storage"
    }

    fn message(&self) -> String {
        "Contract requires too much storage.".to_owned()
    }
}
