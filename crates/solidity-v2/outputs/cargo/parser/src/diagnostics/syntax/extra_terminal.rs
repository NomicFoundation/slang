use serde::Serialize;
use slang_solidity_v2_common::diagnostics::{DiagnosticExtensions, DiagnosticSeverity};
use slang_solidity_v2_common::terminals::TerminalKind;

use super::SyntaxDiagnosticKind;

/// Diagnostic emitted when additional input is found after the parser has
/// already consumed a complete source unit and expected end-of-file.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExtraTerminal {
    /// The extra terminal kind that was encountered past the end of the unit.
    pub found: TerminalKind,
}

impl DiagnosticExtensions for ExtraTerminal {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/extra-terminal"
    }

    fn message(&self) -> String {
        format!(
            "Unexpected {found}. End of file was expected",
            found = self.found
        )
    }
}

impl From<ExtraTerminal> for SyntaxDiagnosticKind {
    fn from(d: ExtraTerminal) -> Self {
        Self::ExtraTerminal(d)
    }
}
