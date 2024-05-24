use std::rc::Rc;

use serde::Serialize;

use crate::cursor::Cursor;
use crate::text_index::TextIndex;
use crate::{AbstractKind as _, KindTypes, TerminalKind as _};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NodeKind<T: KindTypes> {
    NonTerminal(T::NonTerminalKind),
    Terminal(T::TerminalKind),
}

impl<T: KindTypes> From<NodeKind<T>> for &'static str {
    fn from(val: NodeKind<T>) -> Self {
        match val {
            NodeKind::NonTerminal(t) => t.as_static_str(),
            NodeKind::Terminal(t) => t.as_static_str(),
        }
    }
}

impl<T: KindTypes> std::fmt::Display for NodeKind<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::NonTerminal(t) => {
                write!(f, "{}", t.as_static_str())
            }
            NodeKind::Terminal(t) => {
                write!(f, "{}", t.as_static_str())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TerminalNode<T: KindTypes> {
    pub kind: T::TerminalKind,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NonTerminalNode<T: KindTypes> {
    pub kind: T::NonTerminalKind,
    pub text_len: TextIndex,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Edge<T>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node<T: KindTypes> {
    NonTerminal(Rc<NonTerminalNode<T>>),
    Terminal(Rc<TerminalNode<T>>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Edge<T: KindTypes> {
    pub label: Option<T::EdgeLabel>,
    pub node: Node<T>,
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
    pub fn nonterminal(kind: T::NonTerminalKind, children: Vec<Edge<T>>) -> Self {
        let text_len = children.iter().map(|node| node.text_len()).sum();

        Self::NonTerminal(Rc::new(NonTerminalNode {
            kind,
            text_len,
            children,
        }))
    }

    pub fn terminal(kind: T::TerminalKind, text: String) -> Self {
        Self::Terminal(Rc::new(TerminalNode { kind, text }))
    }

    /// Returns a unique identifier of the node. It is not reproducable over parses
    /// and cannot be used in a persistent/serialised sense.
    pub fn id(&self) -> usize {
        match self {
            Self::NonTerminal(node) => Rc::as_ptr(node) as usize,
            Self::Terminal(node) => Rc::as_ptr(node) as usize,
        }
    }

    pub fn kind(&self) -> NodeKind<T> {
        match self {
            Self::NonTerminal(node) => NodeKind::NonTerminal(node.kind),
            Self::Terminal(node) => NodeKind::Terminal(node.kind),
        }
    }

    pub fn text_len(&self) -> TextIndex {
        match self {
            Self::NonTerminal(node) => node.text_len,
            Self::Terminal(node) => (&node.text).into(),
        }
    }

    /// Returns a slice of the edges (not all descendants) leaving this node.
    pub fn edges(&self) -> &[Edge<T>] {
        match self {
            Self::NonTerminal(node) => &node.children,
            Self::Terminal(_) => &[],
        }
    }

    pub fn labeled_edges(&self) -> impl Iterator<Item = &Edge<T>> {
        self.edges().iter().filter(|edge| edge.label.is_some())
    }

    /// Returns a slice of the children (not all descendants) of this node.
    pub fn children(&self) -> &[Edge<T>] {
        self.edges()
    }

    /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    pub fn cursor_with_offset(&self, text_offset: TextIndex) -> Cursor<T> {
        Cursor::new(self.clone(), text_offset)
    }

    /// Reconstructs the original source code from the parse tree.
    pub fn unparse(self) -> String {
        match self {
            Self::NonTerminal(nonterminal) => nonterminal.unparse(),
            Self::Terminal(terminal) => terminal.text.clone(),
        }
    }

    pub fn into_nonterminal(self) -> Option<Rc<NonTerminalNode<T>>> {
        match self {
            Self::NonTerminal(nonterminal) => Some(nonterminal),
            Self::Terminal(..) => None,
        }
    }

    pub fn is_nonterminal(&self) -> bool {
        self.as_nonterminal().is_some()
    }

    pub fn as_nonterminal(&self) -> Option<&Rc<NonTerminalNode<T>>> {
        match self {
            Self::NonTerminal(nonterminal) => Some(nonterminal),
            Self::Terminal(..) => None,
        }
    }

    pub fn is_nonterminal_with_kind(&self, kind: T::NonTerminalKind) -> bool {
        self.as_nonterminal_with_kind(kind).is_some()
    }

    pub fn as_nonterminal_with_kind(
        &self,
        kind: T::NonTerminalKind,
    ) -> Option<&Rc<NonTerminalNode<T>>> {
        self.as_nonterminal().filter(|node| node.kind == kind)
    }

    pub fn is_nonterminal_with_kinds(&self, kinds: &[T::NonTerminalKind]) -> bool {
        self.as_nonterminal_with_kinds(kinds).is_some()
    }

    pub fn as_nonterminal_with_kinds(
        &self,
        kinds: &[T::NonTerminalKind],
    ) -> Option<&Rc<NonTerminalNode<T>>> {
        self.as_nonterminal()
            .filter(|nonterminal| kinds.contains(&nonterminal.kind))
    }

    pub fn into_terminal(self) -> Option<Rc<TerminalNode<T>>> {
        match self {
            Self::Terminal(terminal) => Some(terminal),
            Self::NonTerminal(..) => None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.as_terminal().is_some()
    }

    pub fn as_terminal(&self) -> Option<&Rc<TerminalNode<T>>> {
        match self {
            Self::Terminal(terminal) => Some(terminal),
            Self::NonTerminal(..) => None,
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
            Self::NonTerminal(_) => false,
            Self::Terminal(terminal) => terminal.kind.is_trivia(),
        }
    }
}

impl<T: KindTypes> From<Rc<NonTerminalNode<T>>> for Node<T> {
    fn from(nonterminal: Rc<NonTerminalNode<T>>) -> Self {
        Self::NonTerminal(nonterminal)
    }
}

impl<T: KindTypes> From<Rc<TerminalNode<T>>> for Node<T> {
    fn from(terminal: Rc<TerminalNode<T>>) -> Self {
        Self::Terminal(terminal)
    }
}

impl<T: KindTypes> NonTerminalNode<T> {
    /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    pub fn cursor_with_offset(self: Rc<Self>, text_offset: TextIndex) -> Cursor<T> {
        Cursor::new(Node::NonTerminal(self), text_offset)
    }

    /// Reconstructs the original source code from the parse tree.
    pub fn unparse(self: Rc<Self>) -> String {
        let acc = String::with_capacity(self.text_len.utf8);

        self.cursor_with_offset(TextIndex::ZERO)
            .filter_map(|node| node.into_terminal())
            .fold(acc, |mut acc, terminal| {
                acc.push_str(&terminal.text);
                acc
            })
    }
}
