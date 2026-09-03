use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a number literal used as a value (eg. as a branch
/// of a conditional) is too large for any EVM type: an integer needing more
/// than 256 bits, such as `2**9999`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LiteralTooLarge;

impl DiagnosticExtensions for LiteralTooLarge {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/literal-too-large"
    }

    fn message(&self) -> String {
        "This literal value is too large to fit into EVM slots (32 bytes).".to_owned()
    }
}
