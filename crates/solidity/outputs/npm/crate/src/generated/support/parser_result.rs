// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{
    super::{cst, kinds::*},
    context::ParserContext,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ParserResult {
    Match(Match),
    PrattOperatorMatch(PrattOperatorMatch),
    IncompleteMatch(IncompleteMatch),
    NoMatch(NoMatch),
}

impl ParserResult {
    pub fn r#match(nodes: Vec<cst::Node>, expected_tokens: Vec<TokenKind>) -> Self {
        ParserResult::Match(Match::new(nodes, expected_tokens))
    }

    pub fn pratt_operator_match(elements: Vec<PrattElement>) -> Self {
        ParserResult::PrattOperatorMatch(PrattOperatorMatch::new(elements))
    }

    pub fn incomplete_match(nodes: Vec<cst::Node>, expected_tokens: Vec<TokenKind>) -> Self {
        ParserResult::IncompleteMatch(IncompleteMatch::new(nodes, expected_tokens))
    }

    /// Whenever a parser didn't run because it's disabled due to versioning. Shorthand for `no_match(vec![])`.
    pub fn disabled() -> Self {
        Self::no_match(vec![])
    }

    pub fn no_match(expected_tokens: Vec<TokenKind>) -> Self {
        ParserResult::NoMatch(NoMatch::new(expected_tokens))
    }

    pub fn is_match(&self) -> bool {
        match self {
            ParserResult::Match(_) | ParserResult::PrattOperatorMatch(_) => true,
            _ => false,
        }
    }

    pub fn is_no_match(&self) -> bool {
        match self {
            ParserResult::NoMatch(_) => true,
            _ => false,
        }
    }

    pub fn with_kind(self, new_kind: RuleKind) -> ParserResult {
        match self {
            ParserResult::Match(r#match) => ParserResult::r#match(
                vec![cst::Node::rule(new_kind, r#match.nodes)],
                r#match.expected_tokens,
            ),
            ParserResult::IncompleteMatch(incomplete_match) => ParserResult::incomplete_match(
                vec![cst::Node::rule(new_kind, incomplete_match.nodes)],
                incomplete_match.expected_tokens,
            ),
            ParserResult::NoMatch(_) => self,
            _ => unreachable!("PrattOperatorMatch cannot be converted to a rule"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Match {
    pub nodes: Vec<cst::Node>,
    /// Tokens that would have allowed for more progress. Collected for the purposes of error reporting.
    pub expected_tokens: Vec<TokenKind>,
}

impl Match {
    pub fn new(nodes: Vec<cst::Node>, expected_tokens: Vec<TokenKind>) -> Self {
        Self {
            nodes,
            expected_tokens,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PrattElement {
    Expression {
        nodes: Vec<cst::Node>,
    },
    Prefix {
        kind: RuleKind,
        nodes: Vec<cst::Node>,
        right: u8,
    },
    Binary {
        kind: RuleKind,
        nodes: Vec<cst::Node>,
        left: u8,
        right: u8,
    },
    Postfix {
        kind: RuleKind,
        nodes: Vec<cst::Node>,
        left: u8,
    },
}

impl PrattElement {
    pub fn to_nodes(self) -> Vec<cst::Node> {
        match self {
            Self::Expression { nodes } => nodes.clone(),
            Self::Binary { kind, nodes, .. }
            | Self::Prefix { kind, nodes, .. }
            | Self::Postfix { kind, nodes, .. } => {
                vec![cst::Node::rule(kind, nodes)]
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PrattOperatorMatch {
    pub elements: Vec<PrattElement>,
}

impl PrattOperatorMatch {
    pub fn new(elements: Vec<PrattElement>) -> Self {
        Self { elements }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct IncompleteMatch {
    pub nodes: Vec<cst::Node>,
    /// Tokens that would have allowed for more progress. Collected for the purposes of error reporting.
    pub expected_tokens: Vec<TokenKind>,
}

impl IncompleteMatch {
    pub fn new(nodes: Vec<cst::Node>, expected_tokens: Vec<TokenKind>) -> Self {
        Self {
            nodes,
            expected_tokens,
        }
    }

    /// Advances the stream by the length of the nodes in this match.
    ///
    /// This is used whenever we "accept" the match, even though it's incomplete.
    pub fn consume_stream(&self, input: &mut ParserContext) {
        for node in &self.nodes {
            for _ in 0..node.text_len().char {
                input.next();
            }
        }
    }

    /// Whether this match covers more (not including skipped) bytes than the other.
    pub fn covers_more_than(&self, other: &Self) -> bool {
        let [self_match_len, other_match_len] = [self, other].map(|incomplete| {
            incomplete
                .nodes
                .iter()
                .map(|node| node.text_len().utf8)
                .sum::<usize>()
        });

        self_match_len > other_match_len
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct NoMatch {
    /// Tokens that would have allowed for more progress. Collected for the purposes of error reporting.
    pub expected_tokens: Vec<TokenKind>,
}

impl NoMatch {
    pub fn new(expected_tokens: Vec<TokenKind>) -> Self {
        Self { expected_tokens }
    }
}
