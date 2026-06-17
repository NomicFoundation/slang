use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a contract's or interface's inheritance hierarchy
/// contains a cycle.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CyclicInheritance;

impl DiagnosticExtensions for CyclicInheritance {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/cyclic-inheritance"
    }

    fn message(&self) -> String {
        "Circular inheritance is not allowed.".to_owned()
    }
}
