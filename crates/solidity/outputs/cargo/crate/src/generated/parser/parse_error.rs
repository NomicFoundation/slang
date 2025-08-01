// This file is generated automatically by infrastructure scripts (crates/codegen/runner/src/main.rs:29:26). Please don't edit by hand.

use std::fmt;

use crate::cst::{TerminalKind, TextRange};
use crate::diagnostic::{self, Diagnostic};

/// Represents an error that occurred during parsing.
///
/// This could have been caused by a syntax error, or by reaching the end of the file when more tokens were expected.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseError {
    text_range: TextRange,
    terminals_that_would_have_allowed_more_progress: Vec<TerminalKind>,
}

impl ParseError {
    /// The text range at which the error occurred.
    pub fn text_range(&self) -> &TextRange {
        &self.text_range
    }

    /// Renders the message for this error.
    pub fn message(&self) -> String {
        Diagnostic::message(self)
    }
}

impl ParseError {
    pub(crate) fn create(
        text_range: TextRange,
        mut terminals_that_would_have_allowed_more_progress: Vec<TerminalKind>,
    ) -> Self {
        terminals_that_would_have_allowed_more_progress.sort_unstable();
        terminals_that_would_have_allowed_more_progress.dedup();

        Self {
            text_range,
            terminals_that_would_have_allowed_more_progress,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self
            .terminals_that_would_have_allowed_more_progress
            .is_empty()
        {
            return write!(f, "Expected end of file.");
        }

        let mut expected = self.terminals_that_would_have_allowed_more_progress.iter();

        let first = expected
            .next()
            .expect("we just checked that it's not empty");

        write!(f, "Expected {first}")?;

        for kind in expected {
            write!(f, " or {kind}")?;
        }

        write!(f, ".")
    }
}

impl Diagnostic for ParseError {
    fn text_range(&self) -> TextRange {
        self.text_range.clone()
    }

    fn severity(&self) -> diagnostic::Severity {
        diagnostic::Severity::Error
    }

    fn message(&self) -> String {
        // Uses the impl from `Display` above.
        self.to_string()
    }
}
