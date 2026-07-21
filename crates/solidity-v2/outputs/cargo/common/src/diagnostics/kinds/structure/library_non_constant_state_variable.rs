use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a library declares a state variable that is not
/// `constant`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LibraryNonConstantStateVariable;

impl DiagnosticExtensions for LibraryNonConstantStateVariable {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/library-non-constant-state-variable"
    }

    fn message(&self) -> String {
        "Library cannot have non-constant state variables".to_string()
    }
}
