use std::collections::BTreeSet;
use std::fmt;

use crate::diagnostic::{self, Diagnostic};
use crate::kinds::TerminalKind;
use crate::text_index::TextRange;

/// Represents an error that occurred during parsing.
///
/// This could have been caused by a syntax error, or by reaching the end of the file when more tokens were expected.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseError {
    pub(crate) text_range: TextRange,
    pub(crate) tokens_that_would_have_allowed_more_progress: Vec<TerminalKind>,
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
    pub(crate) fn new(
        text_range: TextRange,
        tokens_that_would_have_allowed_more_progress: Vec<TerminalKind>,
    ) -> Self {
        Self {
            text_range,
            tokens_that_would_have_allowed_more_progress,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tokens_that_would_have_allowed_more_progress.is_empty() {
            write!(f, "Expected end of file.")
        } else {
            let deduped = self
                .tokens_that_would_have_allowed_more_progress
                .iter()
                .collect::<BTreeSet<_>>();

            write!(f, "Expected ")?;

            for kind in deduped.iter().take(deduped.len() - 1) {
                write!(f, "{kind} or ")?;
            }
            let last = deduped.last().expect("we just checked that it's not empty");
            write!(f, "{last}.")?;

            Ok(())
        }
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
