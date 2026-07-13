use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a constructor in an abstract contract is declared `public`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AbstractContractPublicConstructor;

impl DiagnosticExtensions for AbstractContractPublicConstructor {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/abstract-contract-public-constructor"
    }

    fn message(&self) -> String {
        "Abstract contracts cannot have public constructors. Remove the \"public\" keyword to fix this.".to_string()
    }
}
