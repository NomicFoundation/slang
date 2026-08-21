use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the two branches of a conditional have no common
/// type. Mirrors solc's `TypeError 1080`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IncompatibleConditionalBranches;

impl DiagnosticExtensions for IncompatibleConditionalBranches {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/incompatible-conditional-branches"
    }

    fn message(&self) -> String {
        "The branches of a conditional have no common type.".to_owned()
    }
}
