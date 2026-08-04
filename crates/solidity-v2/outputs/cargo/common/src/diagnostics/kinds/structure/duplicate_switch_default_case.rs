use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a Yul `switch` statement declares more than one
/// `default` case.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicateSwitchDefaultCase;

impl DiagnosticExtensions for DuplicateSwitchDefaultCase {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/duplicate-switch-default-case"
    }

    fn message(&self) -> String {
        "A switch statement cannot have more than one default case.".to_string()
    }
}
