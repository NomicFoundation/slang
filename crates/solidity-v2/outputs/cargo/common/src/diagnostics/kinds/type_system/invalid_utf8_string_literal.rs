use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a string literal whose bytes are not valid UTF-8 is
/// converted to `string`, eg. `string(hex"a000")` or `string("\xff")`. The
/// same literal still converts to `bytes`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidUtf8StringLiteral {
    /// The byte offset of the first invalid UTF-8 sequence in the literal.
    pub position: usize,
}

impl DiagnosticExtensions for InvalidUtf8StringLiteral {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/invalid-utf8-string-literal"
    }

    fn message(&self) -> String {
        format!(
            "This string literal cannot be converted to 'string': it contains an invalid UTF-8 sequence at byte {}.",
            self.position
        )
    }
}
