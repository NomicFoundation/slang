// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use crate::cst;
use crate::kinds::TokenKind;
use crate::parse_error::ParseError;
use crate::parse_output::ParseOutput;
use crate::support::context::ParserContext;
use crate::support::parser_result::{IncompleteMatch, Match, ParserResult, SkippedUntil};
use crate::text_index::TextIndex;

pub trait ParserFunction<L>
where
    Self: Fn(&L, &mut ParserContext<'_>) -> ParserResult,
{
    fn parse(&self, language: &L, input: &str) -> ParseOutput;
}

impl<L, F> ParserFunction<L> for F
where
    F: Fn(&L, &mut ParserContext<'_>) -> ParserResult,
{
    fn parse(&self, language: &L, input: &str) -> ParseOutput {
        let mut stream = ParserContext::new(input);
        let result = self(language, &mut stream);

        let is_incomplete = matches!(result, ParserResult::IncompleteMatch(_));
        let is_recovering = matches!(result, ParserResult::SkippedUntil(_));

        match result {
            ParserResult::PrattOperatorMatch(..) => unreachable!("PrattOperatorMatch is internal"),

            ParserResult::NoMatch(no_match) => ParseOutput {
                parse_tree: cst::Node::token(TokenKind::SKIPPED, input.to_string()),
                errors: vec![ParseError::new_covering_range(
                    TextIndex::ZERO..input.into(),
                    no_match.expected_tokens,
                )],
            },
            some_match => {
                let (nodes, expected_tokens) = match some_match {
                    ParserResult::PrattOperatorMatch(..) | ParserResult::NoMatch(..) => {
                        unreachable!("Handled above")
                    }
                    ParserResult::Match(Match {
                        nodes,
                        expected_tokens,
                    })
                    | ParserResult::IncompleteMatch(IncompleteMatch {
                        nodes,
                        expected_tokens,
                    }) => (nodes, expected_tokens),

                    ParserResult::SkippedUntil(SkippedUntil {
                        nodes, expected, ..
                    }) => (nodes, vec![expected]),
                };

                let topmost_rule = match &nodes[..] {
                    [(_name, cst::Node::Rule(rule))] => Rc::clone(rule),
                    [_] => unreachable!(
                        "(Incomplete)Match at the top level of a parser is not a Rule node"
                    ),
                    _ => unreachable!(
                        "(Incomplete)Match at the top level of a parser does not have exactly one node"
                    ),
                };

                let start = stream.position();

                let errors = stream.into_errors();
                // Mark the rest of the unconsumed stream as skipped and report an error
                // NOTE: IncompleteMatch internally consumes the stream when picked via choice,
                // so needs a separate check here.
                if start.utf8 < input.len() || is_incomplete || is_recovering {
                    let start = if is_recovering {
                        topmost_rule.text_len
                    } else {
                        start
                    };
                    let skipped_node =
                        cst::Node::token(TokenKind::SKIPPED, input[start.utf8..].to_string());
                    let mut new_children = topmost_rule.children.clone();
                    new_children.push((String::new(), skipped_node));
                    let mut errors = errors;
                    errors.push(ParseError::new_covering_range(
                        start..input.into(),
                        expected_tokens,
                    ));

                    ParseOutput {
                        parse_tree: cst::Node::rule(topmost_rule.kind, new_children),
                        errors,
                    }
                } else {
                    let parse_tree = cst::Node::Rule(topmost_rule);
                    // Sanity check: Make sure that succesful parse is equivalent to not having any SKIPPED nodes
                    debug_assert_eq!(
                        errors.is_empty(),
                        parse_tree
                            .cursor_with_offset(TextIndex::ZERO)
                            .all(|node| node.as_token_with_kind(&[TokenKind::SKIPPED]).is_none())
                    );

                    ParseOutput { parse_tree, errors }
                }
            }
        }
    }
}
