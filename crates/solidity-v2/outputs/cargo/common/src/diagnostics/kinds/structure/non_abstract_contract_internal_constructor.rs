use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a constructor in a non-abstract contract is declared `internal`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NonAbstractContractInternalConstructor;

impl DiagnosticExtensions for NonAbstractContractInternalConstructor {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/non-abstract-contract-internal-constructor"
    }

    fn message(&self) -> String {
        "Non-abstract contracts cannot have \"internal\" constructors. Remove the \"internal\" keyword and make the contract abstract to fix this.".to_string()
    }
}
