use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::kinds::syntax::SyntaxDiagnosticKind;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::terminals::TerminalKind;

/// Diagnostic emitted when additional input is found after the parser has
/// already consumed a complete source unit and expected end-of-file.
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl From<ExtraTerminal> for DiagnosticKind {
    fn from(d: ExtraTerminal) -> Self {
        Self::Syntax(SyntaxDiagnosticKind::ExtraTerminal(d))
    }
}
