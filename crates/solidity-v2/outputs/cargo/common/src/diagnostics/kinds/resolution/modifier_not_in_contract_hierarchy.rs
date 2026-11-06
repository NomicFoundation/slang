use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A modifier invocation names a modifier declared outside the hierarchy of the
/// enclosing contract or library, eg. in an unrelated contract or in a library.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ModifierNotInContractHierarchy;

impl DiagnosticExtensions for ModifierNotInContractHierarchy {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/modifier-not-in-contract-hierarchy"
    }

    fn message(&self) -> String {
        "Only modifiers of the current contract or of its bases can be invoked.".to_owned()
    }
}
