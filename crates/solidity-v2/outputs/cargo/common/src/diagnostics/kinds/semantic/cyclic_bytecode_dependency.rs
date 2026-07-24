use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at the `new` expression or `type(...).creationCode` /
/// `type(...).runtimeCode` access through which a contract reaches a cycle in
/// the contract bytecode dependency graph.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CyclicBytecodeDependency;

impl DiagnosticExtensions for CyclicBytecodeDependency {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/cyclic-bytecode-dependency"
    }

    fn message(&self) -> String {
        "Circular contract bytecode dependency.".to_owned()
    }
}
