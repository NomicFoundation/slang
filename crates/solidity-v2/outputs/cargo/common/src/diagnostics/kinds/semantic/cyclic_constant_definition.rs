use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at a constant's use site when compile-time evaluation
/// exhausts its depth limit, either through a cycle or a too-deep chain.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CyclicConstantDefinition;

impl DiagnosticExtensions for CyclicConstantDefinition {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/cyclic-constant-definition"
    }

    fn message(&self) -> String {
        "Cyclic constant definition (or maximum recursion depth exhausted).".to_owned()
    }
}
