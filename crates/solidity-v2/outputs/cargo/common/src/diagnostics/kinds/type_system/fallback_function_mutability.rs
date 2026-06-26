use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a fallback function is declared with a state
/// mutability other than `payable` or non-payable (ie. `pure` or `view`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FallbackFunctionMutability {
    /// The offending mutability, as written (eg. `"pure"` or `"view"`).
    pub mutability: String,
}

impl DiagnosticExtensions for FallbackFunctionMutability {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/fallback-function-mutability"
    }

    fn message(&self) -> String {
        format!(
            "Fallback function must be payable or non-payable, but is \"{mutability}\".",
            mutability = self.mutability
        )
    }
}
