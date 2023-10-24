use crate::{
    cst,
    kinds::{IsLexicalContext, TokenKind},
    lexer::Lexer,
    parse_error::ParseError,
    text_index::TextRangeExtensions,
};

use super::{
    parser_result::IncompleteMatch, skip_until_with_nested_delims, ParserContext, ParserResult,
};

pub struct SeparatedHelper;

impl SeparatedHelper {
    pub fn run<LexCtx: IsLexicalContext, L: Lexer>(
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

                    match lexer.peek_token::<LexCtx>(input) {
                        Some(token) if token == separator => {
                            let separator =
                                lexer.parse_token_with_trivia::<LexCtx>(input, separator);
                            match separator {
                                ParserResult::Match(r#match) => {
                                    accum.extend(r#match.nodes);
                                    continue;
                                }
                                _ => unreachable!("We just checked that the separator matches"),
                            }
                        }

                        // Unrecognized, return the accumulated matches.
                        // NOTE: We can't correctly attempt recovery until #600 lands, otherwise we'd risk misparses,
                        // as we need to stop at certain synchronizing tokens (and we can't reliably scan until
                        // a delimiter, as not every list is enclosed in a delimited group).
                        Some(..) | None => return ParserResult::r#match(accum, vec![separator]),
                    }
                }
                // Body was partially parsed, so try to recover by skipping tokens until we see a separator
                ParserResult::IncompleteMatch(incomplete) => {
                    accum.extend(incomplete.nodes);

                    let start = input.position();

                    let skipped = skip_until_with_nested_delims(
                        input,
                        |input| lexer.next_token::<LexCtx>(input),
                        separator,
                        <L as Lexer>::delimiters::<LexCtx>(),
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

                            match lexer.parse_token_with_trivia::<LexCtx>(input, separator) {
                                ParserResult::Match(r#match) => {
                                    accum.extend(r#match.nodes);
                                    continue;
                                }
                                _ => unreachable!("We just checked that the separator matches"),
                            }
                        }

                        // Didn't find a separator during recovery. It might've been the last of the
                        // separatees, so we can't recover to not risk misparses.
                        Some(..) | None => {
                            // Undo the recovery attempt
                            input.set_position(start);

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
