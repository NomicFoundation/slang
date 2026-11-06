use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A modifier invocation resolved to a declaration that is neither a modifier
/// nor a base of the enclosing contract, whose constructor it would invoke.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IdentifierNotModifierOrBase;

impl DiagnosticExtensions for IdentifierNotModifierOrBase {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/identifier-not-modifier-or-base"
    }

    fn message(&self) -> String {
        "Identifier is neither a modifier nor a base contract.".to_owned()
    }
}
