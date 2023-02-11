use std::{ops::Range, rc::Rc};

use anyhow::Result;
use solidity_rust_lib::generated::{
    cst,
    kinds::{RuleKind, TokenKind},
    lex,
};

#[derive(Debug)]
pub enum TestNodeKind {
    Rule(RuleKind),
    Group,
    Token(TokenKind),
    Trivia(TokenKind),
    Contents,
}

pub struct TestNode {
    pub kind: TestNodeKind,
    pub range: Option<Range<usize>>,
    pub children: Vec<TestNode>,
}

impl TestNode {
    pub fn from_cst(node: &Rc<cst::Node>) -> Self {
        return match node.as_ref() {
            cst::Node::Rule { kind, children } => {
                let children = children
                    .iter()
                    .filter(|child| !Self::is_whitespace(child))
                    .map(|child| Self::from_cst(child))
                    .collect();

                let range = Self::calculate_range(&children);

                Self {
                    kind: TestNodeKind::Rule(*kind),
                    range,
                    children,
                }
            }
            cst::Node::Group { children } => {
                let children = children
                    .iter()
                    .filter(|child| !Self::is_whitespace(child))
                    .map(|child| Self::from_cst(child))
                    .collect();

                let range = Self::calculate_range(&children);

                Self {
                    kind: TestNodeKind::Group,
                    range,
                    children,
                }
            }
            cst::Node::Token {
                kind,
                lex_node,
                trivia,
            } => Self::from_token(kind, lex_node, trivia),
        };
    }

    fn from_token(
        token_kind: &TokenKind,
        token_lex_node: &Rc<lex::Node>,
        token_trivia: &Vec<Rc<cst::Node>>,
    ) -> Self {
        let mut leading = vec![];
        let mut trailing = vec![];

        for trivium in token_trivia {
            match trivium.as_ref() {
                cst::Node::Group { .. } | cst::Node::Token { .. } => {
                    unreachable!("Trivium should always be a Rule: {trivium:?}")
                }
                cst::Node::Rule {
                    kind: trivium_kind,
                    children: trivium_children,
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
                range: Some(token_lex_node.range()),
                children: vec![],
            };
        }

        let contents_node = Self {
            kind: TestNodeKind::Contents,
            range: Some(token_lex_node.range()),
            children: vec![],
        };

        let mut children = vec![];
        children.extend(leading);
        children.push(contents_node);
        children.extend(trailing);

        return Self {
            kind: TestNodeKind::Token(*token_kind),
            range: Self::calculate_range(&children),
            children,
        };
    }

    fn collect_trivia(node: &Rc<cst::Node>, collection: &mut Vec<Self>) {
        if Self::is_whitespace(node) {
            return;
        }

        match node.as_ref() {
            cst::Node::Rule { children, .. } | cst::Node::Group { children, .. } => {
                for child in children {
                    Self::collect_trivia(child, collection);
                }
            }
            cst::Node::Token {
                kind,
                trivia,
                lex_node,
            } => {
                assert!(
                    trivia.is_empty(),
                    "Trivia should not contain sub-trivia: {trivia:?}"
                );

                match kind {
                    TokenKind::SingleLineComment | TokenKind::MultilineComment => {
                        collection.push(Self {
                            kind: TestNodeKind::Trivia(*kind),
                            range: Some(lex_node.range()),
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

    fn is_whitespace(token: &Rc<cst::Node>) -> bool {
        return match token.as_ref() {
            cst::Node::Token { kind, .. } => match kind {
                TokenKind::Whitespace | TokenKind::EndOfLine => true,
                _ => false,
            },
            _ => false,
        };
    }

    fn calculate_range(children: &Vec<Self>) -> Option<Range<usize>> {
        let ranges: Vec<&Range<usize>> = children
            .iter()
            .filter_map(|child| child.range.as_ref())
            .collect();

        // Will return `None` if no ranges are found:
        return Some(ranges.first()?.start..ranges.last()?.end);
    }

    pub fn render_preview(&self, source: &str, range: &Range<usize>) -> Result<String> {
        let mut contents = source
            .chars()
            .skip(range.start)
            .take(range.end - range.start)
            .collect::<String>();

        // Trim long values:
        let max_length = 50;
        if contents.len() > max_length {
            let separator = "...";
            contents = contents
                .chars()
                .take(max_length)
                .chain(separator.chars())
                .collect();
        }

        // Double quote, and escape line breaks:
        contents = serde_json::to_string(&contents)?;

        return Ok(contents);
    }
}

impl std::fmt::Display for TestNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match self {
            TestNodeKind::Rule(kind) => write!(f, "{kind:?} (Rule)"),
            TestNodeKind::Group => write!(f, "Group"),
            TestNodeKind::Token(kind) => write!(f, "{kind:?} (Token)"),
            TestNodeKind::Trivia(kind) => write!(f, "{kind:?} (Trivia)"),
            TestNodeKind::Contents => write!(f, "Contents"),
        };
    }
}
