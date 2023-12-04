//! A cursor that can traverse a CST in a DFS pre-order fashion.

use std::rc::Rc;

use super::{
    cst::{Node, RuleNode},
    kinds::{RuleKind, TokenKind},
    text_index::{TextIndex, TextRange},
};

/// A [`PathNode`] that points to a [`RuleNode`].
#[derive(Clone, Debug, PartialEq, Eq)]
struct PathRuleNode {
    rule_node: Rc<RuleNode>,
    child_number: usize,
    text_offset: TextIndex,
}

impl PathRuleNode {
    fn into_path_node(self) -> PathNode {
        PathNode {
            node: Node::Rule(self.rule_node),
            child_number: self.child_number,
            text_offset: self.text_offset,
        }
    }
}

/// A pointer to a [`Node`] in a CST, used by the [`Cursor`] to implement the traversal.
#[derive(Clone, Debug, PartialEq, Eq)]
struct PathNode {
    /// The node the cursor is currently pointing to.
    node: Node,
    /// The index of the current child node in the parent's children.
    // Required to go to the next/previous sibling.
    child_number: usize,
    /// Text offset that corresponds to the beginning of the currently pointed to node.
    text_offset: TextIndex,
}

impl PathNode {
    fn text_range(&self) -> TextRange {
        let start = self.text_offset;
        let end = start + self.node.text_len();
        start..end
    }

    fn to_path_rule_node(&self) -> Option<PathRuleNode> {
        if let Node::Rule(rule_node) = &self.node {
            Some(PathRuleNode {
                rule_node: rule_node.clone(),
                child_number: self.child_number,
                text_offset: self.text_offset,
            })
        } else {
            None
        }
    }
}

/// A cursor that can traverse a CST.
///
/// Nodes are visited in a DFS pre-order traversal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cursor {
    /// The list of ancestor rule nodes that the `current` node is a part of.
    path: Vec<PathRuleNode>,
    /// The node the cursor is currently pointing to.
    current: PathNode,
    /// Whether the cursor is completed, i.e. at the root node as a result of traversal (or when `complete`d).
    /// If `true`, the cursor cannot be moved.
    is_completed: bool,
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
            path: vec![],
            current: PathNode {
                node,
                child_number: 0,
                text_offset,
            },
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
        if let Some(root) = self.path.drain(..).next() {
            self.current = root.into_path_node();
        }
        self.is_completed = true;
    }

    /// Unlike `clone`, this re-roots at the current node.
    /// It does preserve the correct text offset however,
    /// even though the path is reset.
    pub fn spawn(&self) -> Self {
        Self {
            path: vec![],
            current: self.current.clone(),
            is_completed: false,
        }
    }

    /// Whether the cursor can be moved.
    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    /// Returns the currently pointed to [`Node`].
    pub fn node(&self) -> Node {
        self.current.node.clone()
    }

    /// Returns the text offset that corresponds to the beginning of the currently pointed to node.
    pub fn text_offset(&self) -> TextIndex {
        self.current.text_offset
    }

    /// Returns the text range that corresponds to the currently pointed to node.
    pub fn text_range(&self) -> TextRange {
        self.current.text_range()
    }

    /// Returns the depth of the current node in the CST, i.e. the number of ancestors.
    pub fn depth(&self) -> usize {
        self.path.len()
    }

    /// Returns an iterator over the current node's ancestors, starting from the cursor root node.
    pub fn ancestors(&self) -> impl Iterator<Item = &Rc<RuleNode>> {
        self.path.iter().map(|elem| &elem.rule_node)
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
        match self.path.pop() {
            Some(parent) => {
                self.current = parent.into_path_node();

                true
            }
            None => {
                self.is_completed = true;

                false
            }
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
        if let Some(parent) = self.current.to_path_rule_node() {
            if let Some(child_node) = parent.rule_node.children.first().cloned() {
                self.current = PathNode {
                    node: child_node,
                    text_offset: parent.text_offset,
                    child_number: 0,
                };

                self.path.push(parent);

                return true;
            }
        }

        false
    }

    /// Attempts to go to current node's last child.

    /// Returns `false` if the cursor is finished or there's no child to go to.
    pub fn go_to_last_child(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        if let Some(parent) = self.current.to_path_rule_node() {
            let child_number = parent.rule_node.children.len() - 1;
            if let Some(child_node) = parent.rule_node.children.get(child_number).cloned() {
                // This is cheaper than summing up the length of the children
                let text_offset =
                    parent.text_offset + parent.rule_node.text_len - child_node.text_len();

                self.path.push(parent);

                self.current = PathNode {
                    node: child_node,
                    text_offset,
                    child_number,
                };

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

        if let Some(parent) = self.current.to_path_rule_node() {
            if let Some(child_node) = parent.rule_node.children.get(child_number).cloned() {
                // Sum up the length of the children before this child
                // TODO: it might sometimes be quicker to start from the end (like `go_to_last_child`)
                let text_offset = parent.text_offset
                    + parent.rule_node.children[..child_number]
                        .iter()
                        .map(Node::text_len)
                        .sum();

                self.path.push(parent);
                self.current = PathNode {
                    node: child_node,
                    text_offset,
                    child_number,
                };

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

        if let Some(parent_path_element) = self.path.last() {
            let new_child_number = self.current.child_number + 1;
            if let Some(new_child) = parent_path_element.rule_node.children.get(new_child_number) {
                self.current = PathNode {
                    node: new_child.clone(),
                    text_offset: self.current.text_offset + self.current.node.text_len(),
                    child_number: new_child_number,
                };

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

        if self.current.child_number > 0 {
            if let Some(parent_path_element) = self.path.last() {
                let new_child_number = self.current.child_number - 1;
                let new_child = parent_path_element.rule_node.children[new_child_number].clone();

                self.current = PathNode {
                    node: new_child,
                    text_offset: self.current.text_offset - self.current.node.text_len(),
                    child_number: new_child_number,
                };
                return true;
            }
        }

        false
    }

    /// Attempts to go to the next token, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_token(&mut self) -> bool {
        self.go_to_next_matching(|node| matches!(node, Node::Token(_)))
    }

    /// Attempts to go to the next token with any of the given kinds, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_token_with_kinds(&mut self, kinds: &[TokenKind]) -> bool {
        self.go_to_next_matching(
            |node| matches!(node, Node::Token(token) if kinds.contains(&token.kind)),
        )
    }

    /// Attempts to go to the next rule, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_rule(&mut self) -> bool {
        self.go_to_next_matching(|node| matches!(node, Node::Rule(_)))
    }

    /// Attempts to go to the next rule with any of the given kinds, according to the DFS pre-order traversal.
    ///
    /// Returns `false` if the cursor is finished and at the root.
    pub fn go_to_next_rule_with_kinds(&mut self, kinds: &[RuleKind]) -> bool {
        self.go_to_next_matching(
            |node| matches!(node, Node::Rule(rule) if kinds.contains(&rule.kind)),
        )
    }

    fn go_to_next_matching(&mut self, pred: impl Fn(&Node) -> bool) -> bool {
        while self.go_to_next() {
            if pred(&self.current.node) {
                return true;
            }
        }

        false
    }
}
