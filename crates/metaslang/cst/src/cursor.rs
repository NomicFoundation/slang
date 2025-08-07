//! A cursor that can traverse a CST in a DFS pre-order fashion.

use std::rc::Rc;

use crate::kinds::KindTypes;
use crate::nodes::{Edge, Node, NonterminalNode};
use crate::text_index::{TextIndex, TextRange};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Parent<T: KindTypes> {
    None,
    Disconnected(T::EdgeLabel),
    Connected(Rc<PathAncestor<T>>),
}

/// A node in the ancestor path of a [`Cursor`].
#[derive(Clone, Debug, PartialEq, Eq)]
struct PathAncestor<T: KindTypes> {
    parent: Parent<T>,
    nonterminal_node: Rc<NonterminalNode<T>>,
    child_index: usize,
    text_offset: TextIndex,
}

/// A cursor that can traverse a CST.
///
/// Nodes are visited in a DFS pre-order traversal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cursor<T: KindTypes> {
    /// The parent path of this cursor
    parent: Parent<T>,
    /// The node the cursor is currently pointing to.
    node: Node<T>,
    /// The index of the current child node in the parent's children.
    // Required to go to the next/previous sibling.
    child_index: usize,
    /// Text offset that corresponds to the beginning of the currently pointed to node.
    text_offset: TextIndex,
    /// Whether the cursor is completed, i.e. at the root node as a result of traversal (or when `complete`d).
    /// If `true`, the cursor cannot be moved.
    is_completed: bool,
}

impl<T: KindTypes> Cursor<T> {
    fn as_ancestor_node(&self) -> Option<Rc<PathAncestor<T>>> {
        if let Node::<T>::Nonterminal(nonterminal_node) = &self.node {
            Some(Rc::new(PathAncestor {
                parent: self.parent.clone(),
                nonterminal_node: Rc::clone(nonterminal_node),
                child_index: self.child_index,
                text_offset: self.text_offset,
            }))
        } else {
            None
        }
    }

    fn set_from_ancestor_node(&mut self, ancestor: &Rc<PathAncestor<T>>) {
        self.parent = ancestor.parent.clone();
        self.node = Node::<T>::Nonterminal(Rc::clone(&ancestor.nonterminal_node));
        self.child_index = ancestor.child_index;
        self.text_offset = ancestor.text_offset;
    }
}

impl<T: KindTypes> Cursor<T> {
    pub(crate) fn create(node: Node<T>, text_offset: TextIndex) -> Self {
        Self {
            parent: Parent::None,
            node,
            child_index: 0,
            text_offset,
            is_completed: false,
        }
    }

    /// Resets the cursor to the root node.
    pub fn reset(&mut self) {
        self.complete();
        self.is_completed = false;
    }

    /// Completes the cursor, setting it to the root node.
    pub fn complete(&mut self) {
        if let Parent::Connected(parent) = &self.parent.clone() {
            let mut parent = parent;
            while let Parent::Connected(grandparent) = &parent.parent {
                parent = grandparent;
            }
            self.set_from_ancestor_node(parent);
        }
        self.is_completed = true;
    }

    /// Unlike [`clone`][`Self::clone`] this re-roots at the current node; however,
    /// it does preserve the correct text offset even though the path is reset.
    #[must_use]
    pub fn spawn(&self) -> Self {
        let label = self.label();
        Self {
            is_completed: false,
            parent: Parent::Disconnected(label),
            node: self.node.clone(),
            child_index: 0,
            text_offset: self.text_offset,
        }
    }

    /// Whether the cursor can be moved.
    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    /// Returns the currently pointed to [`Node`].
    pub fn node(&self) -> Node<T> {
        self.node.clone()
    }

    /// Returns a label that describes the relationship between the current node and its parent.
    pub fn label(&self) -> T::EdgeLabel {
        match &self.parent {
            Parent::Connected(parent) => {
                let this = &parent.nonterminal_node.children[self.child_index];
                this.label
            }
            Parent::Disconnected(label) => *label,
            Parent::None => T::EdgeLabel::default(),
        }
    }

    /// Returns the text offset that corresponds to the beginning of the currently pointed to node.
    pub fn text_offset(&self) -> TextIndex {
        self.text_offset
    }

    /// Returns the text range that corresponds to the currently pointed to node.
    pub fn text_range(&self) -> TextRange {
        let start = self.text_offset;
        let end = start + self.node.text_len();
        start..end
    }

    /// Returns the depth of the current node in the CST, i.e. the number of ancestors.
    pub fn depth(&self) -> usize {
        let mut depth = 0;
        if let Parent::Connected(parent) = &self.parent {
            let mut parent = parent;
            depth += 1;
            while let Parent::Connected(grandparent) = &parent.parent {
                depth += 1;
                parent = grandparent;
            }
        }

        depth
    }

    /// Returns the list of child edges directly connected to this node.
    pub fn children(&self) -> &[Edge<T>] {
        self.node.children()
    }

    /// Returns an iterator over all descendants of the current node in pre-order traversal.
    pub fn descendants(&self) -> CursorIterator<T> {
        let mut cursor = self.spawn();

        // skip the current node:
        cursor.go_to_next();

        CursorIterator { cursor }
    }

    /// Returns an iterator over all the remaining nodes in the current tree, moving in pre-order traversal, until the tree is completed.
    pub fn remaining_nodes(&self) -> CursorIterator<T> {
        CursorIterator {
            cursor: self.clone(),
        }
    }

    /// Returns an iterator over all ancestors of the current node, starting with the immediate parent, and moving upwards, ending with the root node.
    pub fn ancestors(&self) -> AncestorsIterator<T> {
        let current = self.parent.clone();

        AncestorsIterator { current }
    }

    /// Attempts to go to current node's next one, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        self.go_to_first_child() || self.go_to_next_non_descendant()
    }

    /// Attempts to go to current node's next non-descendant.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_non_descendant(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        while !self.go_to_next_sibling() {
            if !self.go_to_parent() {
                return false;
            }
        }

        true
    }

    /// Attempts to go to current node's previous one, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_previous(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        while !self.go_to_previous_sibling() {
            if !self.go_to_parent() {
                return false;
            }
        }

        while self.go_to_last_child() {}

        true
    }

    /// Attempts to go to current node's parent.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_parent(&mut self) -> bool {
        if let Parent::Connected(parent) = &self.parent.clone() {
            self.set_from_ancestor_node(parent);

            true
        } else {
            self.is_completed = true;

            false
        }
    }

    /// Attempts to go to current node's first child.
    ///
    /// Returns `false` if the cursor is finished or there's no child to go to.
    pub fn go_to_first_child(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        // If the current cursor is a node and it has children, go to first children
        if let Some(new_parent) = self.as_ancestor_node() {
            if let Some(new_child) = new_parent.nonterminal_node.children.first().cloned() {
                self.parent = Parent::Connected(new_parent);
                self.node = new_child.node;
                self.child_index = 0;

                return true;
            }
        }

        false
    }

    /// Attempts to go to current node's last child.
    ///
    /// Returns `false` if the cursor is finished or there's no child to go to.
    pub fn go_to_last_child(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        if let Some(new_parent) = self.as_ancestor_node() {
            if let Some(new_child) = new_parent.nonterminal_node.children.last().cloned() {
                self.child_index = new_parent.nonterminal_node.children.len() - 1;
                self.node = new_child.node;
                // Remember: range is not inclusive
                for sibling in &new_parent.nonterminal_node.children[..self.child_index] {
                    self.text_offset += sibling.text_len();
                }
                self.parent = Parent::Connected(new_parent);

                return true;
            }
        }

        false
    }

    /// Attempts to go to current node's nth child.
    ///
    /// Returns `false` if the cursor is finished or there's no child to go to.
    pub fn go_to_nth_child(&mut self, child_index: usize) -> bool {
        if self.is_completed {
            return false;
        }

        if let Some(new_parent) = self.as_ancestor_node() {
            if let Some(new_child) = new_parent
                .nonterminal_node
                .children
                .get(child_index)
                .cloned()
            {
                self.child_index = child_index;
                self.node = new_child.node;
                // Remember: range is not inclusive
                for sibling in &new_parent.nonterminal_node.children[..self.child_index] {
                    self.text_offset += sibling.text_len();
                }
                self.parent = Parent::Connected(new_parent);

                return true;
            }
        }

        false
    }

    /// Attempts to go to current node's next sibling.
    ///
    /// Returns `false` if the cursor is finished or there's no sibling to go to.
    pub fn go_to_next_sibling(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        if let Parent::Connected(parent) = &self.parent {
            let new_child_number = self.child_index + 1;
            if let Some(new_child) = parent.nonterminal_node.children.get(new_child_number) {
                self.text_offset += self.node.text_len();
                self.child_index = new_child_number;
                self.node = new_child.node.clone();

                return true;
            }
        }

        false
    }

    /// Attempts to go to current node's previous sibling.
    ///
    /// Returns `false` if the cursor is finished or there's no sibling to go to.
    pub fn go_to_previous_sibling(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        if let Parent::Connected(parent) = &self.parent {
            if self.child_index > 0 {
                let new_child_number = self.child_index - 1;
                let new_child = &parent.nonterminal_node.children[new_child_number];
                self.child_index = new_child_number;
                self.node = new_child.node.clone();
                // Remember: range is not inclusive
                self.text_offset = parent.text_offset;
                for sibling in &parent.nonterminal_node.children[..self.child_index] {
                    self.text_offset += sibling.text_len();
                }

                return true;
            }
        }

        false
    }

    /// Attempts to go to the next terminal node, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_terminal(&mut self) -> bool {
        self.go_to_next_matching(|node| node.is_terminal())
    }

    /// Attempts to go to the next terminal node with the given kind, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_terminal_with_kind(&mut self, kind: T::TerminalKind) -> bool {
        self.go_to_next_matching(|node| node.is_terminal_with_kind(kind))
    }

    /// Attempts to go to the next terminal node with any of the given kinds, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_terminal_with_kinds(&mut self, kinds: &[T::TerminalKind]) -> bool {
        self.go_to_next_matching(|node| node.is_terminal_with_kinds(kinds))
    }

    /// Attempts to go to the next nonterminal node, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_nonterminal(&mut self) -> bool {
        self.go_to_next_matching(|node| node.is_nonterminal())
    }

    /// Attempts to go to the next nonterminal node with the given kind, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_nonterminal_with_kind(&mut self, kind: T::NonterminalKind) -> bool {
        self.go_to_next_matching(|node| node.is_nonterminal_with_kind(kind))
    }

    /// Attempts to go to the next nonterminal node with any of the given kinds, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_nonterminal_with_kinds(&mut self, kinds: &[T::NonterminalKind]) -> bool {
        self.go_to_next_matching(|node| node.is_nonterminal_with_kinds(kinds))
    }

    fn go_to_next_matching(&mut self, pred: impl Fn(&Node<T>) -> bool) -> bool {
        while self.go_to_next() {
            if pred(&self.node) {
                return true;
            }
        }

        false
    }
}

/// Iterator over all the remaining nodes in the current tree, moving in pre-order traversal, until the tree is completed.
pub struct CursorIterator<T: KindTypes> {
    cursor: Cursor<T>,
}

impl<T: KindTypes> Iterator for CursorIterator<T> {
    type Item = Edge<T>;

    /// Returns the next edge in the iteration, or `None` if there are no more edges.
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_completed() {
            return None;
        }

        let current = Edge {
            label: self.cursor.label(),
            node: self.cursor.node(),
        };

        self.cursor.go_to_next();

        Some(current)
    }
}

/// Iterator over all ancestors of the current node, starting with the immediate parent, and moving upwards, ending with the root node.
pub struct AncestorsIterator<T: KindTypes> {
    current: Parent<T>,
}

impl<T: KindTypes> Iterator for AncestorsIterator<T> {
    type Item = Rc<NonterminalNode<T>>;

    /// Returns the next nonterminal node in the iteration, or `None` if there are no more nodes.
    fn next(&mut self) -> Option<Self::Item> {
        if let Parent::Connected(current) = self.current.clone() {
            self.current = current.parent.clone();
            Some(Rc::clone(&current.nonterminal_node))
        } else {
            None
        }
    }
}
