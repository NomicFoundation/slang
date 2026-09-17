use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::kinds::semantic::OverridableKind;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a member overrides an inherited function or
/// modifier that is not `virtual`. Reported on the overridden member, where
/// `virtual` is missing.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverridingNonVirtualMember {
    /// The kind of the overridden member.
    pub kind: OverridableKind,
}

impl DiagnosticExtensions for OverridingNonVirtualMember {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/overriding-non-virtual-member"
    }

    fn message(&self) -> String {
        format!(
            "Trying to override non-virtual {}. Did you forget to add 'virtual'?",
            self.kind.as_str()
        )
    }
}
