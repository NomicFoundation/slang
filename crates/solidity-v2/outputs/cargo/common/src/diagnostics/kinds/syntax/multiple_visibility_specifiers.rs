use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a definition carries more than one visibility
/// specifier (e.g. both `public` and `private`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MultipleVisibilitySpecifiers;

impl DiagnosticExtensions for MultipleVisibilitySpecifiers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/multiple-visibility-specifiers"
    }

    fn message(&self) -> String {
        "Only a single visibility specifier can be provided.".to_string()
    }
}

