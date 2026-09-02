use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the two branches of a conditional expression have
/// no type both can convert to, eg. `c ? 1 : -1` (`uint8` against `int8`) or
/// `c ? bytes32(0) : 0`.
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
        "The true and false branches of this conditional expression have no common type.".to_owned()
    }
}
