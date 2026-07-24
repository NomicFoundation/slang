use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at the contract where the bytecode dependency cycle
/// detection gave up on a dependency path longer than its depth limit.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BytecodeDependencyValidatorExhausted;

impl DiagnosticExtensions for BytecodeDependencyValidatorExhausted {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/bytecode-dependency-validator-exhausted"
    }

    fn message(&self) -> String {
        "Contract dependencies exhausting cyclic dependency validator.".to_owned()
    }
}
