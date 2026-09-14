use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an overriding function or public state variable
/// returns different types than the function it overrides.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverrideReturnTypesDiffer {
    /// The kind of the overriding member, as it reads in the message
    /// (`function` or `public state variable`).
    pub kind: String,
}

impl DiagnosticExtensions for OverrideReturnTypesDiffer {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/override-return-types-differ"
    }

    fn message(&self) -> String {
        format!("Overriding {} return types differ.", self.kind)
    }
}
