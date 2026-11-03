use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a plain (non-`unicode`) or Yul string literal holds a character
/// outside of printable ASCII, which only versions before 0.7.0 allowed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidCharacterInString;

impl DiagnosticExtensions for InvalidCharacterInString {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/invalid-character-in-string"
    }

    fn message(&self) -> String {
        // No advice to use a `unicode` string literal instead: import paths can't be one.
        "Invalid character in string. Only printable ASCII is allowed.".to_string()
    }
}
