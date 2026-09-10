use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an overriding function or modifier changes the
/// visibility of the overridden one. Only `external` to `public` is allowed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverrideVisibilityDiffers {
    /// The kind of the overriding member, as it reads in the message
    /// (`function` or `modifier`).
    pub kind: String,
}

impl DiagnosticExtensions for OverrideVisibilityDiffers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/override-visibility-differs"
    }

    fn message(&self) -> String {
        format!("Overriding {} visibility differs.", self.kind)
    }
}
