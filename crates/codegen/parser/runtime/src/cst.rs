use std::rc::Rc;

use serde::Serialize;

use super::{cursor::Cursor, kinds::*, text_index::TextIndex};

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
    pub fn rule(kind: RuleKind, children: Vec<Self>) -> Self {
        let text_len = children.iter().map(Node::text_len).sum();

        Self::Rule(Rc::new(RuleNode {
            kind,
            text_len,
            children,
        }))
    }

    pub fn token(kind: TokenKind, text: String) -> Self {
        Self::Token(Rc::new(TokenNode { kind, text }))
    }

    pub fn text_len(&self) -> TextIndex {
        match self {
            Self::Rule(node) => node.text_len,
            Self::Token(node) => (&node.text).into(),
        }
    }

    /// Returns a slice of the children (not all descendants) of this node.
    pub fn children(&self) -> &[Node] {
        match self {
            Self::Rule(node) => &node.children,
            Self::Token(_) => &[],
        }
    }

    /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    pub fn cursor_with_offset(&self, text_offset: TextIndex) -> Cursor {
        Cursor::new(self.clone(), text_offset)
    }

    /// Reconstructs the original source code from the parse tree.
    pub fn unparse(self) -> String {
        match self {
            Self::Rule(rule) => rule.unparse(),
            Self::Token(token) => token.text.clone(),
        }
    }

    pub fn as_rule(&self) -> Option<&Rc<RuleNode>> {
        match self {
            Self::Rule(node) => Some(node),
            _ => None,
        }
    }

    pub fn into_rule(self) -> Option<Rc<RuleNode>> {
        match self {
            Self::Rule(node) => Some(node),
            _ => None,
        }
    }

    pub fn as_token(&self) -> Option<&Rc<TokenNode>> {
        match self {
            Self::Token(node) => Some(node),
            _ => None,
        }
    }

    pub fn into_token(self) -> Option<Rc<TokenNode>> {
        match self {
            Self::Token(node) => Some(node),
            _ => None,
        }
    }

    pub fn as_token_with_kind(&self, kinds: &[TokenKind]) -> Option<&Rc<TokenNode>> {
        self.as_token().filter(|token| kinds.contains(&token.kind))
    }

    pub fn as_rule_with_kind(&self, kinds: &[RuleKind]) -> Option<&Rc<RuleNode>> {
        self.as_rule().filter(|rule| kinds.contains(&rule.kind))
    }
}

impl From<Rc<RuleNode>> for Node {
    fn from(node: Rc<RuleNode>) -> Self {
        Self::Rule(node)
    }
}

impl From<Rc<TokenNode>> for Node {
    fn from(node: Rc<TokenNode>) -> Self {
        Self::Token(node)
    }
}

impl RuleNode {
    /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    pub fn cursor_with_offset(self: Rc<Self>, text_offset: TextIndex) -> Cursor {
        Cursor::new(Node::Rule(self), text_offset)
    }

    /// Reconstructs the original source code from the parse tree.
    pub fn unparse(self: Rc<Self>) -> String {
        let acc = String::with_capacity(self.text_len.utf8);

        self.cursor_with_offset(TextIndex::ZERO)
            .filter_map(Node::into_token)
            .fold(acc, |mut acc, token| {
                acc.push_str(&token.text);
                acc
            })
    }
}
