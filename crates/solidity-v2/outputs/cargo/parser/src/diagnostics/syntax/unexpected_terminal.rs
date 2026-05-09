use std::collections::BTreeSet;

use itertools::Itertools;
use serde::Serialize;
use slang_solidity_v2_common::diagnostics::{DiagnosticExtensions, DiagnosticSeverity};
use slang_solidity_v2_common::terminals::TerminalKind;

use super::SyntaxDiagnosticKind;

/// Diagnostic emitted when the parser encounters a terminal that is not valid
/// at the current position.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
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

impl From<UnexpectedTerminal> for SyntaxDiagnosticKind {
    fn from(d: UnexpectedTerminal) -> Self {
        Self::UnexpectedTerminal(d)
    }
}
