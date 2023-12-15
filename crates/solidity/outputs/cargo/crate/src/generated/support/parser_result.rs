// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::ops::ControlFlow;

use crate::cst::{self, NamedNode};
use crate::kinds::{RuleKind, TokenKind};
use crate::text_index::TextIndex;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ParserResult {
    Match(Match),
    PrattOperatorMatch(PrattOperatorMatch),
    IncompleteMatch(IncompleteMatch),
    NoMatch(NoMatch),
    SkippedUntil(SkippedUntil),
}

impl Default for ParserResult {
    fn default() -> Self {
        Self::NoMatch(NoMatch {
            expected_tokens: vec![],
        })
    }
}

impl ParserResult {
    pub fn r#match(nodes: Vec<cst::NamedNode>, expected_tokens: Vec<TokenKind>) -> Self {
        ParserResult::Match(Match::new(nodes, expected_tokens))
    }

    pub fn pratt_operator_match(elements: Vec<PrattElement>) -> Self {
        ParserResult::PrattOperatorMatch(PrattOperatorMatch::new(elements))
    }

    pub fn incomplete_match(nodes: Vec<cst::NamedNode>, expected_tokens: Vec<TokenKind>) -> Self {
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
        matches!(
            self,
            ParserResult::Match(_) | ParserResult::PrattOperatorMatch(_)
        )
    }

    pub fn is_no_match(&self) -> bool {
        matches!(self, ParserResult::NoMatch(_))
    }

    #[must_use]
    pub fn with_kind(self, new_kind: RuleKind) -> ParserResult {
        match self {
            ParserResult::Match(r#match) => ParserResult::r#match(
                vec![NamedNode::anon(cst::Node::rule(new_kind, r#match.nodes))],
                r#match.expected_tokens,
            ),
            ParserResult::IncompleteMatch(incomplete_match) => ParserResult::incomplete_match(
                vec![NamedNode::anon(cst::Node::rule(
                    new_kind,
                    incomplete_match.nodes,
                ))],
                incomplete_match.expected_tokens,
            ),
            ParserResult::SkippedUntil(skipped) => ParserResult::SkippedUntil(SkippedUntil {
                nodes: vec![NamedNode::anon(cst::Node::rule(new_kind, skipped.nodes))],
                ..skipped
            }),
            ParserResult::NoMatch(_) => self,
            ParserResult::PrattOperatorMatch(_) => {
                unreachable!("PrattOperatorMatch cannot be converted to a rule")
            }
        }
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> ParserResult {
        if let Some(NamedNode {
            name: prev_name, ..
        }) = self.significant_node_mut()
        {
            *prev_name = name.into();
        }

        self
    }

    /// Returns a significant (non-trivia) node if there is exactly one.
    pub(crate) fn significant_node_mut(&mut self) -> Option<&mut cst::NamedNode> {
        fn is_significant(named: &cst::NamedNode) -> bool {
            match &named.node {
                cst::Node::Rule(rule) => !rule.kind.is_trivia(),
                // FIXME: Some tokens are in fact trivia
                cst::Node::Token(_) => true,
            }
        }

        let nodes = match self {
            ParserResult::Match(r#match) => &mut r#match.nodes[..],
            ParserResult::IncompleteMatch(incomplete_match) => &mut incomplete_match.nodes[..],
            ParserResult::SkippedUntil(skipped) => &mut skipped.nodes[..],
            _ => return None,
        };

        let result = nodes.iter_mut().try_fold(None, |acc, next| match acc {
            // Two significant nodes, bail
            Some(_) if is_significant(next) => ControlFlow::Break(None),
            Some(_) => ControlFlow::Continue(acc),
            None => ControlFlow::Continue(is_significant(next).then_some(next)),
        });

        match result {
            ControlFlow::Continue(value) => value,
            ControlFlow::Break(value) => value,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Match {
    pub nodes: Vec<cst::NamedNode>,
    /// Tokens that would have allowed for more progress. Collected for the purposes of error reporting.
    pub expected_tokens: Vec<TokenKind>,
}

impl Match {
    pub fn new(nodes: Vec<cst::NamedNode>, expected_tokens: Vec<TokenKind>) -> Self {
        Self {
            nodes,
            expected_tokens,
        }
    }

    pub fn is_full_recursive(&self) -> bool {
        self.nodes
            .iter()
            .flat_map(|named| named.node.cursor_with_offset(TextIndex::ZERO))
            .all(|node| node.as_token_with_kind(&[TokenKind::SKIPPED]).is_none())
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PrattElement {
    Expression {
        nodes: Vec<cst::NamedNode>,
    },
    Prefix {
        kind: RuleKind,
        nodes: Vec<cst::NamedNode>,
        right: u8,
    },
    Binary {
        kind: RuleKind,
        nodes: Vec<cst::NamedNode>,
        left: u8,
        right: u8,
    },
    Postfix {
        kind: RuleKind,
        nodes: Vec<cst::NamedNode>,
        left: u8,
    },
}

impl PrattElement {
    pub fn into_nodes(self) -> Vec<cst::NamedNode> {
        match self {
            Self::Expression { nodes } => nodes,
            Self::Binary { kind, nodes, .. }
            | Self::Prefix { kind, nodes, .. }
            | Self::Postfix { kind, nodes, .. } => {
                vec![NamedNode::anon(cst::Node::rule(kind, nodes))]
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
    pub nodes: Vec<cst::NamedNode>,
    /// Tokens that would have allowed for more progress. Collected for the purposes of error reporting.
    pub expected_tokens: Vec<TokenKind>,
}

impl IncompleteMatch {
    pub fn new(nodes: Vec<cst::NamedNode>, expected_tokens: Vec<TokenKind>) -> Self {
        Self {
            nodes,
            expected_tokens,
        }
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SkippedUntil {
    pub nodes: Vec<cst::NamedNode>,
    /// Skipped text following the last node
    pub skipped: String,
    /// At which token was the stream pointing at when we bailed
    pub found: TokenKind,
    /// Token we expected to skip until
    pub expected: TokenKind,
}
