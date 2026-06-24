use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a fallback function declares parameters and/or
/// return values that do not match one of the two accepted signatures:
/// `fallback()` or `fallback(bytes calldata) returns (bytes memory)`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FallbackFunctionSignature;

impl DiagnosticExtensions for FallbackFunctionSignature {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/fallback-function-signature"
    }

    fn message(&self) -> String {
        "Fallback function either has to have the signature \"fallback()\" or \"fallback(bytes calldata) returns (bytes memory)\".".to_string()
    }
}
