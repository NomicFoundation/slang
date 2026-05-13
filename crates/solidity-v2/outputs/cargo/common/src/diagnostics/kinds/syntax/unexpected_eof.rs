use std::collections::BTreeSet;

use itertools::Itertools;
use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::terminals::TerminalKind;

/// Diagnostic emitted when the parser encounters the end of the input while
/// one or more terminals were still expected.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnexpectedEof {
    /// The set of terminals that could have legally appeared at this position.
    pub expected: BTreeSet<TerminalKind>,
}

impl DiagnosticExtensions for UnexpectedEof {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/unexpected-eof"
    }

    fn message(&self) -> String {
        format!(
            "Unexpected end of file. One of {expected_list} was expected",
            expected_list = self.expected.iter().map(|e| format!("{e}")).join(", ")
        )
    }
}
