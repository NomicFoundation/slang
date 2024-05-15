use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::diagnostic::{self, Diagnostic};
use crate::kinds::TokenKind;
use crate::text_index::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseError {
    pub(crate) text_range: TextRange,
    pub(crate) tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

impl ParseError {
    pub fn text_range(&self) -> &TextRange {
        &self.text_range
    }
}

impl ParseError {
    pub(crate) fn new(
        text_range: TextRange,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        Self {
            text_range,
            tokens_that_would_have_allowed_more_progress,
        }
    }
}

impl Error for ParseError {}
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
    fn range(&self) -> TextRange {
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
