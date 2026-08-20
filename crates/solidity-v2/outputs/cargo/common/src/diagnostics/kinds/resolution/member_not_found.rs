use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A member access named something the operand's type does not provide, or
/// provides only in overloads that the call's arguments rule out.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MemberNotFound {
    pub name: String,
}

impl DiagnosticExtensions for MemberNotFound {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/member-not-found"
    }

    fn message(&self) -> String {
        format!(
            "Member '{}' not found or not visible after argument-dependent lookup.",
            self.name
        )
    }
}
