#[cfg(test)]
mod tests;

use slang_solidity::cst::{Node, RuleNode, TokenNode};

pub trait NodeExtensions {
    fn extract_non_trivia(&self) -> String;
}

impl NodeExtensions for Node {
    fn extract_non_trivia(&self) -> String {
        match self {
            Node::Token(token) => token.extract_non_trivia(),
            Node::Rule(rule) => rule.extract_non_trivia(),
        }
    }
}

impl NodeExtensions for RuleNode {
    fn extract_non_trivia(&self) -> String {
        self.children
            .iter()
            .filter(|child| !child.is_trivia())
            .map(|child| child.extract_non_trivia())
            .collect()
    }
}

impl NodeExtensions for TokenNode {
    fn extract_non_trivia(&self) -> String {
        self.text.clone()
    }
}
