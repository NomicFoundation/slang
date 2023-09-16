// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::{cst, kinds::TokenKind, lexer::Lexer, support::parser_result::IncompleteMatch};

use super::{ParserContext, ParserResult};

pub struct SeparatedHelper;

fn opt_parse(
    input: &mut ParserContext,
    parse: impl Fn(&mut ParserContext) -> ParserResult,
) -> Vec<cst::Node> {
    let start = input.position();
    if let ParserResult::Match(r#match) = parse(input) {
        r#match.nodes
    } else {
        input.set_position(start);
        vec![]
    }
}

impl SeparatedHelper {
    pub fn run<const LEX_CTX: u8, L: Lexer>(
        input: &mut ParserContext,
        body_parser: impl Fn(&mut ParserContext) -> ParserResult,
        separator: TokenKind,
        lexer: &L,
    ) -> ParserResult {
        let peek_token_after_trivia = |input: &mut ParserContext| {
            let start = input.position();

            opt_parse(input, |input| lexer.leading_trivia(input));
            let token = lexer.next_token::<LEX_CTX>(input);

            input.set_position(start);
            token
        };

        let mut accum = vec![];
        loop {
            match body_parser(input) {
                ParserResult::Match(r#match) => {
                    accum.extend(r#match.nodes);

                    match peek_token_after_trivia(input) {
                        Some(token) if token == separator => {
                            let separator =
                                lexer.parse_token_with_trivia::<LEX_CTX>(input, separator);
                            match separator {
                                ParserResult::Match(r#match) => {
                                    accum.extend(r#match.nodes);
                                    continue;
                                }
                                _ => unreachable!("We just checked that the separator matches"),
                            }
                        }
                        // Heuristic: lists (separated-by) are often in a delimited group, so if we
                        // see a closing delimiter, we assume that we're done and don't recover.
                        Some(token) if input.closing_delimiters().contains(&token) => {
                            return ParserResult::r#match(accum, vec![separator]);
                        }
                        // Otherwise, we try to recover from unexpected tokens until we see a separator
                        Some(..) => {
                            // TODO: Attempt recovery: sometimes the list is not in a delimited group,
                            // so don't attempt to recover for now to not risk misparses
                            return ParserResult::r#match(accum, vec![separator]);
                        }
                        // EOF
                        None => {
                            return ParserResult::r#match(accum, vec![separator]);
                        }
                    }
                }
                ParserResult::IncompleteMatch(incomplete) => {
                    accum.extend(incomplete.nodes);
                    return ParserResult::IncompleteMatch(IncompleteMatch {
                        nodes: accum,
                        expected_tokens: incomplete.expected_tokens,
                    });
                }
                ParserResult::NoMatch(no_match) => {
                    if accum.len() > 0 {
                        return ParserResult::incomplete_match(accum, no_match.expected_tokens);
                    } else {
                        return ParserResult::no_match(no_match.expected_tokens);
                    }
                }

                ParserResult::PrattOperatorMatch(..) => {
                    unreachable!("PrattOperatorMatch in SeparatedHelper")
                }

                ParserResult::SkippedUntil(..) => todo!(),
            }
        }
    }
}
