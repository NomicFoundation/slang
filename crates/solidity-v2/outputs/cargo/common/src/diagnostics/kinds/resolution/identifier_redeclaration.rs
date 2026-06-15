use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an identifier is declared more than once in a scope
/// where re-declaration is not allowed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IdentifierRedeclaration;

impl DiagnosticExtensions for IdentifierRedeclaration {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/identifier-redeclaration"
    }

    fn message(&self) -> String {
        "Identifier already declared.".to_owned()
    }
}
