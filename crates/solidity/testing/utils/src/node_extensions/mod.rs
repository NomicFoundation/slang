#[cfg(test)]
mod tests;

use slang_solidity::syntax::nodes::{Node, RuleKind};

pub trait NodeExtensions {
    fn is_trivia(&self) -> bool;
    fn extract_non_trivia(&self) -> String;
}

impl NodeExtensions for Node {
    fn is_trivia(&self) -> bool {
        match self {
            Node::Token(_) => false,
            Node::Rule(rule_node) => {
                rule_node.kind == RuleKind::LeadingTrivia
                    || rule_node.kind == RuleKind::TrailingTrivia
            }
        }
    }

    fn extract_non_trivia(&self) -> String {
        match self {
            Node::Token(token_node) => token_node.text.clone(),
            Node::Rule(rule_node) => rule_node
                .children
                .iter()
                .filter(|child| !child.is_trivia())
                .map(|child| child.extract_non_trivia())
                .collect(),
        }
    }
}
