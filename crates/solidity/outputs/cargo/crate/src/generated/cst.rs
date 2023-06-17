// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::cmp::{max, min};
use std::ops::Range;
use std::rc::Rc;

use serde::Serialize;

use super::kinds::*;
use super::language::{TextPosition, TextRange};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    Rule {
        kind: RuleKind,
        range: TextRange,
        children: Vec<Rc<Node>>,
    },
    Token {
        kind: TokenKind,
        range: TextRange,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        trivia: Vec<Rc<Node>>,
    },
}

impl Node {
    pub fn range(&self) -> TextRange {
        match self {
            Self::Rule { range, .. } => range.clone(),
            Self::Token { range, .. } => range.clone(),
        }
    }

    pub fn range_including_trivia(&self) -> TextRange {
        match self {
            Self::Rule { range, .. } => range.clone(),
            Self::Token { range, trivia, .. } => {
                if trivia.is_empty() {
                    range.clone()
                } else {
                    Range {
                        start: min(
                            range.start,
                            trivia.first().unwrap().range_including_trivia().start,
                        ),
                        end: max(
                            range.end,
                            trivia.last().unwrap().range_including_trivia().end,
                        ),
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
impl Node {
    pub(crate) fn rule(kind: RuleKind, range: TextRange, children: Vec<Rc<Self>>) -> Rc<Self> {
        return Rc::new(Node::Rule {
            kind,
            range,
            children,
        });
    }

    pub(crate) fn token(
        kind: TokenKind,
        range: TextRange,
        leading_trivia: Option<Rc<Node>>,
        trailing_trivia: Option<Rc<Node>>,
    ) -> Rc<Self> {
        let mut trivia = vec![];
        if let Some(leading_trivia) = leading_trivia {
            trivia.push(leading_trivia)
        }
        if let Some(trailing_trivia) = trailing_trivia {
            trivia.push(trailing_trivia)
        }

        return Rc::new(Node::Token {
            kind,
            range,
            trivia,
        });
    }
}

pub(crate) enum NodeBuilder {
    Empty { position: TextPosition },
    Single { node: Rc<Node> },
    Multiple { nodes: Vec<Rc<Node>> },
}

#[allow(dead_code)]
impl NodeBuilder {
    pub(crate) fn empty(position: TextPosition) -> Self {
        return Self::Empty { position };
    }

    pub(crate) fn single(node: Rc<Node>) -> Self {
        return Self::Single { node };
    }

    pub(crate) fn multiple(builders: Vec<Self>) -> Self {
        assert_ne!(
            builders.len(),
            0,
            "codegen should have used empty() builder instead."
        );

        if builders.len() == 1 {
            return builders.into_iter().next().unwrap();
        }

        let start_position = builders.first().unwrap().range().start;

        let mut nodes = vec![];

        for builder in builders {
            match builder {
                Self::Empty { .. } => {}
                Self::Single { node: other } => nodes.push(other),
                Self::Multiple { nodes: mut other } => nodes.append(&mut other),
            }
        }

        return if nodes.is_empty() {
            Self::Empty {
                position: start_position,
            }
        } else {
            Self::Multiple { nodes }
        };
    }

    pub(crate) fn with_kind(self, kind: RuleKind) -> Self {
        let range = self.range();

        let children = match self {
            Self::Empty { .. } => vec![],
            Self::Single { node } => vec![node],
            Self::Multiple { nodes } => nodes,
        };

        return Self::Single {
            node: Node::rule(kind, range, children),
        };
    }

    pub(crate) fn build(self) -> Rc<Node> {
        return match self {
            Self::Single { node } => node,
            _ => panic!("cannot build unnamed nodes"),
        };
    }

    pub(crate) fn range(&self) -> TextRange {
        return match self {
            Self::Empty { position } => position.to_owned()..position.to_owned(),
            Self::Single { node } => node.range_including_trivia(),
            Self::Multiple { nodes } => Range {
                start: nodes.first().unwrap().range_including_trivia().start,
                end: nodes.last().unwrap().range_including_trivia().end,
            },
        };
    }
}
