use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a constructor is declared outside of a contract
/// (i.e. inside an interface or library).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstructorNotInContract;

impl DiagnosticExtensions for ConstructorNotInContract {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/constructor-not-in-contract"
    }

    fn message(&self) -> String {
        "Constructors can only be defined for contracts.".to_string()
    }
}
