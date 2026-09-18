use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::kinds::semantic::OverridableKind;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function or modifier without a body overrides
/// one that has an implementation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnimplementedOverrideOfImplemented {
    /// The kind of the overriding member.
    pub kind: OverridableKind,
}

impl DiagnosticExtensions for UnimplementedOverrideOfImplemented {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/unimplemented-override-of-implemented"
    }

    fn message(&self) -> String {
        format!(
            "Overriding an implemented {} with an unimplemented {} is not allowed.",
            self.kind.as_str(),
            self.kind.as_str()
        )
    }
}
