use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an overriding function or public state variable
/// loosens the state mutability of the function it overrides, or changes a
/// `payable` one at all.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverrideChangesStateMutability {
    /// The kind of the overriding member, as it reads in the message
    /// (`function` or `public state variable`).
    pub kind: String,
    /// The mutability of the overridden function (eg. `"view"`).
    pub from: String,
    /// The mutability of the overriding member (eg. `"nonpayable"`).
    pub to: String,
}

impl DiagnosticExtensions for OverrideChangesStateMutability {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/override-changes-state-mutability"
    }

    fn message(&self) -> String {
        format!(
            "Overriding {} changes state mutability from \"{}\" to \"{}\".",
            self.kind, self.from, self.to
        )
    }
}
