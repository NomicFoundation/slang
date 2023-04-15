#[cfg(test)]
mod tests;

use slang_solidity::syntax::nodes::Node;

pub trait NodeExtensions {
    fn extract_non_trivia(&self, source: &str) -> String;
}

impl NodeExtensions for Node {
    fn extract_non_trivia(&self, source: &str) -> String {
        match self {
            Node::Token { range, .. } => {
                let range = range;
                let result = source
                    .bytes()
                    .skip(range.start)
                    .take(range.end - range.start)
                    .collect();

                return String::from_utf8(result).unwrap();
            }
            Node::Rule { children, .. } => {
                return children
                    .iter()
                    .map(|child| child.extract_non_trivia(source))
                    .collect();
            }
        };
    }
}
