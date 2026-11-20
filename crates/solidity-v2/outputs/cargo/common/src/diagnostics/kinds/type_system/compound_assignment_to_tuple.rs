use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a compound assignment (eg. `+=`) has a tuple on its
/// left hand side. Only a plain assignment destructures a tuple.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompoundAssignmentToTuple;

impl DiagnosticExtensions for CompoundAssignmentToTuple {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/compound-assignment-to-tuple"
    }

    fn message(&self) -> String {
        "Compound assignment is not allowed for tuple types.".to_owned()
    }
}
