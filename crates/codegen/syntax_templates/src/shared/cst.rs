use std::rc::Rc;

use serde::Serialize;

use super::{
    cursor::Cursor,
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

    pub fn cursor(&self) -> Cursor {
        Cursor::new(self.clone())
    }

    pub fn as_token_with_kind(&self, kinds: &[TokenKind]) -> Option<&Rc<TokenNode>> {
        if let Node::Token(token_node) = self {
            if kinds.contains(&token_node.kind) {
                return Some(token_node);
            }
        }
        return None;
    }

    pub fn as_token_matching<F: Fn(&Rc<TokenNode>) -> bool>(
        &self,
        predicate: F,
    ) -> Option<&Rc<TokenNode>> {
        if let Node::Token(token_node) = self {
            if predicate(&token_node) {
                return Some(token_node);
            }
        }
        return None;
    }

    pub fn as_rule_with_kind(&self, kinds: &[RuleKind]) -> Option<&Rc<RuleNode>> {
        if let Node::Rule(rule_node) = self {
            if kinds.contains(&rule_node.kind) {
                return Some(rule_node);
            }
        }
        return None;
    }

    pub fn as_rule_matching<F: Fn(&Rc<RuleNode>) -> bool>(
        &self,
        predicate: F,
    ) -> Option<&Rc<RuleNode>> {
        if let Node::Rule(rule_node) = self {
            if predicate(&rule_node) {
                return Some(rule_node);
            }
        }
        return None;
    }
}
