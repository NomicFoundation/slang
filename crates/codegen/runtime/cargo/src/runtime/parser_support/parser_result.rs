use std::ops::ControlFlow;

use metaslang_cst::TerminalKind;

use crate::cst::{self, LabeledNode, Node};
use crate::kinds::{NodeLabel, RuleKind, TokenKind};
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
    pub fn r#match(nodes: Vec<cst::LabeledNode>, expected_tokens: Vec<TokenKind>) -> Self {
        ParserResult::Match(Match::new(nodes, expected_tokens))
    }

    pub fn pratt_operator_match(elements: Vec<PrattElement>) -> Self {
        ParserResult::PrattOperatorMatch(PrattOperatorMatch::new(elements))
    }

    pub fn incomplete_match(nodes: Vec<cst::LabeledNode>, expected_tokens: Vec<TokenKind>) -> Self {
        ParserResult::IncompleteMatch(IncompleteMatch::new(nodes, expected_tokens))
    }

    /// Whenever a parser didn't run because it's disabled due to versioning. Shorthand for `no_match(vec![])`.
    pub fn disabled() -> Self {
        Self::no_match(vec![])
    }

    pub fn no_match(expected_tokens: Vec<TokenKind>) -> Self {
        ParserResult::NoMatch(NoMatch::new(expected_tokens))
    }

    #[must_use]
    pub fn with_kind(self, new_kind: RuleKind) -> ParserResult {
        match self {
            ParserResult::Match(r#match) => ParserResult::r#match(
                vec![LabeledNode::anonymous(cst::Node::rule(
                    new_kind,
                    r#match.nodes,
                ))],
                r#match.expected_tokens,
            ),
            ParserResult::IncompleteMatch(incomplete_match) => ParserResult::incomplete_match(
                vec![LabeledNode::anonymous(cst::Node::rule(
                    new_kind,
                    incomplete_match.nodes,
                ))],
                incomplete_match.expected_tokens,
            ),
            ParserResult::SkippedUntil(skipped) => ParserResult::SkippedUntil(SkippedUntil {
                nodes: vec![LabeledNode::anonymous(cst::Node::rule(
                    new_kind,
                    skipped.nodes,
                ))],
                ..skipped
            }),
            ParserResult::NoMatch(_) => self,
            ParserResult::PrattOperatorMatch(_) => {
                unreachable!("PrattOperatorMatch cannot be converted to a rule")
            }
        }
    }

    #[must_use]
    pub fn with_label(mut self, label: NodeLabel) -> ParserResult {
        if let Some(LabeledNode {
            label: prev_label, ..
        }) = self.significant_node_mut()
        {
            *prev_label = Some(label);
        }
        // Also allow to name a single trivia token node
        else if let ParserResult::Match(Match { nodes, .. }) = &mut self {
            if let [node] = nodes.as_mut_slice() {
                if node.as_token().is_some_and(|tok| tok.kind.is_trivia()) {
                    node.label = Some(label);
                }
            }
        }

        self
    }

    /// Returns a significant (non-trivia) node if there is exactly one.
    pub(crate) fn significant_node_mut(&mut self) -> Option<&mut cst::LabeledNode> {
        let nodes = match self {
            ParserResult::Match(r#match) => &mut r#match.nodes[..],
            ParserResult::IncompleteMatch(incomplete_match) => &mut incomplete_match.nodes[..],
            ParserResult::SkippedUntil(skipped) => &mut skipped.nodes[..],
            _ => return None,
        };

        let result = nodes.iter_mut().try_fold(None, |acc, next| match acc {
            // Two significant nodes, bail
            Some(_) if !next.is_trivia() => ControlFlow::Break(None),
            Some(_) => ControlFlow::Continue(acc),
            None => ControlFlow::Continue((!next.is_trivia()).then_some(next)),
        });

        match result {
            ControlFlow::Continue(value) => value,
            ControlFlow::Break(value) => value,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Match {
    pub nodes: Vec<cst::LabeledNode>,
    /// Tokens that would have allowed for more progress. Collected for the purposes of error reporting.
    pub expected_tokens: Vec<TokenKind>,
}

impl Match {
    pub fn new(nodes: Vec<cst::LabeledNode>, expected_tokens: Vec<TokenKind>) -> Self {
        Self {
            nodes,
            expected_tokens,
        }
    }

    pub fn is_full_recursive(&self) -> bool {
        self.nodes
            .iter()
            .flat_map(|node| node.cursor_with_offset(TextIndex::ZERO))
            .all(|node| node.as_token_with_kind(TokenKind::SKIPPED).is_none())
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PrattElement {
    Expression {
        nodes: Vec<cst::LabeledNode>,
    },
    Prefix {
        kind: RuleKind,
        nodes: Vec<cst::LabeledNode>,
        right: u8,
    },
    Binary {
        kind: RuleKind,
        nodes: Vec<cst::LabeledNode>,
        left: u8,
        right: u8,
    },
    Postfix {
        kind: RuleKind,
        nodes: Vec<cst::LabeledNode>,
        left: u8,
    },
}

impl PrattElement {
    pub fn into_nodes(self) -> Vec<cst::LabeledNode> {
        match self {
            Self::Expression { nodes } => nodes,
            Self::Binary { kind, nodes, .. }
            | Self::Prefix { kind, nodes, .. }
            | Self::Postfix { kind, nodes, .. } => {
                vec![LabeledNode::anonymous(cst::Node::rule(kind, nodes))]
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
    pub nodes: Vec<cst::LabeledNode>,
    /// Tokens that would have allowed for more progress. Collected for the purposes of error reporting.
    pub expected_tokens: Vec<TokenKind>,
}

impl IncompleteMatch {
    pub fn new(nodes: Vec<cst::LabeledNode>, expected_tokens: Vec<TokenKind>) -> Self {
        Self {
            nodes,
            expected_tokens,
        }
    }

    /// Whether this prefix-matched at least `n` (non-skipped) significant tokens.
    pub fn matches_at_least_n_tokens(&self, n: u8) -> bool {
        let result = self
            .nodes
            .iter()
            .flat_map(|node| node.cursor_with_offset(TextIndex::ZERO))
            .try_fold(0u8, |mut acc, node| {
                match node {
                    Node::Token(tok) if tok.kind != TokenKind::SKIPPED && !tok.kind.is_trivia() => {
                        acc += 1;
                    }
                    _ => {}
                }

                // Short-circuit not to walk the whole tree if we've already matched enough
                if acc >= n {
                    ControlFlow::Break(acc)
                } else {
                    ControlFlow::Continue(acc)
                }
            });

        match result {
            ControlFlow::Continue(value) | ControlFlow::Break(value) => value >= n,
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
    pub nodes: Vec<cst::LabeledNode>,
    /// Skipped text following the last node
    pub skipped: String,
    /// At which token was the stream pointing at when we bailed
    pub found: TokenKind,
    /// Token we expected to skip until
    pub expected: TokenKind,
}
