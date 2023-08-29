use std::rc::Rc;

use super::{
    super::{cst, kinds::TokenKind, parse_error::ParseError, parse_output::ParseOutput},
    context::ParserContext,
    parser_result::*,
};

pub trait ParserFunction<L>
where
    Self: Fn(&L, &mut ParserContext) -> ParserResult,
{
    fn parse(&self, language: &L, input: &str) -> ParseOutput;
}

impl<L, F> ParserFunction<L> for F
where
    F: Fn(&L, &mut ParserContext) -> ParserResult,
{
    fn parse(&self, language: &L, input: &str) -> ParseOutput {
        let mut stream = ParserContext::new(input);
        let result = self(language, &mut stream);

        let is_incomplete = matches!(result, ParserResult::IncompleteMatch(_));

        match result {
            ParserResult::NoMatch(no_match) => ParseOutput {
                parse_tree: cst::Node::token(TokenKind::SKIPPED, input.to_string()),
                errors: vec![ParseError::new_covering_range(
                    Default::default()..input.into(),
                    no_match.expected_tokens,
                )],
            },
            ParserResult::IncompleteMatch(IncompleteMatch {
                nodes,
                expected_tokens,
            })
            | ParserResult::Match(Match {
                nodes,
                expected_tokens,
            }) => {
                let topmost_rule = match &nodes[..] {
                    [cst::Node::Rule(rule)] => Rc::clone(&rule),
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
                if start.utf8 < input.len() || is_incomplete {
                    let skipped_node =
                        cst::Node::token(TokenKind::SKIPPED, input[start.utf8..].to_string());
                    let mut new_children = topmost_rule.children.clone();
                    new_children.push(skipped_node);
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
                    ParseOutput {
                        parse_tree: cst::Node::Rule(topmost_rule),
                        errors,
                    }
                }
            }
            ParserResult::PrattOperatorMatch(..) => unreachable!("PrattOperatorMatch is internal"),
        }
    }
}
