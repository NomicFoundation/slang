use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at the constant where the cycle detection gave up on a
/// dependency path longer than its depth limit.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CyclicDependencyValidatorExhausted;

impl DiagnosticExtensions for CyclicDependencyValidatorExhausted {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/cyclic-dependency-validator-exhausted"
    }

    fn message(&self) -> String {
        "Variable definition exhausting cyclic dependency validator.".to_owned()
    }
}
