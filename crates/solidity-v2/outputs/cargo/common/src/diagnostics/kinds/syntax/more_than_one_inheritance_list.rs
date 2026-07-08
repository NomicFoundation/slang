use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a contract declares more than one inheritance
/// (`is`) list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MoreThanOneInheritanceList;

impl DiagnosticExtensions for MoreThanOneInheritanceList {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/more-than-one-inheritance-list"
    }

    fn message(&self) -> String {
        "More than one inheritance list.".to_string()
    }
}
