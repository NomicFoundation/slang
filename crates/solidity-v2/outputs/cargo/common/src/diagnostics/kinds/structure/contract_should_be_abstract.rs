use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a non-`abstract` contract has one or more
/// unimplemented functions or modifiers (declared in the contract itself or
/// inherited from a base contract or interface). Such a contract cannot be
/// deployed and must be marked `abstract`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractShouldBeAbstract {
    /// The name of the contract that should be marked `abstract`.
    pub name: String,
}

impl DiagnosticExtensions for ContractShouldBeAbstract {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/contract-should-be-abstract"
    }

    fn message(&self) -> String {
        format!("Contract \"{}\" should be marked as abstract.", self.name)
    }
}
