use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A symbol in a `using {...} for` directive did not resolve to a unique
/// declaration (it is either undeclared or ambiguous).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IdentifierNotFunctionOrNotUnique;

impl DiagnosticExtensions for IdentifierNotFunctionOrNotUnique {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/identifier-not-function-or-not-unique"
    }

    fn message(&self) -> String {
        "Identifier is not a function name or not unique.".to_owned()
    }
}
