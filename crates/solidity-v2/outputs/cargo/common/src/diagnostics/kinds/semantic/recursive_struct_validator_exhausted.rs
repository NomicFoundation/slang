use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at the struct where the cycle detection gave up on a
/// by-value path longer than its depth limit.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RecursiveStructValidatorExhausted;

impl DiagnosticExtensions for RecursiveStructValidatorExhausted {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/recursive-struct-validator-exhausted"
    }

    fn message(&self) -> String {
        "Struct definition exhausts cyclic dependency validator.".to_owned()
    }
}
