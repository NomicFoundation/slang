//! A cursor that can traverse a CST in a DFS pre-order fashion.

use std::rc::Rc;

use crate::kinds::KindTypes;
use crate::nodes::{Edge, Node, NonterminalNode};
use crate::text_index::{TextIndex, TextRange};

/// A node in the ancestor path of a [`Cursor`].
#[derive(Clone, Debug, PartialEq, Eq)]
struct PathAncestor<T: KindTypes> {
    parent: Option<Rc<PathAncestor<T>>>,
    nonterminal_node: Rc<NonterminalNode<T>>,
    child_number: usize,
    text_offset: TextIndex,
}

/// A cursor that can traverse a CST.
///
/// Nodes are visited in a DFS pre-order traversal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cursor<T: KindTypes> {
    /// The parent path of this cursor
    parent: Option<Rc<PathAncestor<T>>>,
    /// The node the cursor is currently pointing to.
    node: Node<T>,
    /// The index of the current child node in the parent's children.
    // Required to go to the next/previous sibling.
    child_number: usize,
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
                child_number: self.child_number,
                text_offset: self.text_offset,
            }))
        } else {
            None
        }
    }

    fn set_from_ancestor_node(&mut self, ancestor: &Rc<PathAncestor<T>>) {
        self.parent = ancestor.parent.clone();
        self.node = Node::<T>::Nonterminal(Rc::clone(&ancestor.nonterminal_node));
        self.child_number = ancestor.child_number;
        self.text_offset = ancestor.text_offset;
    }
}

impl<T: KindTypes> Iterator for Cursor<T> {
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_completed {
            None
        } else {
            let cur = self.node();
            self.go_to_next();

            Some(cur)
        }
    }
}

impl<T: KindTypes> Cursor<T> {
    pub(crate) fn new(node: Node<T>, text_offset: TextIndex) -> Self {
        Self {
            parent: None,
            node,
            child_number: 0,
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
        if let Some(parent) = &self.parent.clone() {
            let mut parent = parent;
            while let Some(grandparent) = &parent.parent {
                parent = grandparent;
            }
            self.set_from_ancestor_node(parent);
        }
        self.is_completed = true;
    }

    /// Unlike `clone`, this re-roots at the current node.
    /// It does preserve the correct text offset however,
    /// even though the path is reset.
    #[must_use]
    pub fn spawn(&self) -> Self {
        Self {
            is_completed: false,
            parent: None,
            node: self.node.clone(),
            child_number: 0,
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

    pub fn label(&self) -> Option<T::EdgeLabel> {
        self.parent.as_ref().and_then(|parent| {
            let this = &parent.nonterminal_node.children[self.child_number];

            this.label
        })
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
        if let Some(parent) = &self.parent {
            let mut parent = parent;
            depth += 1;
            while let Some(grandparent) = &parent.parent {
                depth += 1;
                parent = grandparent;
            }
        }

        depth
    }

    /// Returns an iterator over the current node's ancestors, starting from the parent of the current node.
    pub fn ancestors(&self) -> impl Iterator<Item = Rc<NonterminalNode<T>>> {
        struct Iter<T: KindTypes> {
            a: Option<Rc<PathAncestor<T>>>,
        }
        impl<T: KindTypes> Iterator for Iter<T> {
            type Item = Rc<NonterminalNode<T>>;

            fn next(&mut self) -> Option<Self::Item> {
                if let Some(a) = self.a.take() {
                    self.a = a.parent.clone();
                    Some(Rc::clone(&a.nonterminal_node))
                } else {
                    None
                }
            }
        }
        Iter {
            a: self.parent.clone(),
        }
    }

    /// Attempts to go to current node's next one, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        self.go_to_first_child() || self.go_to_next_non_descendent()
    }

    /// Attempts to go to current node's next non-descendent.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_non_descendent(&mut self) -> bool {
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
        if let Some(parent) = &self.parent.clone() {
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
                self.parent = Some(new_parent);
                self.node = new_child.node;
                self.child_number = 0;

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
                self.child_number = new_parent.nonterminal_node.children.len() - 1;
                self.node = new_child.node;
                // Remember: range is not inclusive
                for sibling in &new_parent.nonterminal_node.children[..self.child_number] {
                    self.text_offset += sibling.text_len();
                }
                self.parent = Some(new_parent);

                return true;
            }
        }

        false
    }

    /// Attempts to go to current node's nth child.
    ///
    /// Returns `false` if the cursor is finished or there's no child to go to.
    pub fn go_to_nth_child(&mut self, child_number: usize) -> bool {
        if self.is_completed {
            return false;
        }

        if let Some(new_parent) = self.as_ancestor_node() {
            if let Some(new_child) = new_parent
                .nonterminal_node
                .children
                .get(child_number)
                .cloned()
            {
                self.child_number = child_number;
                self.node = new_child.node;
                // Remember: range is not inclusive
                for sibling in &new_parent.nonterminal_node.children[..self.child_number] {
                    self.text_offset += sibling.text_len();
                }
                self.parent = Some(new_parent);

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

        if let Some(parent) = &self.parent {
            let new_child_number = self.child_number + 1;
            if let Some(new_child) = parent.nonterminal_node.children.get(new_child_number) {
                self.text_offset += self.node.text_len();
                self.child_number = new_child_number;
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

        if let Some(parent) = &self.parent {
            if self.child_number > 0 {
                let new_child_number = self.child_number - 1;
                let new_child = &parent.nonterminal_node.children[new_child_number];
                self.child_number = new_child_number;
                self.node = new_child.node.clone();
                // Remember: range is not inclusive
                self.text_offset = parent.text_offset;
                for sibling in &parent.nonterminal_node.children[..self.child_number] {
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

/// A [`Cursor`] that also keeps track of the labels of the nodes it visits.
pub struct CursorWithEdges<T: KindTypes> {
    cursor: Cursor<T>,
}

impl<T: KindTypes> CursorWithEdges<T> {
    pub fn without_edges(self) -> Cursor<T> {
        self.cursor
    }
}

impl<T: KindTypes> Iterator for CursorWithEdges<T> {
    type Item = Edge<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let label = self.cursor.label();

        self.cursor.next().map(|node| Self::Item { label, node })
    }
}

impl<T: KindTypes> std::ops::Deref for CursorWithEdges<T> {
    type Target = Cursor<T>;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

impl<T: KindTypes> std::ops::DerefMut for CursorWithEdges<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cursor
    }
}

impl<T: KindTypes> Cursor<T> {
    /// Returns a [`CursorWithEdges`] that wraps this cursor.
    pub fn with_edges(self) -> CursorWithEdges<T> {
        CursorWithEdges::<T>::from(self)
    }
}

impl<T: KindTypes> From<Cursor<T>> for CursorWithEdges<T> {
    fn from(cursor: Cursor<T>) -> Self {
        CursorWithEdges::<T> { cursor }
    }
}
