use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a definition carries more than one `override`
/// specifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MultipleOverrideSpecifiers;

impl DiagnosticExtensions for MultipleOverrideSpecifiers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/multiple-override-specifiers"
    }

    fn message(&self) -> String {
        "Only a single override specifier can be provided.".to_string()
    }
}
