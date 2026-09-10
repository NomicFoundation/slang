use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a member overrides an inherited one without an
/// `override` specifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MissingOverrideSpecifier {
    /// The kind of the overriding member, as it reads in the message
    /// (`function`, `modifier` or `public state variable`).
    pub kind: String,
}

impl DiagnosticExtensions for MissingOverrideSpecifier {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/missing-override-specifier"
    }

    fn message(&self) -> String {
        format!(
            "Overriding {} is missing \"override\" specifier.",
            self.kind
        )
    }
}
