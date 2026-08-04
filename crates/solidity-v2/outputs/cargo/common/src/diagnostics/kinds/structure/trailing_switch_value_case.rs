use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a Yul `switch` statement declares a 'value' case after
/// its 'default' case.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TrailingSwitchValueCase;

impl DiagnosticExtensions for TrailingSwitchValueCase {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/trailing-switch-value-case"
    }

    fn message(&self) -> String {
        "A switch statement cannot have a value case after the default case.".to_string()
    }
}
