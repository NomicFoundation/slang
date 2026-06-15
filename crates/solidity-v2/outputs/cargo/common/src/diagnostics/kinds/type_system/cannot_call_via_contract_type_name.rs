use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function is called through a contract or interface
/// *type name* (eg. `C.f()`) rather than through an instance. Such a reference
/// is a non-callable declaration — only good for taking its `.selector` — so
/// the call is invalid. Mirrors solc's `TypeError 3419`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CannotCallViaContractTypeName;

impl DiagnosticExtensions for CannotCallViaContractTypeName {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/cannot-call-via-contract-type-name"
    }

    fn message(&self) -> String {
        "Cannot call function via contract type name.".to_owned()
    }
}
