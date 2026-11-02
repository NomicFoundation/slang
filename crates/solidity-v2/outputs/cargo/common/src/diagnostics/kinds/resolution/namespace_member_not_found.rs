use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// An identifier path named something its qualifier does not declare, or does
/// not make visible from there.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NamespaceMemberNotFound {
    pub name: String,
    pub namespace: String,
}

impl DiagnosticExtensions for NamespaceMemberNotFound {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/namespace-member-not-found"
    }

    fn message(&self) -> String {
        format!(
            "'{}' not found or not visible in '{}'.",
            self.name, self.namespace
        )
    }
}
