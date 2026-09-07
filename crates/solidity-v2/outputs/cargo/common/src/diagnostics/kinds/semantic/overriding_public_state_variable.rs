use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a member overrides an inherited `public` state
/// variable. Reported on the state variable, which cannot be overridden.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverridingPublicStateVariable;

impl DiagnosticExtensions for OverridingPublicStateVariable {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/overriding-public-state-variable"
    }

    fn message(&self) -> String {
        "Cannot override public state variable.".to_string()
    }
}
