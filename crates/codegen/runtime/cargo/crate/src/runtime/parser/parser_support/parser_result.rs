use std::ops::ControlFlow;

use crate::cst::{
    Edge, EdgeLabel, Node, NonterminalKind, TerminalKind, TerminalKindExtensions, TextIndex,
};

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
            expected_terminals: vec![],
        })
    }
}

impl ParserResult {
    pub fn r#match(nodes: Vec<Edge>, expected_terminals: Vec<TerminalKind>) -> Self {
        ParserResult::Match(Match::new(nodes, expected_terminals))
    }

    pub fn pratt_operator_match(elements: Vec<PrattElement>) -> Self {
        ParserResult::PrattOperatorMatch(PrattOperatorMatch::new(elements))
    }

    pub fn incomplete_match(nodes: Vec<Edge>, expected_terminals: Vec<TerminalKind>) -> Self {
        ParserResult::IncompleteMatch(IncompleteMatch::new(nodes, expected_terminals))
    }

    pub fn no_match(expected_terminals: Vec<TerminalKind>) -> Self {
        ParserResult::NoMatch(NoMatch::new(expected_terminals))
    }

    #[must_use]
    pub fn with_kind(self, new_kind: NonterminalKind) -> ParserResult {
        match self {
            ParserResult::Match(r#match) => ParserResult::r#match(
                vec![Edge::root(Node::nonterminal(new_kind, r#match.nodes))],
                r#match.expected_terminals,
            ),
            ParserResult::IncompleteMatch(incomplete_match) => ParserResult::incomplete_match(
                vec![Edge::root(Node::nonterminal(
                    new_kind,
                    incomplete_match.nodes,
                ))],
                incomplete_match.expected_terminals,
            ),
            ParserResult::SkippedUntil(skipped) => ParserResult::SkippedUntil(SkippedUntil {
                nodes: vec![Edge::root(Node::nonterminal(new_kind, skipped.nodes))],
                ..skipped
            }),
            ParserResult::NoMatch(no_match) => ParserResult::no_match(no_match.expected_terminals),
            ParserResult::PrattOperatorMatch(_) => {
                unreachable!("PrattOperatorMatch cannot be converted to a nonterminal")
            }
        }
    }

    #[must_use]
    pub fn with_label(mut self, label: EdgeLabel) -> ParserResult {
        if let Some(Edge {
            label: prev_label, ..
        }) = self.significant_node_mut()
        {
            *prev_label = label;
        }
        // Also allow to name a single trivia terminal node
        else if let ParserResult::Match(Match { nodes, .. }) = &mut self {
            if let [node] = nodes.as_mut_slice() {
                if node.as_terminal().is_some_and(|tok| tok.kind.is_trivia()) {
                    node.label = label;
                }
            }
        }

        self
    }

    /// Returns a significant (non-trivia) node if there is exactly one.
    pub(crate) fn significant_node_mut(&mut self) -> Option<&mut Edge> {
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

    /// Whether this prefix-matched at least `n` (non-skipped) significant terminals.
    pub fn matches_at_least_n_terminals(&self, n: u8) -> bool {
        if n == 0 {
            return true;
        }

        match self {
            ParserResult::Match(Match { nodes, .. })
            | ParserResult::IncompleteMatch(IncompleteMatch { nodes, .. })
            | ParserResult::SkippedUntil(SkippedUntil { nodes, .. }) => {
                matches_at_least_n_terminals(nodes.iter(), n)
            }
            ParserResult::PrattOperatorMatch(m) => {
                let nodes = m.elements.iter().flat_map(|elem| elem.nodes());
                matches_at_least_n_terminals(nodes, n)
            }
            ParserResult::NoMatch(_) => false,
        }
    }
}

fn matches_at_least_n_terminals<'a>(nodes: impl Iterator<Item = &'a Edge>, n: u8) -> bool {
    if n == 0 {
        return true;
    }

    let result = nodes
        .flat_map(|edge| {
            edge.node
                .clone()
                .create_cursor(TextIndex::ZERO)
                .remaining_nodes()
        })
        .try_fold(0u8, |mut acc, edge| {
            match edge.node {
                Node::Terminal(tok) if tok.kind.is_valid() && !tok.kind.is_trivia() => {
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Match {
    pub nodes: Vec<Edge>,
    /// Terminals that would have allowed for more progress. Collected for the purposes of error reporting.
    pub expected_terminals: Vec<TerminalKind>,
}

impl Match {
    pub fn new(nodes: Vec<Edge>, expected_terminals: Vec<TerminalKind>) -> Self {
        Self {
            nodes,
            expected_terminals,
        }
    }

    pub fn is_full_recursive(&self) -> bool {
        self.nodes
            .iter()
            .flat_map(|edge| {
                edge.node
                    .clone()
                    .create_cursor(TextIndex::ZERO)
                    .remaining_nodes()
            })
            .all(|edge| {
                edge.as_terminal()
                    .filter(|tok| !tok.kind.is_valid())
                    .is_none()
            })
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PrattElement {
    Expression {
        nodes: Vec<Edge>,
    },
    Prefix {
        kind: NonterminalKind,
        nodes: Vec<Edge>,
        right: u8,
    },
    Binary {
        kind: NonterminalKind,
        nodes: Vec<Edge>,
        left: u8,
        right: u8,
    },
    Postfix {
        kind: NonterminalKind,
        nodes: Vec<Edge>,
        left: u8,
    },
}

impl PrattElement {
    pub fn nodes(&self) -> &Vec<Edge> {
        match self {
            Self::Expression { nodes }
            | Self::Prefix { nodes, .. }
            | Self::Binary { nodes, .. }
            | Self::Postfix { nodes, .. } => nodes,
        }
    }

    pub fn into_nodes(self) -> Vec<Edge> {
        self.into_nodes_with_label(EdgeLabel::Root)
    }

    pub fn into_nodes_with_label(self, label: EdgeLabel) -> Vec<Edge> {
        match self {
            Self::Expression { nodes } => nodes,
            Self::Binary { kind, nodes, .. }
            | Self::Prefix { kind, nodes, .. }
            | Self::Postfix { kind, nodes, .. } => {
                vec![Edge {
                    label,
                    node: Node::nonterminal(kind, nodes),
                }]
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
    pub nodes: Vec<Edge>,
    /// Terminals that would have allowed for more progress. Collected for the purposes of error reporting.
    pub expected_terminals: Vec<TerminalKind>,
}

impl IncompleteMatch {
    pub fn new(nodes: Vec<Edge>, expected_terminals: Vec<TerminalKind>) -> Self {
        Self {
            nodes,
            expected_terminals,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct NoMatch {
    /// Terminals that would have allowed for more progress. Collected for the purposes of error reporting.
    pub expected_terminals: Vec<TerminalKind>,
}

impl NoMatch {
    pub fn new(expected_terminals: Vec<TerminalKind>) -> Self {
        Self { expected_terminals }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SkippedUntil {
    pub nodes: Vec<Edge>,
    /// Skipped text following the last node
    pub skipped: String,
    /// At which terminal was the stream pointing at when we bailed
    pub found: TerminalKind,
    /// Terminal we expected to skip until
    pub expected: TerminalKind,
}
