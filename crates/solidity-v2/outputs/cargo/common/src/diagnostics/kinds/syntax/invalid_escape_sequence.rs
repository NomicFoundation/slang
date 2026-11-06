use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a string literal holds a backslash that does not start a valid
/// escape sequence
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidEscapeSequence;

impl DiagnosticExtensions for InvalidEscapeSequence {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/invalid-escape-sequence"
    }

    fn message(&self) -> String {
        "Invalid escape sequence. Only '\\n', '\\r', '\\t', '\\'', '\\\"', '\\\\', '\\xNN', '\\uNNNN' or a newline can follow a backslash.".to_string()
    }
}
