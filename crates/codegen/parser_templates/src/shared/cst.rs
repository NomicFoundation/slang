use std::ops::Range;
use std::rc::Rc;

use serde::Serialize;

use super::kinds::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    Rule {
        kind: RuleKind,
        span: Range<usize>,
        children: Vec<Rc<Node>>,
    },
    Token {
        kind: TokenKind,
        span: Range<usize>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        trivia: Vec<Rc<Node>>,
    },
}

impl Node {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Rule { children, .. } => children.is_empty(),
            Self::Token { trivia, span, .. } => span.is_empty() && trivia.is_empty(),
        }
    }

    pub fn span(&self) -> Range<usize> {
        match self {
            Self::Rule { span, .. } => span.clone(),
            Self::Token { span, .. } => span.clone(),
        }
    }

    pub fn start(&self) -> usize {
        match self {
            Self::Rule { span, .. } => span.start,
            Self::Token { span, .. } => span.start,
        }
    }

    pub fn end(&self) -> usize {
        match self {
            Self::Rule { span, .. } => span.end,
            Self::Token { span, .. } => span.end,
        }
    }

    pub fn rule(kind: RuleKind, children: Vec<Rc<Self>>) -> Rc<Self> {
        let mut children = children
            .into_iter()
            .filter(|c| !c.is_empty())
            .collect::<Vec<_>>();
        if children.is_empty() {
            return Rc::new(Self::Rule {
                kind,
                span: Range { start: 0, end: 0 },
                children,
            });
        } else if children.len() == 1 {
            let child = children.pop().unwrap();
            if let Self::Rule {
                kind: RuleKind::_ANON,
                span,
                children,
            } = child.as_ref()
            {
                return Rc::new(Self::Rule {
                    kind,
                    span: span.clone(),
                    children: children.clone(),
                });
            } else {
                return Rc::new(Self::Rule {
                    kind,
                    span: child.span(),
                    children: vec![child],
                });
            }
        } else {
            let span = Range {
                start: children.first().unwrap().start(),
                end: children.last().unwrap().end(),
            };
            return Rc::new(Self::Rule {
                kind,
                span,
                children,
            });
        }
    }

    pub fn token(
        kind: TokenKind,
        span: Range<usize>,
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
        Rc::new(Self::Token { kind, span, trivia })
    }

    pub fn top_level_rule(kind: RuleKind, node: Rc<Self>) -> Rc<Self> {
        if let Self::Rule {
            kind: RuleKind::_ANON,
            span,
            children,
        } = node.as_ref()
        {
            return Rc::new(Self::Rule {
                kind,
                span: span.clone(),
                children: children.clone(),
            });
        }
        Rc::new(Self::Rule {
            kind,
            span: node.span(),
            children: vec![node],
        })
    }
}
