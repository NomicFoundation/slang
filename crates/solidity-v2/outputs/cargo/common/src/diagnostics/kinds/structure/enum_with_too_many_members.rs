use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an enum declares more than 256 members.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EnumWithTooManyMembers;

impl DiagnosticExtensions for EnumWithTooManyMembers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/enum-with-too-many-members"
    }

    fn message(&self) -> String {
        "Enum with more than 256 members is not allowed.".to_string()
    }
}
