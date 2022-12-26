use std::{ops::Range, rc::Rc};

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
                let children = children.iter().map(|child| Self::from_cst(child)).collect();
                let range = Self::calculate_range(&children);

                Self {
                    kind: TestNodeKind::Rule(*kind),
                    range,
                    children,
                }
            }
            cst::Node::Group { children } => {
                let children = children.iter().map(|child| Self::from_cst(child)).collect();
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
                        let test_trivium_child = match Self::from_trivia(trivium_child) {
                            Some(value) => value,
                            None => continue,
                        };

                        match trivium_kind {
                            RuleKind::LeadingTrivia => leading.push(test_trivium_child),
                            RuleKind::TrailingTrivia => trailing.push(test_trivium_child),
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

    fn from_trivia(trivia: &Rc<cst::Node>) -> Option<Self> {
        match trivia.as_ref() {
            cst::Node::Rule { .. } | cst::Node::Group { .. } => {
                unreachable!("Unexpected trivia: {trivia:?}")
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
                    TokenKind::Whitespace | TokenKind::EndOfLine => {
                        // hide this from snapshots
                        return None;
                    }
                    TokenKind::SingleLineComment | TokenKind::MultilineComment => {
                        return Some(Self {
                            kind: TestNodeKind::Trivia(*kind),
                            range: Some(lex_node.range()),
                            children: vec![],
                        });
                    }
                    other => {
                        unreachable!("Unexpected trivia token kind: {other:?}")
                    }
                }
            }
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
}
