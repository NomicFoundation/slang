use std::rc::Rc;

use crate::{
    kinds::{KindTypes, NodeKind},
    nodes::{Node, NodeId, NonterminalNode, TerminalNode},
    text_index::TextIndex,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxNode<T: KindTypes> {
    parent: Option<Rc<SyntaxNode<T>>>,
    label: T::EdgeLabel,
    text_offset: TextIndex,
    green: Node<T>,
}

impl<T: KindTypes> SyntaxNode<T> {
    pub fn create_root(node: Node<T>) -> Rc<Self> {
        Rc::new(Self {
            parent: None,
            label: T::EdgeLabel::default(),
            text_offset: TextIndex::ZERO,
            green: node,
        })
    }

    pub(crate) fn erase_root(&self) -> Rc<Self> {
        Rc::new(Self {
            parent: None,
            label: self.label,
            text_offset: self.text_offset,
            green: self.green.clone(),
        })
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

    pub fn text_offset(&self) -> TextIndex {
        self.text_offset
    }

    /// Returns the list of child edges directly connected to this node.
    pub fn children(self: &Rc<Self>) -> Vec<Rc<Self>> {
        let mut children = Vec::with_capacity(self.green.children().len());
        let mut text_offset = self.text_offset;
        for edge in self.green.children() {
            children.push(Rc::new(SyntaxNode {
                parent: Some(Rc::clone(self)),
                label: edge.label,
                text_offset,
                green: edge.node.clone(),
            }));
            text_offset += edge.node.text_len();
        }
        children
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
}
