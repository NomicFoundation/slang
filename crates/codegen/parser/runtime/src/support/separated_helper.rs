use crate::{
    cst, kinds::TokenKind, lexer::Lexer, parse_error::ParseError, text_index::TextRangeExtensions,
};

use super::{
    parser_result::IncompleteMatch, skip_until_with_nested_delims, ParserContext, ParserResult,
};

pub struct SeparatedHelper;

impl SeparatedHelper {
    pub fn run<const LEX_CTX: u8, L: Lexer>(
        input: &mut ParserContext,
        body_parser: impl Fn(&mut ParserContext) -> ParserResult,
        separator: TokenKind,
        lexer: &L,
    ) -> ParserResult {
        let mut accum = vec![];
        loop {
            match body_parser(input) {
                ParserResult::Match(r#match) => {
                    accum.extend(r#match.nodes);

                    // Parse the leading trivia so that we can peek the next significant token
                    if let ParserResult::Match(r#match) = lexer.leading_trivia(input) {
                        accum.extend(r#match.nodes);
                    }

                    match lexer.peek_token::<LEX_CTX>(input) {
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
                // Body was partially parsed, so try to recover by skipping tokens until we see a separator
                ParserResult::IncompleteMatch(incomplete) => {
                    accum.extend(incomplete.nodes);

                    let start = input.position();

                    let skipped = skip_until_with_nested_delims(
                        input,
                        |input| lexer.next_token::<LEX_CTX>(input),
                        separator,
                        <L as Lexer>::delimiters::<LEX_CTX>(),
                    );

                    match skipped {
                        // A separator was found, so we can recover the incomplete match
                        Some((found, skipped_range)) if found == separator => {
                            accum.push(cst::Node::token(
                                TokenKind::SKIPPED,
                                input.content(skipped_range.utf8()),
                            ));
                            input.emit(ParseError {
                                text_range: skipped_range,
                                tokens_that_would_have_allowed_more_progress: incomplete
                                    .expected_tokens,
                            });

                            match lexer.parse_token_with_trivia::<LEX_CTX>(input, separator) {
                                ParserResult::Match(r#match) => {
                                    accum.extend(r#match.nodes);
                                    continue;
                                }
                                _ => unreachable!("We just checked that the separator matches"),
                            }
                        }
                        // Didn't find a separator during recovery. It might've been the last of the
                        // separatees, so we can't recover to not risk misparses.
                        Some(..) => {
                            // Undo the recovery attempt
                            input.set_position(start);

                            return ParserResult::IncompleteMatch(IncompleteMatch {
                                nodes: accum,
                                expected_tokens: incomplete.expected_tokens,
                            });
                        }
                        // Separator wasn't found till EOF, so we can't recover
                        None => {
                            return ParserResult::IncompleteMatch(IncompleteMatch {
                                nodes: accum,
                                expected_tokens: incomplete.expected_tokens,
                            });
                        }
                    }
                }
                ParserResult::NoMatch(no_match) => {
                    if accum.len() > 0 {
                        return ParserResult::incomplete_match(accum, no_match.expected_tokens);
                    } else {
                        return ParserResult::no_match(no_match.expected_tokens);
                    }
                }

                ParserResult::SkippedUntil(skipped) => {
                    accum.extend(skipped.nodes);
                    return ParserResult::SkippedUntil(super::parser_result::SkippedUntil {
                        nodes: accum,
                        ..skipped
                    });
                }

                ParserResult::PrattOperatorMatch(..) => {
                    unreachable!("PrattOperatorMatch in SeparatedHelper")
                }
            }
        }
    }
}
