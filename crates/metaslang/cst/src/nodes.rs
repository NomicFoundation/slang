use std::rc::Rc;

use serde::Serialize;

use crate::cursor::{Cursor, CursorIterator};
use crate::kinds::{KindTypes, NodeKind, TerminalKindExtensions};
use crate::text_index::TextIndex;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TerminalNode<T: KindTypes> {
    pub kind: T::TerminalKind,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NonterminalNode<T: KindTypes> {
    pub kind: T::NonterminalKind,

    // skip serde since this doesn't exist on `TerminalNode`. We can add to both in the future if found useful.
    #[serde(skip)]
    pub text_len: TextIndex,

    pub children: Vec<Edge<T>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum Node<T: KindTypes> {
    Nonterminal(Rc<NonterminalNode<T>>),
    Terminal(Rc<TerminalNode<T>>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Edge<T: KindTypes> {
    pub label: Option<T::EdgeLabel>,
    pub node: Node<T>,
}

impl<T: KindTypes> NonterminalNode<T> {
    pub fn new(kind: T::NonterminalKind, children: Vec<Edge<T>>) -> Self {
        let text_len = children.iter().map(|edge| edge.text_len()).sum();

        NonterminalNode {
            kind,
            text_len,
            children,
        }
    }
}

impl<T: KindTypes> Edge<T> {
    /// Creates an anonymous node (without a label).
    pub fn anonymous(node: Node<T>) -> Self {
        Self { label: None, node }
    }
}

impl<T: KindTypes> std::ops::Deref for Edge<T> {
    type Target = Node<T>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<T: KindTypes> Node<T> {
    pub fn nonterminal(kind: T::NonterminalKind, children: Vec<Edge<T>>) -> Self {
        Self::Nonterminal(Rc::new(NonterminalNode::new(kind, children)))
    }

    pub fn terminal(kind: T::TerminalKind, text: String) -> Self {
        Self::Terminal(Rc::new(TerminalNode { kind, text }))
    }

    /// Returns a unique identifier of the node. It is not reproducable over parses
    /// and cannot be used in a persistent/serialised sense.
    pub fn id(&self) -> usize {
        match self {
            Self::Nonterminal(node) => node.id(),
            Self::Terminal(node) => node.id(),
        }
    }

    pub fn kind(&self) -> NodeKind<T> {
        match self {
            Self::Nonterminal(node) => NodeKind::Nonterminal(node.kind),
            Self::Terminal(node) => NodeKind::Terminal(node.kind),
        }
    }

    pub fn text_len(&self) -> TextIndex {
        match self {
            Self::Nonterminal(node) => node.text_len,
            Self::Terminal(node) => (&node.text).into(),
        }
    }

    /// Returns the list of child edges directly connected to this node.
    pub fn children(&self) -> &[Edge<T>] {
        match self {
            Self::Nonterminal(node) => node.children(),
            Self::Terminal(node) => node.children(),
        }
    }

    /// Returns an iterator over all descendants of the current node in pre-order traversal.
    pub fn descendants(self) -> CursorIterator<T> {
        Cursor::new(self, TextIndex::ZERO).descendants()
    }

    /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    pub fn cursor_with_offset(self, text_offset: TextIndex) -> Cursor<T> {
        Cursor::new(self, text_offset)
    }

    /// Reconstructs the original source code from the node and its sub-tree.
    pub fn unparse(&self) -> String {
        match self {
            Self::Nonterminal(node) => node.unparse(),
            Self::Terminal(node) => node.unparse(),
        }
    }

    pub fn into_nonterminal(self) -> Option<Rc<NonterminalNode<T>>> {
        match self {
            Self::Nonterminal(nonterminal) => Some(nonterminal),
            Self::Terminal(..) => None,
        }
    }

    pub fn is_nonterminal(&self) -> bool {
        self.as_nonterminal().is_some()
    }

    pub fn as_nonterminal(&self) -> Option<&Rc<NonterminalNode<T>>> {
        match self {
            Self::Nonterminal(nonterminal) => Some(nonterminal),
            Self::Terminal(..) => None,
        }
    }

    pub fn is_nonterminal_with_kind(&self, kind: T::NonterminalKind) -> bool {
        self.as_nonterminal_with_kind(kind).is_some()
    }

    pub fn as_nonterminal_with_kind(
        &self,
        kind: T::NonterminalKind,
    ) -> Option<&Rc<NonterminalNode<T>>> {
        self.as_nonterminal().filter(|node| node.kind == kind)
    }

    pub fn is_nonterminal_with_kinds(&self, kinds: &[T::NonterminalKind]) -> bool {
        self.as_nonterminal_with_kinds(kinds).is_some()
    }

    pub fn as_nonterminal_with_kinds(
        &self,
        kinds: &[T::NonterminalKind],
    ) -> Option<&Rc<NonterminalNode<T>>> {
        self.as_nonterminal()
            .filter(|nonterminal| kinds.contains(&nonterminal.kind))
    }

    pub fn into_terminal(self) -> Option<Rc<TerminalNode<T>>> {
        match self {
            Self::Terminal(terminal) => Some(terminal),
            Self::Nonterminal(..) => None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.as_terminal().is_some()
    }

    pub fn as_terminal(&self) -> Option<&Rc<TerminalNode<T>>> {
        match self {
            Self::Terminal(terminal) => Some(terminal),
            Self::Nonterminal(..) => None,
        }
    }

    pub fn is_terminal_with_kind(&self, kind: T::TerminalKind) -> bool {
        self.as_terminal_with_kind(kind).is_some()
    }

    pub fn as_terminal_with_kind(&self, kind: T::TerminalKind) -> Option<&Rc<TerminalNode<T>>> {
        self.as_terminal().filter(|terminal| terminal.kind == kind)
    }

    pub fn is_terminal_with_kinds(&self, kinds: &[T::TerminalKind]) -> bool {
        self.as_terminal_with_kinds(kinds).is_some()
    }

    pub fn as_terminal_with_kinds(
        &self,
        kinds: &[T::TerminalKind],
    ) -> Option<&Rc<TerminalNode<T>>> {
        self.as_terminal()
            .filter(|terminal| kinds.contains(&terminal.kind))
    }

    pub fn is_trivia(&self) -> bool {
        match self {
            Self::Nonterminal(_) => false,
            Self::Terminal(terminal) => terminal.kind.is_trivia(),
        }
    }
}

impl<T: KindTypes> From<Rc<NonterminalNode<T>>> for Node<T> {
    fn from(nonterminal: Rc<NonterminalNode<T>>) -> Self {
        Self::Nonterminal(nonterminal)
    }
}

impl<T: KindTypes> From<Rc<TerminalNode<T>>> for Node<T> {
    fn from(terminal: Rc<TerminalNode<T>>) -> Self {
        Self::Terminal(terminal)
    }
}

impl<T: KindTypes> NonterminalNode<T> {
    /// Returns a unique identifier of the node. It is not reproducable over parses
    /// and cannot be used in a persistent/serialised sense.
    pub fn id(self: &Rc<Self>) -> usize {
        Rc::as_ptr(self) as usize
    }

    /// Returns the list of child edges directly connected to this node.
    pub fn children(self: &Rc<Self>) -> &[Edge<T>] {
        &self.children
    }

    /// Returns an iterator over all descendants of the current node in pre-order traversal.
    pub fn descendants(self: Rc<Self>) -> CursorIterator<T> {
        Node::Nonterminal(self).descendants()
    }

    /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    pub fn cursor_with_offset(self: Rc<Self>, text_offset: TextIndex) -> Cursor<T> {
        Node::Nonterminal(self).cursor_with_offset(text_offset)
    }

    /// Reconstructs the original source code from the node and its sub-tree.
    pub fn unparse(&self) -> String {
        fn aux(parent: &NonterminalNode<impl KindTypes>, buffer: &mut String) {
            for child in &parent.children {
                match &child.node {
                    Node::Nonterminal(node) => aux(node, buffer),
                    Node::Terminal(node) => buffer.push_str(&node.text),
                };
            }
        }

        let mut buffer = String::with_capacity(self.text_len.utf8);
        aux(self, &mut buffer);
        buffer
    }
}

impl<T: KindTypes> TerminalNode<T> {
    /// Returns a unique identifier of the node. It is not reproducable over parses
    /// and cannot be used in a persistent/serialised sense.
    pub fn id(self: &Rc<Self>) -> usize {
        Rc::as_ptr(self) as usize
    }

    /// Returns the list of child edges directly connected to this node.
    pub fn children(self: &Rc<Self>) -> &[Edge<T>] {
        &[]
    }

    /// Returns an iterator over all descendants of the current node in pre-order traversal.
    pub fn descendants(self: Rc<Self>) -> CursorIterator<T> {
        Node::Terminal(self).descendants()
    }

    /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    pub fn cursor_with_offset(self: Rc<Self>, text_offset: TextIndex) -> Cursor<T> {
        Node::Terminal(self).cursor_with_offset(text_offset)
    }

    /// Reconstructs the original source code from the node and its sub-tree.
    pub fn unparse(&self) -> String {
        self.text.clone()
    }
}
