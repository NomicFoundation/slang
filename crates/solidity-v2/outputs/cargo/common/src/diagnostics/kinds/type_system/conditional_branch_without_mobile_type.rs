use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a conditional's branch has no mobile type, so it
/// denotes no value the conditional could yield. Mirrors solc's `TypeError
/// 9717` for the true branch and `TypeError 3703` for the false branch from
/// 0.8.21, and its `TypeError 1080` before that; unlike solc, a module type
/// name has no mobile type either.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConditionalBranchWithoutMobileType;

impl DiagnosticExtensions for ConditionalBranchWithoutMobileType {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/conditional-branch-without-mobile-type"
    }

    fn message(&self) -> String {
        "Invalid mobile type in conditional branch.".to_owned()
    }
}
