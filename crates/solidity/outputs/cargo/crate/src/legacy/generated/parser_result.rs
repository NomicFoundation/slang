// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{cst, stream::Stream};

use crate::syntax::nodes::{RuleKind, TokenKind};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Match {
    pub nodes: Vec<cst::Node>,
    pub tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

impl Match {
    pub fn new(
        nodes: Vec<cst::Node>,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        Self {
            nodes,
            tokens_that_would_have_allowed_more_progress,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PrattOperatorMatch {
    pub nodes: Vec<cst::Node>,
    pub operator_kind: RuleKind,
    pub left_binding_power: u8,
    pub right_binding_power: u8,
}

impl PrattOperatorMatch {
    pub fn new(
        nodes: Vec<cst::Node>,
        operator_kind: RuleKind,
        left_binding_power: u8,
        right_binding_power: u8,
    ) -> Self {
        Self {
            nodes,
            operator_kind,
            left_binding_power,
            right_binding_power,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct IncompleteMatch {
    pub nodes: Vec<cst::Node>,
    pub tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

impl IncompleteMatch {
    pub fn new(
        nodes: Vec<cst::Node>,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        Self {
            nodes,
            tokens_that_would_have_allowed_more_progress,
        }
    }

    pub fn consume_stream(&self, stream: &mut Stream) {
        for node in &self.nodes {
            for _ in 0..node.text_len().char {
                stream.next();
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct NoMatch {
    pub tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

impl NoMatch {
    pub fn new(tokens_that_would_have_allowed_more_progress: Vec<TokenKind>) -> Self {
        Self {
            tokens_that_would_have_allowed_more_progress,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ParserResult {
    Match(Match),
    PrattOperatorMatch(PrattOperatorMatch),
    IncompleteMatch(IncompleteMatch),
    NoMatch(NoMatch),
}

impl ParserResult {
    pub fn r#match(
        nodes: Vec<cst::Node>,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        ParserResult::Match(Match::new(
            nodes,
            tokens_that_would_have_allowed_more_progress,
        ))
    }

    pub fn pratt_operator_match(
        nodes: Vec<cst::Node>,
        operator_kind: RuleKind,
        left_binding_power: u8,
        right_binding_power: u8,
    ) -> Self {
        ParserResult::PrattOperatorMatch(PrattOperatorMatch::new(
            nodes,
            operator_kind,
            left_binding_power,
            right_binding_power,
        ))
    }

    pub fn incomplete_match(
        nodes: Vec<cst::Node>,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        ParserResult::IncompleteMatch(IncompleteMatch::new(
            nodes,
            tokens_that_would_have_allowed_more_progress,
        ))
    }

    pub fn no_match(tokens_that_would_have_allowed_more_progress: Vec<TokenKind>) -> Self {
        ParserResult::NoMatch(NoMatch::new(tokens_that_would_have_allowed_more_progress))
    }

    pub fn to_pratt_element_operator(
        self,
        operator_kind: RuleKind,
        left_binding_power: u8,
        right_binding_power: u8,
    ) -> Self {
        match self {
            ParserResult::Match(r#match) => Self::pratt_operator_match(
                r#match.nodes,
                operator_kind,
                left_binding_power,
                right_binding_power,
            ),
            ParserResult::PrattOperatorMatch(_) => {
                unreachable!("This is already a PrattOperatorMatch")
            }
            _ => self,
        }
    }

    pub fn is_match(&self) -> bool {
        match self {
            ParserResult::Match(_) | ParserResult::PrattOperatorMatch(_) => true,
            _ => false,
        }
    }

    pub fn with_kind(self, new_kind: RuleKind) -> ParserResult {
        match self {
            ParserResult::Match(r#match) => ParserResult::r#match(
                vec![cst::Node::rule(new_kind, r#match.nodes)],
                r#match.tokens_that_would_have_allowed_more_progress,
            ),
            ParserResult::IncompleteMatch(incomplete_match) => ParserResult::incomplete_match(
                vec![cst::Node::rule(new_kind, incomplete_match.nodes)],
                incomplete_match.tokens_that_would_have_allowed_more_progress,
            ),
            ParserResult::NoMatch(_) => self,
            _ => unreachable!("PrattOperatorMatch cannot be converted to a rule"),
        }
    }

    pub fn is_better_match_than(&self, other: &ParserResult) -> bool {
        match (self, other) {
            (ParserResult::Match(_), _) | (ParserResult::PrattOperatorMatch(_), _) => true,
            (_, ParserResult::Match(_)) | (_, ParserResult::PrattOperatorMatch(_)) => false,
            (ParserResult::NoMatch(_), _) => false,
            (_, ParserResult::NoMatch(_)) => true,
            (
                ParserResult::IncompleteMatch(IncompleteMatch {
                    nodes: first_match, ..
                }),
                ParserResult::IncompleteMatch(IncompleteMatch {
                    nodes: second_match,
                    ..
                }),
            ) => {
                let first_size = first_match
                    .iter()
                    .fold(0, |acc, node| acc + node.text_len().utf8);
                let second_size = second_match
                    .iter()
                    .fold(0, |acc, node| acc + node.text_len().utf8);
                first_size < second_size
            }
        }
    }
}
