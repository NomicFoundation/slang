use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A qualifier in an identifier path names a declaration that has no members
/// to look up, like a variable or a struct.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IdentifierNotNamespace {
    pub name: String,
}

impl DiagnosticExtensions for IdentifierNotNamespace {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/identifier-not-namespace"
    }

    fn message(&self) -> String {
        format!(
            "'{}' is not a contract, interface, library, enum or import alias.",
            self.name
        )
    }
}
