use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a source file selects the legacy ABI coder via a
/// `pragma abicoder v1;` directive. Slang only models ABI coder v2 (the default
/// since Solidity `0.8.0`), so the legacy encoder is not supported.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnsupportedAbicoderV1;

impl DiagnosticExtensions for UnsupportedAbicoderV1 {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/unsupported-abicoder-v1"
    }

    fn message(&self) -> String {
        "ABI coder v1 is not supported.".to_string()
    }
}
