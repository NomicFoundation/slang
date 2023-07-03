use std::rc::Rc;

use serde::Serialize;

use super::{
    kinds::{RuleKind, TokenKind},
    text_index::TextIndex,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RuleNode {
    pub kind: RuleKind,
    pub text_len: TextIndex,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TokenNode {
    pub kind: TokenKind,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    Rule(Rc<RuleNode>),
    Token(Rc<TokenNode>),
}

impl Node {
    #[allow(dead_code)]
    pub(crate) fn rule(kind: RuleKind, children: Vec<Self>) -> Self {
        let mut text_len = Default::default();
        for child in &children {
            text_len += child.text_len();
        }
        return Self::Rule(Rc::new(RuleNode {
            kind,
            text_len,
            children: children,
        }));
    }

    #[allow(dead_code)]
    pub(crate) fn token(kind: TokenKind, text: String) -> Self {
        Self::Token(Rc::new(TokenNode { kind, text }))
    }

    pub fn text_len(&self) -> TextIndex {
        match self {
            Self::Rule(node) => node.text_len,
            Self::Token(node) => (&node.text).into(),
        }
    }
}
