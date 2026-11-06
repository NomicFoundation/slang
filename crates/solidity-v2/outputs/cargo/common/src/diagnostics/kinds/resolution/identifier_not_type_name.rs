use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A type name resolved to a declaration that is not a type, like a variable, a
/// function or an event.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IdentifierNotTypeName;

impl DiagnosticExtensions for IdentifierNotTypeName {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/identifier-not-type-name"
    }

    fn message(&self) -> String {
        "Identifier does not name a type.".to_owned()
    }
}
