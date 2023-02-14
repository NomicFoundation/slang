// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::cmp::{max, min};
use std::ops::Range;
use std::rc::Rc;

use serde::Serialize;

use super::kinds::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    Rule {
        kind: RuleKind,
        range: Range<usize>,
        children: Vec<Rc<Node>>,
    },
    Token {
        kind: TokenKind,
        range: Range<usize>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        trivia: Vec<Rc<Node>>,
    },
}

impl Node {
    pub fn range(&self) -> Range<usize> {
        match self {
            Self::Rule { range, .. } => range.clone(),
            Self::Token { range, .. } => range.clone(),
        }
    }

    pub fn range_including_trivia(&self) -> Range<usize> {
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

    pub fn rule(kind: RuleKind, children: Vec<Rc<Self>>) -> Rc<Self> {
        let mut flattened_children: Vec<Rc<Self>> = Vec::new();
        for child in children.into_iter() {
            match child.as_ref() {
                Node::Rule { children, .. } if children.is_empty() => {}
                Node::Rule {
                    kind: RuleKind::_SEQUENCE,
                    children,
                    ..
                } => flattened_children.extend(children.iter().cloned()),
                _ => flattened_children.push(child.clone()),
            }
        }
        let range = if flattened_children.is_empty() {
            Range { start: 0, end: 0 }
        } else {
            Range {
                start: flattened_children
                    .first()
                    .unwrap()
                    .range_including_trivia()
                    .start,
                end: flattened_children
                    .last()
                    .unwrap()
                    .range_including_trivia()
                    .end,
            }
        };
        return Rc::new(Self::Rule {
            kind,
            range,
            children: flattened_children,
        });
    }

    pub fn token(
        kind: TokenKind,
        range: Range<usize>,
        leading_trivia: Option<Rc<Self>>,
        trailing_trivia: Option<Rc<Self>>,
    ) -> Rc<Self> {
        let mut trivia = vec![];
        if let Some(leading_trivia) = leading_trivia {
            trivia.push(leading_trivia)
        }
        if let Some(trailing_trivia) = trailing_trivia {
            trivia.push(trailing_trivia)
        }
        Rc::new(Self::Token {
            kind,
            range,
            trivia,
        })
    }

    pub fn top_level_rule(kind: RuleKind, node: Rc<Self>) -> Rc<Self> {
        if let Self::Rule {
            kind: RuleKind::_SEQUENCE,
            range,
            children,
        } = node.as_ref()
        {
            Rc::new(Self::Rule {
                kind,
                range: range.clone(),
                children: children.clone(),
            })
        } else {
            Rc::new(Self::Rule {
                kind,
                range: node.range(),
                children: vec![node],
            })
        }
    }
}
