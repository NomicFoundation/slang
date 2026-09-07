use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an overriding modifier declares different
/// parameter types than the modifier it overrides.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverrideChangesModifierSignature;

impl DiagnosticExtensions for OverrideChangesModifierSignature {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/override-changes-modifier-signature"
    }

    fn message(&self) -> String {
        "Override changes modifier signature.".to_string()
    }
}
