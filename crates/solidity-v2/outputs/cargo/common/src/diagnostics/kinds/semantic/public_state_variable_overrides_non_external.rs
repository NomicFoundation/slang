use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a `public` state variable overrides a function
/// that is not `external`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublicStateVariableOverridesNonExternal;

impl DiagnosticExtensions for PublicStateVariableOverridesNonExternal {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/public-state-variable-overrides-non-external"
    }

    fn message(&self) -> String {
        "Public state variables can only override functions with external visibility.".to_string()
    }
}
