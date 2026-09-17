use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::kinds::semantic::OverridableKind;
use crate::diagnostics::severity::DiagnosticSeverity;

/// The state mutability of a function or of a state variable's getter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum StateMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

impl StateMutability {
    /// The mutability as it reads in a diagnostic message (e.g. `nonpayable`).
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Pure => "pure",
            Self::View => "view",
            Self::NonPayable => "nonpayable",
            Self::Payable => "payable",
        }
    }
}

/// Diagnostic emitted when an overriding function or public state variable
/// loosens the state mutability of the function it overrides, or changes a
/// `payable` one at all.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverrideChangesStateMutability {
    /// The kind of the overriding member.
    pub kind: OverridableKind,
    /// The mutability of the overridden function.
    pub from: StateMutability,
    /// The mutability of the overriding member.
    pub to: StateMutability,
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
            "Overriding {} changes state mutability from '{}' to '{}'.",
            self.kind.as_str(),
            self.from.as_str(),
            self.to.as_str()
        )
    }
}
