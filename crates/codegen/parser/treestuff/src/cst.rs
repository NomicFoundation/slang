use std::rc::Rc;

use serde::Serialize;

use crate::cursor::Cursor;
use crate::text_index::TextIndex;
use crate::ModuleInputs;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TerminalNode<T: ModuleInputs> {
    pub kind: T::TerminalKind,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NonTerminalNode<T: ModuleInputs> {
    pub kind: T::NonTerminalKind,
    pub text_len: TextIndex,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<LabeledNode<T>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node<T: ModuleInputs> {
    Rule(Rc<NonTerminalNode<T>>),
    Token(Rc<TerminalNode<T>>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LabeledNode<T: ModuleInputs> {
    pub label: Option<T::LabelKind>,
    pub node: Node<T>,
}

impl<T: ModuleInputs> LabeledNode<T> {
    /// Creates an anonymous node (without a label).
    pub fn anonymous(node: Node<T>) -> Self {
        Self { label: None, node }
    }
}

impl<T: ModuleInputs> std::ops::Deref for LabeledNode<T> {
    type Target = Node<T>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<T: ModuleInputs> Node<T> {
    pub fn rule(kind: T::NonTerminalKind, children: Vec<LabeledNode<T>>) -> Self {
        let text_len = children.iter().map(|node| node.text_len()).sum();

        Self::Rule(Rc::new(NonTerminalNode::<T> {
            kind,
            text_len,
            children,
        }))
    }

    pub fn token(kind: T::TerminalKind, text: String) -> Self {
        Self::Token(Rc::new(TerminalNode::<T> { kind, text }))
    }

    pub fn text_len(&self) -> TextIndex {
        match self {
            Self::Rule(node) => node.text_len,
            Self::Token(node) => (&node.text).into(),
        }
    }

    /// Returns a slice of the children (not all descendants) of this node.
    pub fn children(&self) -> &[LabeledNode<T>] {
        match self {
            Self::Rule(node) => &node.children,
            Self::Token(_) => &[],
        }
    }

    /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    pub fn cursor_with_offset(&self, text_offset: TextIndex) -> Cursor<T> {
        Cursor::<T>::new(self.clone(), text_offset)
    }

    /// Reconstructs the original source code from the parse tree.
    pub fn unparse(self) -> String {
        match self {
            Self::Rule(rule) => rule.unparse(),
            Self::Token(token) => token.text.clone(),
        }
    }

    pub fn into_rule(self) -> Option<Rc<NonTerminalNode<T>>> {
        match self {
            Self::Rule(rule) => Some(rule),
            Self::Token(..) => None,
        }
    }

    pub fn is_rule(&self) -> bool {
        self.as_rule().is_some()
    }

    pub fn as_rule(&self) -> Option<&Rc<NonTerminalNode<T>>> {
        match self {
            Self::Rule(rule) => Some(rule),
            Self::Token(..) => None,
        }
    }

    pub fn is_rule_with_kind(&self, kind: T::NonTerminalKind) -> bool {
        self.as_rule_with_kind(kind).is_some()
    }

    pub fn as_rule_with_kind(&self, kind: T::NonTerminalKind) -> Option<&Rc<NonTerminalNode<T>>> {
        self.as_rule().filter(|rule| rule.kind == kind)
    }

    pub fn is_rule_with_kinds(&self, kinds: &[T::NonTerminalKind]) -> bool {
        self.as_rule_with_kinds(kinds).is_some()
    }

    pub fn as_rule_with_kinds(
        &self,
        kinds: &[T::NonTerminalKind],
    ) -> Option<&Rc<NonTerminalNode<T>>> {
        self.as_rule().filter(|rule| kinds.contains(&rule.kind))
    }

    pub fn into_token(self) -> Option<Rc<TerminalNode<T>>> {
        match self {
            Self::Token(token) => Some(token),
            Self::Rule(..) => None,
        }
    }

    pub fn is_token(&self) -> bool {
        self.as_token().is_some()
    }

    pub fn as_token(&self) -> Option<&Rc<TerminalNode<T>>> {
        match self {
            Self::Token(token) => Some(token),
            Self::Rule(..) => None,
        }
    }

    pub fn is_token_with_kind(&self, kind: T::TerminalKind) -> bool {
        self.as_token_with_kind(kind).is_some()
    }

    pub fn as_token_with_kind(&self, kind: T::TerminalKind) -> Option<&Rc<TerminalNode<T>>> {
        self.as_token().filter(|token| token.kind == kind)
    }

    pub fn is_token_with_kinds(&self, kinds: &[T::TerminalKind]) -> bool {
        self.as_token_with_kinds(kinds).is_some()
    }

    pub fn as_token_with_kinds(&self, kinds: &[T::TerminalKind]) -> Option<&Rc<TerminalNode<T>>> {
        self.as_token().filter(|token| kinds.contains(&token.kind))
    }
}

impl<T: ModuleInputs> From<Rc<NonTerminalNode<T>>> for Node<T> {
    fn from(node: Rc<NonTerminalNode<T>>) -> Self {
        Self::Rule(node)
    }
}

impl<T: ModuleInputs> From<Rc<TerminalNode<T>>> for Node<T> {
    fn from(node: Rc<TerminalNode<T>>) -> Self {
        Self::Token(node)
    }
}

impl<T: ModuleInputs> NonTerminalNode<T> {
    /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    pub fn cursor_with_offset(self: Rc<Self>, text_offset: TextIndex) -> Cursor<T> {
        Cursor::<T>::new(Node::<T>::Rule(self), text_offset)
    }

    /// Reconstructs the original source code from the parse tree.
    pub fn unparse(self: Rc<Self>) -> String {
        let acc = String::with_capacity(self.text_len.utf8);

        self.cursor_with_offset(TextIndex::ZERO)
            .filter_map(|node| node.into_token())
            .fold(acc, |mut acc, token| {
                acc.push_str(&token.text);
                acc
            })
    }
}
