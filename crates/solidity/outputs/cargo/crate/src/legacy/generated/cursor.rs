// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use super::{
    cst::{Node, RuleNode, TokenNode},
    text_index::{TextIndex, TextRange},
};

use crate::syntax::nodes::{RuleKind, TokenKind};

#[derive(Clone, Debug, PartialEq, Eq)]
struct CursorPathElement {
    rule_node: Rc<RuleNode>,
    child_number: usize,
    text_offset: TextIndex,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CursorLeaf {
    node: Node,
    child_number: usize,
    text_offset: TextIndex,
}

impl CursorLeaf {
    pub fn text_range(&self) -> TextRange {
        let start = self.text_offset;
        let end = start + self.node.text_len();
        start..end
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cursor {
    path: Vec<CursorPathElement>,
    leaf: CursorLeaf,
    is_completed: bool,
}

#[allow(dead_code)]
impl Cursor {
    pub(crate) fn new(node: Node) -> Self {
        Self {
            path: vec![],
            leaf: CursorLeaf {
                node,
                child_number: 0,
                text_offset: Default::default(),
            },
            is_completed: false,
        }
    }

    pub fn reset(&mut self) {
        self.complete();
        self.is_completed = false;
    }

    pub fn complete(&mut self) {
        if let Some(path_element) = self.path.get(0) {
            self.leaf.text_offset = path_element.text_offset;
            self.leaf.child_number = path_element.child_number;
            self.leaf.node = Node::Rule(path_element.rule_node.clone());
            self.path.clear();
        }
        self.is_completed = true;
    }

    // Unlike clone, this re-roots at the current node.
    // It does preserve the correct text offset however,
    // even though the path is reset.
    pub fn spawn(&self) -> Self {
        Self {
            path: vec![],
            leaf: self.leaf.clone(),
            is_completed: false,
        }
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    pub fn node(&self) -> Node {
        self.leaf.node.clone()
    }

    pub fn text_offset(&self) -> TextIndex {
        self.leaf.text_offset
    }

    pub fn text_range(&self) -> TextRange {
        self.leaf.text_range()
    }

    pub fn path_rule_nodes(&self) -> Vec<Rc<RuleNode>> {
        self.path
            .iter()
            .map(|path_element| path_element.rule_node.clone())
            .collect()
    }

    pub fn go_to_next(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        if !self.go_to_first_child() {
            return self.go_to_next_non_descendent();
        }
        return true;
    }

    pub fn go_to_next_non_descendent(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        while !self.go_to_next_sibling() {
            if !self.go_to_parent() {
                return false;
            }
        }
        return true;
    }

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
        return true;
    }

    pub fn go_to_parent(&mut self) -> bool {
        if self.path.is_empty() {
            self.is_completed = true;
            return false;
        }

        let path_element = self.path.pop().unwrap();
        self.leaf.text_offset = path_element.text_offset;
        self.leaf.child_number = path_element.child_number;
        self.leaf.node = Node::Rule(path_element.rule_node);
        return true;
    }

    pub fn go_to_first_child(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        // Check that the leaf is a rule node, and destructure if so
        if let CursorLeaf {
            node: Node::Rule(parent_rule_node),
            text_offset: parent_text_offset,
            child_number: parent_child_number,
        } = &self.leaf
        {
            let child_number = 0;
            if let Some(child_node) = parent_rule_node.children.get(child_number).cloned() {
                self.path.push(CursorPathElement {
                    rule_node: parent_rule_node.clone(),
                    child_number: *parent_child_number,
                    text_offset: *parent_text_offset,
                });
                self.leaf.text_offset = *parent_text_offset;
                self.leaf.child_number = child_number;
                self.leaf.node = child_node;
                return true;
            }
        }

        return false;
    }

    pub fn go_to_last_child(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        // Check that the leaf is a rule node, and destructure if so
        if let CursorLeaf {
            node: Node::Rule(parent_rule_node),
            text_offset: parent_text_offset,
            child_number: parent_child_number,
        } = &self.leaf
        {
            let child_number = parent_rule_node.children.len() - 1;
            if let Some(child_node) = parent_rule_node.children.get(child_number).cloned() {
                // This is cheaper than summing up the length of the children
                let text_offset =
                    *parent_text_offset + parent_rule_node.text_len - child_node.text_len();
                self.path.push(CursorPathElement {
                    rule_node: parent_rule_node.clone(),
                    child_number: *parent_child_number,
                    text_offset: *parent_text_offset,
                });
                self.leaf.text_offset = text_offset;
                self.leaf.child_number = child_number;
                self.leaf.node = child_node;
                return true;
            }
        }

        return false;
    }

    pub fn go_to_nth_child(&mut self, child_number: usize) -> bool {
        if self.is_completed {
            return false;
        }

        // Check that the leaf is a rule node, and destructure if so
        if let CursorLeaf {
            node: Node::Rule(parent_rule_node),
            text_offset: parent_text_offset,
            child_number: parent_child_number,
        } = &self.leaf
        {
            if let Some(child_node) = parent_rule_node.children.get(child_number).cloned() {
                // Sum up the length of the children before this child
                // TODO: it might sometimes be quicker to start from the end (like `go_to_last_child`)
                let mut text_offset = *parent_text_offset;
                for child in &parent_rule_node.children[..child_number] {
                    text_offset += child.text_len();
                }
                self.path.push(CursorPathElement {
                    rule_node: parent_rule_node.clone(),
                    child_number: *parent_child_number,
                    text_offset: *parent_text_offset,
                });
                self.leaf.text_offset = text_offset;
                self.leaf.child_number = child_number;
                self.leaf.node = child_node;
                return true;
            }
        }

        return false;
    }

    pub fn go_to_next_sibling(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        if let Some(parent_path_element) = self.path.last() {
            let new_child_number = self.leaf.child_number + 1;
            if let Some(new_child) = parent_path_element.rule_node.children.get(new_child_number) {
                self.leaf.text_offset += self.leaf.node.text_len();
                self.leaf.child_number = new_child_number;
                self.leaf.node = new_child.clone();
                return true;
            }
        }

        return false;
    }

    pub fn go_to_previous_sibling(&mut self) -> bool {
        if self.is_completed {
            return false;
        }

        if self.leaf.child_number > 0 {
            if let Some(parent_path_element) = self.path.last() {
                let new_child_number = self.leaf.child_number + 1;
                let new_child = parent_path_element.rule_node.children[new_child_number].clone();
                self.leaf.text_offset -= self.leaf.node.text_len();
                self.leaf.child_number = new_child_number;
                self.leaf.node = new_child;
                return true;
            }
        }

        return false;
    }

    pub fn find_matching<R, F: Fn(&Cursor) -> Option<R>>(&mut self, filter_map: F) -> Option<R> {
        while !self.is_completed {
            if let Some(result) = filter_map(self) {
                return Some(result);
            }
            self.go_to_next();
        }
        return None;
    }

    pub fn find_token_with_kind(&mut self, kinds: &[TokenKind]) -> Option<Rc<TokenNode>> {
        while !self.is_completed {
            if let Some(token_node) = self.leaf.node.as_token_with_kind(kinds).cloned() {
                return Some(token_node);
            }
            self.go_to_next();
        }
        return None;
    }

    pub fn find_token_matching<F: Fn(&Rc<TokenNode>) -> bool>(
        &mut self,
        predicate: F,
    ) -> Option<Rc<TokenNode>> {
        while !self.is_completed {
            if let Some(token_node) = self.leaf.node.as_token_matching(&predicate) {
                return Some(token_node.clone());
            }
            self.go_to_next();
        }
        return None;
    }

    pub fn find_rule_with_kind(&mut self, kinds: &[RuleKind]) -> Option<Rc<RuleNode>> {
        while !self.is_completed {
            if let Some(rule_node) = self.leaf.node.as_rule_with_kind(kinds) {
                return Some(rule_node.clone());
            }
            self.go_to_next();
        }
        return None;
    }

    pub fn find_rule_matching<F: Fn(&Rc<RuleNode>) -> bool>(
        &mut self,
        predicate: F,
    ) -> Option<Rc<RuleNode>> {
        while !self.is_completed {
            if let Some(rule_node) = self.leaf.node.as_rule_matching(&predicate) {
                return Some(rule_node.clone());
            }
            self.go_to_next();
        }
        return None;
    }
}
