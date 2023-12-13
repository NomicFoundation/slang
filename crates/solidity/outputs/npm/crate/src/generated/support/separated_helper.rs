// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst;
use crate::kinds::{IsLexicalContext, TokenKind};
use crate::lexer::Lexer;
use crate::parse_error::ParseError;
use crate::support::parser_result::{ParserResult, SkippedUntil};
use crate::support::recovery::skip_until_with_nested_delims;
use crate::support::ParserContext;
use crate::text_index::TextRangeExtensions;

pub struct SeparatedHelper;

impl SeparatedHelper {
    pub fn run<L: Lexer, LexCtx: IsLexicalContext>(
        input: &mut ParserContext<'_>,
        lexer: &L,
        body_parser: impl Fn(&mut ParserContext<'_>) -> ParserResult,
        separator: TokenKind,
    ) -> ParserResult {
        let mut accum = vec![];
        loop {
            match body_parser(input) {
                ParserResult::Match(r#match) => {
                    accum.extend(r#match.nodes);

                    match lexer.peek_token_with_trivia::<LexCtx>(input) {
                        Some(token) if token == separator => {
                            match lexer.parse_token_with_trivia::<LexCtx>(input, separator) {
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

                    match skip_until_with_nested_delims::<_, LexCtx>(input, lexer, separator) {
                        // A separator was found, so we can recover the incomplete match
                        Some((found, skipped_range)) if found == separator => {
                            accum.push((
                                String::new(),
                                cst::Node::token(
                                    TokenKind::SKIPPED,
                                    input.content(skipped_range.utf8()),
                                ),
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

                            return ParserResult::incomplete_match(
                                accum,
                                incomplete.expected_tokens,
                            );
                        }
                    }
                }
                ParserResult::NoMatch(no_match) => {
                    return if accum.is_empty() {
                        ParserResult::no_match(no_match.expected_tokens)
                    } else {
                        ParserResult::incomplete_match(accum, no_match.expected_tokens)
                    };
                }

                ParserResult::SkippedUntil(skipped) => {
                    accum.extend(skipped.nodes);

                    return ParserResult::SkippedUntil(SkippedUntil {
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
