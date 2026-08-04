use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted for `pragma experimental solidity`, the experimental
/// Solidity language front-end, which Slang does not support.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnsupportedExperimentalSolidity;

impl DiagnosticExtensions for UnsupportedExperimentalSolidity {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/unsupported-experimental-solidity"
    }

    fn message(&self) -> String {
        "The 'solidity' experimental feature is not supported.".to_string()
    }
}
