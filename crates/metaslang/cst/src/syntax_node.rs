use std::rc::Rc;

use crate::cursor::Cursor;
use crate::kinds::{KindTypes, NodeKind};
use crate::nodes::{Node, NodeId, NonterminalNode, TerminalNode};
use crate::text_index::TextIndex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxNode<T: KindTypes> {
    parent: Option<Rc<SyntaxNode<T>>>,
    label: T::EdgeLabel,
    green: Node<T>,
}

impl<T: KindTypes> SyntaxNode<T> {
    pub fn create_root(node: Node<T>) -> Rc<Self> {
        Rc::new(Self {
            parent: None,
            label: T::EdgeLabel::default(),
            green: node,
        })
    }

    pub(crate) fn erase_root(self: &Rc<Self>) -> Rc<Self> {
        if self.parent.is_some() {
            Rc::new(Self {
                parent: None,
                label: self.label,
                green: self.green.clone(),
            })
        } else {
            Rc::clone(self)
        }
    }

    /// Returns a unique identifier of the node. It is not reproducible over parses
    /// and cannot be used in a persistent/serialised sense.
    pub fn id(&self) -> NodeId {
        self.green.id()
    }

    pub fn parent(&self) -> Option<Rc<SyntaxNode<T>>> {
        self.parent.as_ref().map(Rc::clone)
    }

    pub fn parent_ref(&self) -> Option<&Rc<SyntaxNode<T>>> {
        self.parent.as_ref()
    }

    /// The kind of the node.
    pub fn kind(&self) -> NodeKind<T> {
        self.green.kind()
    }

    pub fn label(&self) -> T::EdgeLabel {
        self.label
    }

    /// The length of the node, as a `TextIndex`.
    pub fn text_len(&self) -> TextIndex {
        self.green.text_len()
    }

    /// Returns the list of child edges directly connected to this node.
    pub fn children(self: &Rc<Self>) -> Vec<Rc<Self>> {
        let mut children = Vec::with_capacity(self.green.children().len());
        for edge in self.green.children() {
            children.push(Rc::new(SyntaxNode {
                parent: Some(Rc::clone(self)),
                label: edge.label,
                green: edge.node.clone(),
            }));
        }
        children
    }

    /// Returns the number of child nodes
    #[inline]
    pub fn children_count(&self) -> usize {
        self.green.children().len()
    }

    /// Returns the label of the nth child. Panics if the given index is out of bounds.
    #[inline]
    pub fn child_label(&self, index: usize) -> T::EdgeLabel {
        self.green.children()[index].label
    }

    /// Check that the nth child is a valid node and is not trivia
    #[inline]
    pub fn child_is_valid_and_not_trivia(&self, index: usize) -> bool {
        let node = &self.green.children()[index].node;
        node.is_valid() && !node.is_trivia()
    }

    /// Returns the `SyntaxNode` for the nth-child
    #[inline]
    pub fn nth_child(self: &Rc<Self>, index: usize) -> Rc<Self> {
        let edge = &self.green.children()[index];
        Rc::new(SyntaxNode {
            parent: Some(Rc::clone(self)),
            label: edge.label,
            green: edge.node.clone(),
        })
    }

    pub fn child_index(&self, node_id: NodeId) -> Option<usize> {
        self.green
            .children()
            .iter()
            .position(|edge| edge.node.id() == node_id)
    }

    pub fn index_in_parent(&self) -> Option<usize> {
        self.parent
            .as_ref()
            .and_then(|parent| parent.child_index(self.id()))
    }

    /// Reconstructs the original source code from the node and its sub-tree.
    pub fn unparse(&self) -> String {
        self.green.unparse()
    }

    /// Converts the node into a `NonterminalNode`, if possible.
    pub fn into_nonterminal(self) -> Option<Rc<NonterminalNode<T>>> {
        self.green.into_nonterminal()
    }

    /// Returns if the node is a nonterminal.
    pub fn is_nonterminal(&self) -> bool {
        self.as_nonterminal().is_some()
    }

    /// Converts the node into a `NonterminalNode`, if possible.
    pub fn as_nonterminal(&self) -> Option<&Rc<NonterminalNode<T>>> {
        self.green.as_nonterminal()
    }

    /// Returns if this node is a nonterminal with the given nonterminal kind.
    pub fn is_nonterminal_with_kind(&self, kind: T::NonterminalKind) -> bool {
        self.as_nonterminal_with_kind(kind).is_some()
    }

    /// Returns the `NonterminalNode` with the specific `kind`, or `None` if it is not a nonterminal or it doesn't have
    /// the specific kind.
    pub fn as_nonterminal_with_kind(
        &self,
        kind: T::NonterminalKind,
    ) -> Option<&Rc<NonterminalNode<T>>> {
        self.as_nonterminal().filter(|node| node.kind == kind)
    }

    /// Returns if this node is a nonterminal with the given nonterminal kinds.
    pub fn is_nonterminal_with_kinds(&self, kinds: &[T::NonterminalKind]) -> bool {
        self.as_nonterminal_with_kinds(kinds).is_some()
    }

    /// Returns the `NonterminalNode` with the specific `kinds`, or `None` if it is not a nonterminal or it doesn't
    /// have any of the specific kinds.
    pub fn as_nonterminal_with_kinds(
        &self,
        kinds: &[T::NonterminalKind],
    ) -> Option<&Rc<NonterminalNode<T>>> {
        self.as_nonterminal()
            .filter(|nonterminal| kinds.contains(&nonterminal.kind))
    }

    /// Returns the node as a `TerminalNode`, if it's a terminal node.
    pub fn into_terminal(self) -> Option<Rc<TerminalNode<T>>> {
        self.green.into_terminal()
    }

    /// Returns if the node is a terminal node.
    pub fn is_terminal(&self) -> bool {
        self.as_terminal().is_some()
    }

    /// Returns the node as a `TerminalNode`, if it's a terminal node.
    pub fn as_terminal(&self) -> Option<&Rc<TerminalNode<T>>> {
        self.green.as_terminal()
    }

    /// Returns if this node is a terminal node with the given terminal kind.
    pub fn is_terminal_with_kind(&self, kind: T::TerminalKind) -> bool {
        self.as_terminal_with_kind(kind).is_some()
    }

    /// Returns the `TerminalNode` with the specific `kind`, or `None` if it is a nonterminal node or it doesn't
    /// have the specific kind.
    pub fn as_terminal_with_kind(&self, kind: T::TerminalKind) -> Option<&Rc<TerminalNode<T>>> {
        self.as_terminal().filter(|terminal| terminal.kind == kind)
    }

    /// Returns if this node is a terminal node with the given terminal kinds.
    pub fn is_terminal_with_kinds(&self, kinds: &[T::TerminalKind]) -> bool {
        self.as_terminal_with_kinds(kinds).is_some()
    }

    /// Returns the `TerminalNode` with the specific `kinds`, or `None` if it is a nonterminal node or it doesn't
    /// have any of the specific kinds.
    pub fn as_terminal_with_kinds(
        &self,
        kinds: &[T::TerminalKind],
    ) -> Option<&Rc<TerminalNode<T>>> {
        self.as_terminal()
            .filter(|terminal| kinds.contains(&terminal.kind))
    }

    /// Returns if this node is a terminal node with the trivia kind.
    pub fn is_trivia(&self) -> bool {
        self.green.is_trivia()
    }

    /// Returns true if this node is a nonterminal, or if it's a temrinal with a valid kind (that is, any valid token of
    /// the language).
    pub fn is_valid(&self) -> bool {
        self.green.is_valid()
    }

    /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    pub fn create_cursor(&self, text_offset: TextIndex) -> Cursor<T> {
        match &self.green {
            Node::Nonterminal(nonterminal_node) => {
                Rc::clone(nonterminal_node).create_cursor(text_offset)
            }
            Node::Terminal(terminal_node) => Rc::clone(terminal_node).create_cursor(text_offset),
        }
    }
}
