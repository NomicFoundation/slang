use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at the declaration of every constant from which a
/// cyclic dependency is reachable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CyclicConstantDependency {
    /// The name of the constant being reported.
    pub name: String,
    /// The name of its first dependency on the path to the cycle.
    pub via: String,
}

impl DiagnosticExtensions for CyclicConstantDependency {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/cyclic-constant-dependency"
    }

    fn message(&self) -> String {
        format!(
            "The value of the constant {} has a cyclic dependency via {}.",
            self.name, self.via
        )
    }
}
