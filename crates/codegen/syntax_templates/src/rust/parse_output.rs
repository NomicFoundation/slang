use std::collections::BTreeSet;

use super::{cst, kinds::TokenKind, parse_error::render_error_report, text_index::TextRange};

#[derive(Debug, PartialEq)]
pub struct ParseOutput {
    pub(crate) parse_tree: cst::Node,
    pub(crate) errors: Vec<ParseError>,
}

impl ParseOutput {
    pub fn parse_tree(&self) -> cst::Node {
        return self.parse_tree.clone();
    }

    pub fn errors(&self) -> &Vec<ParseError> {
        return &self.errors;
    }

    pub fn is_valid(&self) -> bool {
        return self.errors.is_empty();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub(crate) range: TextRange,
    pub(crate) tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

impl ParseError {
    pub fn range(&self) -> &TextRange {
        return &self.range;
    }

    pub fn tokens_that_would_have_allowed_more_progress(&self) -> Vec<String> {
        let tokens_that_would_have_allowed_more_progress = self
            .tokens_that_would_have_allowed_more_progress
            .iter()
            .map(|kind| kind.as_ref())
            .collect::<BTreeSet<_>>();
        return tokens_that_would_have_allowed_more_progress
            .into_iter()
            .map(|token| token.to_string())
            .collect();
    }

    pub fn to_error_report(&self, source_id: &str, source: &str, with_colour: bool) -> String {
        return render_error_report(self, source_id, source, with_colour);
    }
}
