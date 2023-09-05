// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::kinds::TokenKind;
use crate::parse_error::ParseError;
use crate::support::ParserResult;
use crate::text_index::TextRangeExtensions as _;

use super::parser_result::SkippedUntil;
use super::ParserContext;

impl ParserResult {
    pub fn recover_until_with_nested_delims(
        self,
        input: &mut ParserContext,
        next_token: impl Fn(&mut ParserContext) -> Option<TokenKind>,
        leading_trivia: impl Fn(&mut ParserContext) -> ParserResult,
        expected: TokenKind,
        delims: &[(TokenKind, TokenKind)],
    ) -> ParserResult {
        let before_recovery = input.position();

        // Parse trivia so that we can compare the next relevant token to the expected
        let leading_trivia = {
            let start = input.position();
            if let ParserResult::Match(leading_trivia) = leading_trivia(input) {
                leading_trivia.nodes
            } else {
                input.set_position(start);
                vec![]
            }
        };

        let start = input.position();

        let mut peek_token = || {
            let start = input.position();
            let tok = next_token(input);
            input.set_position(start);
            tok
        };

        match self {
            ParserResult::IncompleteMatch(mut result) => {
                let mut stack = vec![];

                loop {
                    let save = input.position();
                    match next_token(input) {
                        // Found the expected token
                        Some(token) if stack.is_empty() && token == expected => {
                            result.nodes.extend(leading_trivia);

                            // Don't consume the delimiter; parent will consume it
                            input.set_position(save);

                            let text_range = start..save;

                            input.emit(ParseError {
                                text_range: text_range.clone(),
                                tokens_that_would_have_allowed_more_progress: result
                                    .expected_tokens
                                    .clone(),
                            });

                            return ParserResult::SkippedUntil(SkippedUntil {
                                nodes: result.nodes,
                                expected,
                                skipped: input.content(text_range.utf8()),
                                found: token,
                            });
                        }
                        // Found a closing delimiter that's expected by the parent - unwind the parse stack
                        Some(token)
                            if stack.is_empty() && input.closing_delimiters().contains(&token) =>
                        {
                            result.nodes.extend(leading_trivia);

                            // Don't consume the delimiter; parent will consume it
                            input.set_position(save);

                            let text_range = start..save;
                            input.emit(ParseError {
                                text_range: text_range.clone(),
                                tokens_that_would_have_allowed_more_progress: result
                                    .expected_tokens
                                    .clone(),
                            });

                            // TODO: Unwind the parse stack, rather than returning match here
                            return ParserResult::SkippedUntil(SkippedUntil {
                                nodes: result.nodes,
                                expected,
                                skipped: input.content(text_range.utf8()),
                                found: token,
                            });
                        }
                        // Found the local closing delimiter, pop the stack
                        Some(token) if stack.last() == Some(&token) => {
                            stack.pop();
                        }
                        Some(token) => {
                            // Found a local opening delimiter, push onto stack
                            if let Some((_, close)) = delims.iter().find(|(op, _)| token == *op) {
                                stack.push(*close);
                            } else {
                                // Keep eating (eventually hits EOF)
                            }
                        }
                        // EOF
                        None => {
                            // Undo the stream consumption
                            input.set_position(before_recovery);

                            return ParserResult::IncompleteMatch(result);
                        }
                    }
                }
            }
            // TODO: What to do about no match?
            // We got a match but there are unexpected tokens, so try to recover from them
            ParserResult::Match(mut result) if peek_token() != Some(expected) => {
                let mut stack = vec![];

                loop {
                    let save = input.position();

                    match next_token(input) {
                        // Found the expected token
                        Some(token) if stack.is_empty() && token == expected => {
                            result.nodes.extend(leading_trivia);
                            result.expected_tokens.push(expected);
                            // Don't consume the delimiter; parent will consume it
                            input.set_position(save);

                            let text_range = start..save;

                            input.emit(ParseError {
                                text_range: text_range.clone(),
                                tokens_that_would_have_allowed_more_progress: result
                                    .expected_tokens
                                    .clone(),
                            });

                            return ParserResult::SkippedUntil(SkippedUntil {
                                nodes: result.nodes,
                                expected,
                                skipped: input.content(text_range.utf8()),
                                found: token,
                            });
                        }
                        // Found a closing delimiter that's expected by the parent - unwind the parse stack
                        Some(token)
                            if stack.is_empty() && input.closing_delimiters().contains(&token) =>
                        {
                            result.nodes.extend(leading_trivia);
                            result.expected_tokens.push(expected);

                            // Don't consume the delimiter; parent will consume it
                            input.set_position(save);

                            let text_range = start..save;
                            input.emit(ParseError {
                                text_range: text_range.clone(),
                                tokens_that_would_have_allowed_more_progress: result
                                    .expected_tokens
                                    .clone(),
                            });

                            // TODO: Unwind the parse stack, rather than returning match here
                            return ParserResult::SkippedUntil(SkippedUntil {
                                nodes: result.nodes,
                                expected,
                                skipped: input.content(text_range.utf8()),
                                found: token,
                            });
                        }
                        // Found the local closing delimiter, pop the stack
                        Some(token) if stack.last() == Some(&token) => {
                            stack.pop();
                        }
                        Some(token) => {
                            // Found a local opening delimiter, push onto stack
                            if let Some((_, close)) = delims.iter().find(|(op, _)| token == *op) {
                                stack.push(*close);
                            } else {
                                // Keep eating (eventually hits EOF)
                            }
                        }
                        // EOF
                        None => {
                            // Undo the stream consumption
                            input.set_position(before_recovery);

                            // Let the outer parser handle this
                            return ParserResult::Match(result);
                        }
                    }
                }
            }
            result => {
                // Undo the trivia parse
                input.set_position(before_recovery);
                result
            }
        }
    }
}
