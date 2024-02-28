// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

//! A cursor that can traverse a CST in a DFS pre-order fashion.

use std::rc::Rc;

use crate::cst::{LabeledNode, Node, RuleNode};
use crate::kinds::{NodeLabel, RuleKind, TokenKind};
use crate::text_index::{TextIndex, TextRange};

/// A node in the ancestor path of a [`Cursor`].
#[derive(Clone, Debug, PartialEq, Eq)]
struct PathAncestor {
    parent: Option<Rc<PathAncestor>>,
    rule_node: Rc<RuleNode>,
    child_number: usize,
    text_offset: TextIndex,
}

/// A cursor that can traverse a CST.
///
/// Nodes are visited in a DFS pre-order traversal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cursor {
    /// The parent path of this cursor
    parent: Option<Rc<PathAncestor>>,
    /// The node the cursor is currently pointing to.
    node: Node,
    /// The index of the current child node in the parent's children.
    // Required to go to the next/previous sibling.
    child_number: usize,
    /// Text offset that corresponds to the beginning of the currently pointed to node.
    text_offset: TextIndex,
    /// Whether the cursor is completed, i.e. at the root node as a result of traversal (or when `complete`d).
    /// If `true`, the cursor cannot be moved.
    is_completed: bool,
}

impl Cursor {
    fn as_ancestor_node(&self) -> Option<Rc<PathAncestor>> {
        if let Node::Rule(rule_node) = &self.node {
            Some(Rc::new(PathAncestor {
                parent: self.parent.clone(),
                rule_node: rule_node.clone(),
                child_number: self.child_number,
                text_offset: self.text_offset,
            }))
        } else {
            None
        }
    }

    fn set_from_ancestor_node(&mut self, ancestor: &Rc<PathAncestor>) {
        self.parent = ancestor.parent.clone();
        self.node = Node::Rule(ancestor.rule_node.clone());
        self.child_number = ancestor.child_number;
        self.text_offset = ancestor.text_offset;
    }
}

impl Iterator for Cursor {
    type Item = Node;

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

impl Cursor {
    pub(crate) fn new(node: Node, text_offset: TextIndex) -> Self {
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
    pub fn node(&self) -> Node {
        self.node.clone()
    }

    pub fn label(&self) -> Option<NodeLabel> {
        self.parent.as_ref().and_then(|parent| {
            let this = &parent.rule_node.children[self.child_number];

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
    pub fn ancestors(&self) -> impl Iterator<Item = Rc<RuleNode>> {
        struct Iter {
            a: Option<Rc<PathAncestor>>,
        }
        impl Iterator for Iter {
            type Item = Rc<RuleNode>;

            fn next(&mut self) -> Option<Self::Item> {
                if let Some(a) = self.a.take() {
                    self.a = a.parent.clone();
                    Some(a.rule_node.clone())
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
            if let Some(new_child) = new_parent.rule_node.children.first().cloned() {
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
            if let Some(new_child) = new_parent.rule_node.children.last().cloned() {
                self.child_number = new_parent.rule_node.children.len() - 1;
                // This is cheaper than summing up the length of the children
                self.text_offset += new_parent.rule_node.text_len - new_child.text_len();
                self.node = new_child.node;
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
            if let Some(new_child) = new_parent.rule_node.children.get(child_number).cloned() {
                self.node = new_child.node;
                self.child_number = child_number;
                // Sum up the length of the children before this child
                // TODO: it might sometimes be quicker to start from the end (like `go_to_last_child`)
                self.text_offset += new_parent.rule_node.children[..child_number]
                    .iter()
                    .map(|node| node.text_len())
                    .sum();
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
            if let Some(new_child) = parent.rule_node.children.get(new_child_number) {
                self.text_offset += self.node.text_len();
                self.node = new_child.node.clone();
                self.child_number = new_child_number;

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
                let new_child = &parent.rule_node.children[new_child_number];
                self.text_offset -= new_child.node.text_len();
                self.node = new_child.node.clone();
                self.child_number = new_child_number;

                return true;
            }
        }

        false
    }

    /// Attempts to go to the next token, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_token(&mut self) -> bool {
        self.go_to_next_matching(|node| node.is_token())
    }

    /// Attempts to go to the next token with the given kind, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_token_with_kind(&mut self, kind: TokenKind) -> bool {
        self.go_to_next_matching(|node| node.is_token_with_kind(kind))
    }

    /// Attempts to go to the next token with any of the given kinds, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_token_with_kinds(&mut self, kinds: &[TokenKind]) -> bool {
        self.go_to_next_matching(|node| node.is_token_with_kinds(kinds))
    }

    /// Attempts to go to the next rule, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_rule(&mut self) -> bool {
        self.go_to_next_matching(|node| node.is_rule())
    }

    /// Attempts to go to the next rule with the given kind, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_rule_with_kind(&mut self, kind: RuleKind) -> bool {
        self.go_to_next_matching(|node| node.is_rule_with_kind(kind))
    }

    /// Attempts to go to the next rule with any of the given kinds, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_rule_with_kinds(&mut self, kinds: &[RuleKind]) -> bool {
        self.go_to_next_matching(|node| node.is_rule_with_kinds(kinds))
    }

    fn go_to_next_matching(&mut self, pred: impl Fn(&Node) -> bool) -> bool {
        while self.go_to_next() {
            if pred(&self.node) {
                return true;
            }
        }

        false
    }
}

/// A [`Cursor`] that also keeps track of the labels of the nodes it visits.
pub struct CursorWithLabels {
    cursor: Cursor,
}

impl CursorWithLabels {
    pub fn without_labels(self) -> Cursor {
        self.cursor
    }
}

impl Iterator for CursorWithLabels {
    type Item = LabeledNode;

    fn next(&mut self) -> Option<Self::Item> {
        let label = self.cursor.label();

        self.cursor.next().map(|node| LabeledNode { label, node })
    }
}

impl std::ops::Deref for CursorWithLabels {
    type Target = Cursor;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

impl std::ops::DerefMut for CursorWithLabels {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cursor
    }
}

impl Cursor {
    /// Returns a [`CursorWithLabels`] that wraps this cursor.
    pub fn with_labels(self) -> CursorWithLabels {
        CursorWithLabels::from(self)
    }
}

impl From<Cursor> for CursorWithLabels {
    fn from(cursor: Cursor) -> Self {
        CursorWithLabels { cursor }
    }
}
