#[cfg(test)]
mod tests;

use slang_solidity::{
    cst::{Node, RuleNode, TokenNode},
    kinds::RuleKind,
};

pub trait NodeExtensions {
    fn is_trivia(&self) -> bool;
    fn extract_non_trivia(&self) -> String;
}

impl NodeExtensions for Node {
    fn is_trivia(&self) -> bool {
        return match self {
            Node::Token(token) => token.is_trivia(),
            Node::Rule(rule) => rule.is_trivia(),
        };
    }

    fn extract_non_trivia(&self) -> String {
        return match self {
            Node::Token(token) => token.extract_non_trivia(),
            Node::Rule(rule) => rule.extract_non_trivia(),
        };
    }
}

impl NodeExtensions for RuleNode {
    fn is_trivia(&self) -> bool {
        matches!(
            self.kind,
            RuleKind::LeadingTrivia | RuleKind::TrailingTrivia | RuleKind::EndOfFileTrivia
        )
    }

    fn extract_non_trivia(&self) -> String {
        return self
            .children
            .iter()
            .filter(|child| !child.is_trivia())
            .map(|child| child.extract_non_trivia())
            .collect();
    }
}

impl NodeExtensions for TokenNode {
    fn is_trivia(&self) -> bool {
        return false;
    }

    fn extract_non_trivia(&self) -> String {
        return self.text.clone();
    }
}
