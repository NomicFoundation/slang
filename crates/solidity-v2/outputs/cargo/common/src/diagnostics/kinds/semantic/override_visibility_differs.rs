use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an overriding function changes the visibility of
/// the overridden one. Only `external` to `public` is allowed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverrideVisibilityDiffers;

impl DiagnosticExtensions for OverrideVisibilityDiffers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/override-visibility-differs"
    }

    fn message(&self) -> String {
        "Overriding function visibility differs.".to_string()
    }
}
