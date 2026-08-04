use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted for `pragma experimental SMTChecker`, which Slang does
/// not support (it was deprecated in Solidity `0.8.4`; the model checker is
/// enabled through compiler settings instead).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnsupportedExperimentalSmtChecker;

impl DiagnosticExtensions for UnsupportedExperimentalSmtChecker {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/unsupported-experimental-smt-checker"
    }

    fn message(&self) -> String {
        "The 'SMTChecker' experimental feature is not supported.".to_string()
    }
}
