use std::rc::Rc;

use anyhow::Result;
use slang_solidity::syntax::nodes::{Node, RuleKind, TextRange, TokenKind};

#[derive(Debug)]
pub enum TestNodeKind {
    Rule(RuleKind),
    Token(TokenKind),
    Trivia(TokenKind),
    Contents,
}

pub struct TestNode {
    pub kind: TestNodeKind,
    pub range: TextRange,
    pub children: Vec<TestNode>,
}

impl TestNode {
    pub fn from_cst(node: &Rc<Node>) -> Self {
        return match node.as_ref() {
            Node::Rule {
                kind,
                range,
                children,
            } => {
                let children = children
                    .iter()
                    .filter(|child| !Self::is_whitespace(child))
                    .map(|child| Self::from_cst(child))
                    .collect();

                Self {
                    kind: TestNodeKind::Rule(*kind),
                    range: range.clone(),
                    children,
                }
            }
            Node::Token {
                kind,
                range,
                trivia,
            } => Self::from_token(kind, range, node.range_including_trivia(), trivia),
        };
    }

    fn from_token(
        token_kind: &TokenKind,
        token_range: &TextRange,
        node_range: TextRange,
        token_trivia: &Vec<Rc<Node>>,
    ) -> Self {
        let mut leading = vec![];
        let mut trailing = vec![];

        for trivium in token_trivia {
            match trivium.as_ref() {
                Node::Token { .. } => {
                    unreachable!("Trivium should always be a Rule: {trivium:?}")
                }
                Node::Rule {
                    kind: trivium_kind,
                    children: trivium_children,
                    ..
                } => {
                    for trivium_child in trivium_children {
                        match trivium_kind {
                            RuleKind::LeadingTrivia => {
                                Self::collect_trivia(trivium_child, &mut leading);
                            }
                            RuleKind::TrailingTrivia => {
                                Self::collect_trivia(trivium_child, &mut trailing);
                            }
                            _ => unreachable!("Unexpected trivium kind: {trivium_kind:?}"),
                        }
                    }
                }
            }
        }

        // If no trivia, or they were all skipped (whitespace) just return the inner lex_node:
        if leading.is_empty() && trailing.is_empty() {
            return Self {
                kind: TestNodeKind::Token(*token_kind),
                range: token_range.clone(),
                children: vec![],
            };
        }

        let contents_node = Self {
            kind: TestNodeKind::Contents,
            range: token_range.clone(),
            children: vec![],
        };

        let mut children = vec![];
        children.extend(leading);
        children.push(contents_node);
        children.extend(trailing);

        return Self {
            kind: TestNodeKind::Token(*token_kind),
            range: node_range,
            children,
        };
    }

    fn collect_trivia(node: &Rc<Node>, collection: &mut Vec<Self>) {
        if Self::is_whitespace(node) {
            return;
        }

        match node.as_ref() {
            Node::Rule { children, .. } => {
                for child in children {
                    Self::collect_trivia(child, collection);
                }
            }
            Node::Token {
                kind,
                trivia,
                range,
            } => {
                assert!(
                    trivia.is_empty(),
                    "Trivia should not contain sub-trivia: {trivia:?}"
                );

                match kind {
                    TokenKind::SingleLineComment | TokenKind::MultilineComment => {
                        collection.push(Self {
                            kind: TestNodeKind::Trivia(*kind),
                            range: range.clone(),
                            children: vec![],
                        });
                    }
                    other => {
                        unreachable!("Unexpected trivia token kind: {other:?}")
                    }
                };
            }
        };
    }

    fn is_whitespace(token: &Rc<Node>) -> bool {
        return match token.as_ref() {
            Node::Token { kind, .. } => match kind {
                TokenKind::Whitespace | TokenKind::EndOfLine => true,
                _ => false,
            },
            _ => false,
        };
    }

    pub fn render_preview(&self, source: &str, range: &TextRange) -> Result<String> {
        let max_length = 50;
        let length = range.end.byte - range.start.byte;

        // Trim long values:
        let contents = source
            .bytes()
            .skip(range.start.byte)
            .take(length.clamp(0, max_length))
            .collect();

        // Add terminator if trimmed:
        let mut contents = String::from_utf8(contents)?;
        if length > max_length {
            contents.push_str("...");
        }

        // Escape line breaks:
        let contents = contents
            .replace("\t", "\\t")
            .replace("\r", "\\r")
            .replace("\n", "\\n");

        // Surround by quotes:
        let contents = {
            let delimiter = if contents.contains("\"") && !contents.contains("'") {
                "'"
            } else {
                "\""
            };
            let contents = contents.replace(delimiter, &format!("\\{delimiter}"));
            format!("{delimiter}{contents}{delimiter}")
        };

        return Ok(contents);
    }
}

impl std::fmt::Display for TestNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match self {
            TestNodeKind::Rule(kind) => write!(f, "{kind:?} (Rule)"),
            TestNodeKind::Token(kind) => write!(f, "{kind:?} (Token)"),
            TestNodeKind::Trivia(kind) => write!(f, "{kind:?} (Trivia)"),
            TestNodeKind::Contents => write!(f, "Contents"),
        };
    }
}
