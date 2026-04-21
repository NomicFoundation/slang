use std::collections::BTreeSet;

use itertools::Itertools;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::kinds::syntax::SyntaxDiagnosticKind;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::terminals::TerminalKind;

/// Diagnostic emitted when the parser encounters a terminal that is not valid
/// at the current position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnexpectedTerminal {
    /// The terminal kind that was encountered at the failure site.
    pub found: TerminalKind,
    /// The set of terminals that could have legally appeared at this position.
    pub expected: BTreeSet<TerminalKind>,
}

impl DiagnosticExtensions for UnexpectedTerminal {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/unexpected-terminal"
    }

    fn message(&self) -> String {
        format!(
            "Unexpected {found}. One of {expected_list} was expected",
            found = self.found,
            expected_list = self.expected.iter().map(|e| format!("{e}")).join(", ")
        )
    }
}

impl From<UnexpectedTerminal> for DiagnosticKind {
    fn from(d: UnexpectedTerminal) -> Self {
        Self::Syntax(SyntaxDiagnosticKind::UnexpectedTerminal(d))
    }
}
