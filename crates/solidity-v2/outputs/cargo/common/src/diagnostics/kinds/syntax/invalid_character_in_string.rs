use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Which language a string literal belongs to: Solidity and Yul report the same mistakes
/// differently, as Yul has no `unicode` string literals.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum StringLiteralKind {
    Solidity,
    Yul,
}

/// Diagnostic emitted when a plain (non-`unicode`) or Yul string literal holds a character
/// outside of printable ASCII, which only versions before 0.7.0 allowed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidCharacterInString {
    pub kind: StringLiteralKind,
}

impl DiagnosticExtensions for InvalidCharacterInString {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/invalid-character-in-string"
    }

    fn message(&self) -> String {
        match self.kind {
            StringLiteralKind::Solidity => "Invalid character in string. Use a 'unicode\"...\"' string literal if you're trying to use Unicode characters.",
            StringLiteralKind::Yul => "Invalid character in string. Only printable ASCII is allowed.",
        }
        .to_string()
    }
}
