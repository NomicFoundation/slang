use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a non-free, non-constructor function does not
/// specify a visibility modifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MissingFunctionVisibility {
    /// The visibility keyword suggested for this function.
    pub suggested_visibility: String,
}

impl DiagnosticExtensions for MissingFunctionVisibility {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/missing-function-visibility"
    }

    fn message(&self) -> String {
        format!(
            "No visibility specified. Did you intend to add \"{}\"?",
            self.suggested_visibility
        )
    }
}
