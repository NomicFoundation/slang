//! A cursor that can traverse a CST in a DFS pre-order fashion.

use std::rc::Rc;

use crate::kinds::KindTypes;
use crate::syntax_node::SyntaxNode;
use crate::text_index::{TextIndex, TextRange};

/// A cursor that can traverse a CST.
///
/// Nodes are visited in a DFS pre-order traversal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyntaxCursor<T: KindTypes> {
    /// The node the cursor is currently pointing to.
    node: Rc<SyntaxNode<T>>,
    /// The index of the current child node in the parent's children.
    // Required to go to the next/previous sibling.
    child_index: usize,
    /// Whether the cursor is completed, i.e. at the root node as a result of traversal (or when `complete`d).
    /// If `true`, the cursor cannot be moved.
    is_completed: bool,
}

impl<T: KindTypes> SyntaxCursor<T> {
    pub fn create(node: Rc<SyntaxNode<T>>) -> Self {
        Self {
            node,
            child_index: 0,
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
        let mut parent = Rc::clone(&self.node);
        while let Some(next_parent) = parent.parent() {
            parent = next_parent;
        }
        self.node = parent;
        self.is_completed = true;
    }

    /// Unlike [`clone`][`Self::clone`] this re-roots at the current node; however,
    /// it does preserve the correct text offset even though the path is reset.
    #[must_use]
    pub fn spawn(&self) -> Self {
        Self {
            is_completed: false,
            node: self.node.erase_root(),
            child_index: 0,
        }
    }

    /// Whether the cursor can be moved.
    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    /// Returns the currently pointed to [`Node`].
    pub fn node(&self) -> Rc<SyntaxNode<T>> {
        self.node.clone()
    }

    /// Returns the text offset that corresponds to the beginning of the currently pointed to node.
    pub fn text_offset(&self) -> TextIndex {
        self.node.text_offset()
    }

    /// Returns the text range that corresponds to the currently pointed to node.
    pub fn text_range(&self) -> TextRange {
        let start = self.text_offset();
        let end = start + self.node.text_len();
        start..end
    }

    /// Returns the depth of the current node in the CST, i.e. the number of ancestors.
    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut parent = &self.node;
        while let Some(next_parent) = parent.parent_ref() {
            depth += 1;
            parent = next_parent;
        }

        depth
    }

    /// Returns the list of child edges directly connected to this node.
    pub fn children(&self) -> Vec<Rc<SyntaxNode<T>>> {
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
        let current = self.node.clone();

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
        if let Some(parent) = self.node.parent() {
            self.node = parent;

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
        let mut children = self.node.children();
        if !children.is_empty() {
            self.node = children.remove(0);
            self.child_index = 0;

            return true;
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

        let mut children = self.node.children();
        if children.is_empty() {
            return false;
        }

        self.child_index = children.len() - 1;
        self.node = children.remove(self.child_index);

        true
    }

    /// Attempts to go to current node's nth child.
    ///
    /// Returns `false` if the cursor is finished or there's no child to go to.
    pub fn go_to_nth_child(&mut self, child_index: usize) -> bool {
        if self.is_completed {
            return false;
        }

        let mut children = self.node.children();
        if child_index + 1 > children.len() {
            return false;
        }

        self.node = children.remove(child_index);
        self.child_index = child_index;

        false
    }

    /// Attempts to go to current node's next sibling.
    ///
    /// Returns `false` if the cursor is finished or there's no sibling to go to.
    pub fn go_to_next_sibling(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        let Some(parent) = self.node.parent() else {
            return false;
        };

        let mut siblings = parent.children();
        let index = siblings
            .iter()
            .position(|parent_child| *parent_child == self.node)
            .expect("did not find node in parent's children");
        if index + 1 >= siblings.len() {
            return false;
        }
        self.child_index = index + 1;
        self.node = siblings.remove(index + 1);

        true
    }

    /// Attempts to go to current node's previous sibling.
    ///
    /// Returns `false` if the cursor is finished or there's no sibling to go to.
    pub fn go_to_previous_sibling(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        let Some(parent) = self.node.parent() else {
            return false;
        };

        let mut siblings = parent.children();
        let index = siblings
            .iter()
            .position(|parent_child| *parent_child == self.node)
            .expect("did not find node in parent's children");
        if index == 0 {
            return false;
        }
        self.child_index = index - 1;
        self.node = siblings.remove(index - 1);

        true
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

    fn go_to_next_matching(&mut self, pred: impl Fn(&SyntaxNode<T>) -> bool) -> bool {
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
    cursor: SyntaxCursor<T>,
}

impl<T: KindTypes> Iterator for CursorIterator<T> {
    type Item = Rc<SyntaxNode<T>>;

    /// Returns the next edge in the iteration, or `None` if there are no more edges.
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_completed() {
            return None;
        }

        let current = Rc::clone(&self.cursor.node);

        self.cursor.go_to_next();

        Some(current)
    }
}

/// Iterator over all ancestors of the current node, starting with the immediate parent, and moving upwards, ending with the root node.
pub struct AncestorsIterator<T: KindTypes> {
    current: Rc<SyntaxNode<T>>,
}

impl<T: KindTypes> Iterator for AncestorsIterator<T> {
    type Item = Rc<SyntaxNode<T>>;

    /// Returns the next nonterminal node in the iteration, or `None` if there are no more nodes.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(parent) = self.current.parent() {
            self.current = Rc::clone(&parent);
            Some(parent)
        } else {
            None
        }
    }
}
