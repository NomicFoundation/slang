use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a definition carries more than one `virtual`
/// specifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MultipleVirtualSpecifiers;

impl DiagnosticExtensions for MultipleVirtualSpecifiers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/multiple-virtual-specifiers"
    }

    fn message(&self) -> String {
        "Only a single virtual specifier can be provided.".to_string()
    }
}
