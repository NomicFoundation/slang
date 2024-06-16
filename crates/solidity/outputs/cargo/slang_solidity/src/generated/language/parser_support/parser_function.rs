// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use crate::cst::{self, Edge};
use crate::kinds::TerminalKind;
use crate::language::lexer::Lexer;
use crate::language::parser_support::context::ParserContext;
use crate::language::parser_support::parser_result::{
    IncompleteMatch, Match, ParserResult, SkippedUntil,
};
use crate::parse_error::ParseError;
use crate::parse_output::ParseOutput;
use crate::text_index::TextIndex;

pub trait ParserFunction<L>
where
    Self: Fn(&L, &mut ParserContext<'_>) -> ParserResult,
{
    fn parse(&self, language: &L, input: &str) -> ParseOutput;
}

impl<L, F> ParserFunction<L> for F
where
    L: Lexer,
    F: Fn(&L, &mut ParserContext<'_>) -> ParserResult,
{
    fn parse(&self, language: &L, input: &str) -> ParseOutput {
        let mut stream = ParserContext::new(input);
        let mut result = self(language, &mut stream);

        // For a succesful/recovered parse, collect any remaining trivia as part of the parse result
        if let ParserResult::Match(r#match) = &mut result {
            let [topmost] = r#match.nodes.as_mut_slice() else {
                unreachable!(
                    "Match at the top level of a parse does not have exactly one Nonterminal node"
                )
            };

            let eof_trivia = match Lexer::leading_trivia(language, &mut stream) {
                ParserResult::Match(eof_trivia) if !eof_trivia.nodes.is_empty() => {
                    Some(eof_trivia.nodes)
                }
                _ => None,
            };

            if let (cst::Node::Nonterminal(nonterminal), Some(eof_trivia)) =
                (&mut topmost.node, eof_trivia)
            {
                let mut new_children = nonterminal.children.clone();
                new_children.extend(eof_trivia);

                topmost.node = cst::Node::nonterminal(nonterminal.kind, new_children);
            }
        }

        let is_incomplete = matches!(result, ParserResult::IncompleteMatch(_));
        let is_recovering = matches!(result, ParserResult::SkippedUntil(_));

        match result {
            ParserResult::PrattOperatorMatch(..) => unreachable!("PrattOperatorMatch is internal"),

            ParserResult::NoMatch(no_match) => ParseOutput {
                parse_tree: cst::Node::terminal(TerminalKind::SKIPPED, input.to_string()),
                errors: vec![ParseError::new(
                    TextIndex::ZERO..input.into(),
                    no_match.expected_terminals,
                )],
            },
            some_match => {
                let (nodes, expected_terminals) = match some_match {
                    ParserResult::PrattOperatorMatch(..) | ParserResult::NoMatch(..) => {
                        unreachable!("Handled above")
                    }
                    ParserResult::Match(Match {
                        nodes,
                        expected_terminals,
                    })
                    | ParserResult::IncompleteMatch(IncompleteMatch {
                        nodes,
                        expected_terminals,
                    }) => (nodes, expected_terminals),

                    ParserResult::SkippedUntil(SkippedUntil {
                        nodes, expected, ..
                    }) => (nodes, vec![expected]),
                };

                let topmost_node = match &nodes[..] {
                    [Edge { node: cst::Node::Nonterminal(nonterminal), ..} ] => Rc::clone(nonterminal),
                    [_] => unreachable!(
                        "(Incomplete)Match at the top level of a parser is not a Nonterminal node"
                    ),
                    _ => unreachable!(
                        "(Incomplete)Match at the top level of a parser does not have exactly one node"
                    ),
                };

                let start = stream.position();

                // Mark the rest of the unconsumed stream as skipped and report an error
                // NOTE: IncompleteMatch internally consumes the stream when picked via choice,
                // so needs a separate check here.
                if start.utf8 < input.len() || is_incomplete || is_recovering {
                    let start = if is_recovering {
                        topmost_node.text_len
                    } else {
                        start
                    };
                    let skipped_node =
                        cst::Node::terminal(TerminalKind::SKIPPED, input[start.utf8..].to_string());
                    let mut new_children = topmost_node.children.clone();
                    new_children.push(Edge::anonymous(skipped_node));

                    let mut errors = stream.into_errors();
                    errors.push(ParseError::new(start..input.into(), expected_terminals));

                    ParseOutput {
                        parse_tree: cst::Node::nonterminal(topmost_node.kind, new_children),
                        errors,
                    }
                } else {
                    let parse_tree = cst::Node::Nonterminal(topmost_node);
                    let errors = stream.into_errors();

                    // Sanity check: Make sure that succesful parse is equivalent to not having any SKIPPED nodes
                    debug_assert_eq!(
                        errors.is_empty(),
                        parse_tree
                            .cursor_with_offset(TextIndex::ZERO)
                            .all(|node| node
                                .as_terminal_with_kind(TerminalKind::SKIPPED)
                                .is_none())
                    );

                    ParseOutput { parse_tree, errors }
                }
            }
        }
    }
}
