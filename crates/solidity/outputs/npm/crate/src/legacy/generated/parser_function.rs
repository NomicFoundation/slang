// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{
    cst,
    parse_output::{ParseError, ParseOutput},
    parser_result::*,
    stream::Stream,
};

use crate::syntax::nodes::TokenKind;

// Return type of the function has to be a type parameter of the trait
pub trait ParserFunction<L, R>
where
    Self: Fn(&L, &mut Stream) -> R,
{
    fn parse(&self, language: &L, input: &str) -> Option<ParseOutput>;
}

impl<L, F> ParserFunction<L, ParserResult> for F
where
    F: Fn(&L, &mut Stream) -> ParserResult,
{
    fn parse(&self, language: &L, input: &str) -> Option<ParseOutput> {
        let mut stream = Stream::new(input);
        Some(match self(language, &mut stream) {
            ParserResult::NoMatch(no_match) => ParseOutput {
                parse_tree: cst::Node::token(TokenKind::SKIPPED, input.to_string()),
                errors: vec![ParseError::new_covering_range(
                    Default::default()..input.into(),
                    no_match.tokens_that_would_have_allowed_more_progress,
                )],
            },
            ParserResult::IncompleteMatch(IncompleteMatch {
                mut nodes,
                tokens_that_would_have_allowed_more_progress,
            }) => {
                if nodes.len() != 1 {
                    unreachable!(
                        "IncompleteMatch at the top level of a parser has more than one node"
                    )
                }
                if let cst::Node::Rule(rule_node) = nodes.remove(0) {
                    let start = stream.position();
                    let skipped_node =
                        cst::Node::token(TokenKind::SKIPPED, input[start.utf8..].to_string());
                    let mut new_children = rule_node.children.clone();
                    new_children.push(skipped_node);
                    ParseOutput {
                        parse_tree: cst::Node::rule(rule_node.kind, new_children),
                        errors: vec![ParseError::new_covering_range(
                            start..input.into(),
                            tokens_that_would_have_allowed_more_progress,
                        )],
                    }
                } else {
                    unreachable!("IncompleteMatch at the top level of a parser is not a Rule node")
                }
            }
            ParserResult::Match(mut r#match) => {
                if r#match.nodes.len() != 1 {
                    unreachable!("Match at the top level of a parser has more than one node")
                }
                if let cst::Node::Rule(rule_node) = r#match.nodes.remove(0) {
                    let start = stream.position();
                    if start.utf8 < input.len() {
                        let skipped_node =
                            cst::Node::token(TokenKind::SKIPPED, input[start.utf8..].to_string());
                        let mut new_children = rule_node.children.clone();
                        new_children.push(skipped_node);
                        ParseOutput {
                            parse_tree: cst::Node::rule(rule_node.kind, new_children),
                            errors: vec![ParseError::new_covering_range(
                                start..input.into(),
                                r#match.tokens_that_would_have_allowed_more_progress,
                            )],
                        }
                    } else {
                        ParseOutput {
                            parse_tree: cst::Node::Rule(rule_node),
                            errors: vec![],
                        }
                    }
                } else {
                    unreachable!("Match at the top level of a parser is not a Rule node")
                }
            }
            ParserResult::PrattOperatorMatch(..) => unreachable!("PrattOperatorMatch is internal"),
        })
    }
}

impl<L, F> ParserFunction<L, Option<ParserResult>> for F
where
    F: Fn(&L, &mut Stream) -> Option<ParserResult>,
{
    fn parse(&self, language: &L, input: &str) -> Option<ParseOutput> {
        let mut stream = Stream::new(input);
        self(language, &mut stream).map(|result| match result {
            ParserResult::NoMatch(no_match) => ParseOutput {
                parse_tree: cst::Node::token(TokenKind::SKIPPED, input.to_string()),
                errors: vec![ParseError::new_covering_range(
                    Default::default()..input.into(),
                    no_match.tokens_that_would_have_allowed_more_progress,
                )],
            },
            ParserResult::IncompleteMatch(IncompleteMatch {
                mut nodes,
                tokens_that_would_have_allowed_more_progress,
            }) => {
                if nodes.len() != 1 {
                    unreachable!(
                        "IncompleteMatch at the top level of a parser has more than one node"
                    )
                }
                if let cst::Node::Rule(rule_node) = nodes.remove(0) {
                    let start = stream.position();
                    let skipped_node =
                        cst::Node::token(TokenKind::SKIPPED, input[start.utf8..].to_string());
                    let mut new_children = rule_node.children.clone();
                    new_children.push(skipped_node);
                    ParseOutput {
                        parse_tree: cst::Node::rule(rule_node.kind, new_children),
                        errors: vec![ParseError::new_covering_range(
                            start..input.into(),
                            tokens_that_would_have_allowed_more_progress,
                        )],
                    }
                } else {
                    unreachable!("IncompleteMatch at the top level of a parser is not a Rule node")
                }
            }
            ParserResult::Match(mut r#match) => {
                if r#match.nodes.len() != 1 {
                    unreachable!("Match at the top level of a parser has more than one node")
                }
                if let cst::Node::Rule(rule_node) = r#match.nodes.remove(0) {
                    let start = stream.position();
                    if start.utf8 < input.len() {
                        let skipped_node =
                            cst::Node::token(TokenKind::SKIPPED, input[start.utf8..].to_string());
                        let mut new_children = rule_node.children.clone();
                        new_children.push(skipped_node);
                        ParseOutput {
                            parse_tree: cst::Node::rule(rule_node.kind, new_children),
                            errors: vec![ParseError::new_covering_range(
                                start..input.into(),
                                r#match.tokens_that_would_have_allowed_more_progress,
                            )],
                        }
                    } else {
                        ParseOutput {
                            parse_tree: cst::Node::Rule(rule_node),
                            errors: vec![],
                        }
                    }
                } else {
                    unreachable!("Match at the top level of a parser is not a Rule node")
                }
            }
            ParserResult::PrattOperatorMatch(..) => unreachable!("PrattOperatorMatch is internal"),
        })
    }
}
