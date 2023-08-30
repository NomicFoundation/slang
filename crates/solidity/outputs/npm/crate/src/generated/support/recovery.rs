// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst;
use crate::kinds::TokenKind;
use crate::parse_error::ParseError;
use crate::support::ParserResult;
use crate::text_index::{TextRange, TextRangeExtensions as _};

use super::ParserContext;

impl ParserResult {
    pub fn try_recover_with(
        self,
        input: &mut ParserContext,
        skip_tokens_for_recovery: impl Fn(&mut ParserContext) -> Option<TextRange>,
    ) -> ParserResult {
        match self {
            ParserResult::IncompleteMatch(mut result) => {
                if let Some(skipped) = skip_tokens_for_recovery(input) {
                    result.nodes.push(cst::Node::token(
                        TokenKind::SKIPPED,
                        input.content(skipped.utf8()),
                    ));

                    input.emit(ParseError {
                        text_range: skipped,
                        tokens_that_would_have_allowed_more_progress: result
                            .expected_tokens
                            .clone(),
                    });
                }

                ParserResult::r#match(result.nodes, result.expected_tokens)
            }
            result => result,
        }
    }

    pub fn recover_until_with_nested_delims(
        self,
        input: &mut ParserContext,
        next_token: impl Fn(&mut ParserContext) -> Option<TokenKind>,
        expected: TokenKind,
        delimiters: &[(TokenKind, TokenKind)],
    ) -> ParserResult {
        let start = input.position();

        match self {
            ParserResult::IncompleteMatch(mut result) => {
                let mut stack = vec![];

                loop {
                    let save = input.position();
                    match next_token(input) {
                        // Found the expected token
                        Some(token) if stack.is_empty() && token == expected => {
                            // TODO: Consume the stream?

                            // Don't consume the delimiter; parent will consume it
                            input.set_position(save);

                            let text_range = start..save;
                            result.nodes.push(cst::Node::token(
                                TokenKind::SKIPPED,
                                input.content(text_range.utf8()),
                            ));
                            input.emit(ParseError {
                                text_range,
                                tokens_that_would_have_allowed_more_progress: result
                                    .expected_tokens
                                    .clone(),
                            });

                            return ParserResult::r#match(result.nodes, result.expected_tokens);
                        }
                        // Found a closing delimiter that's expected by the parent - unwind the parse stack
                        Some(token)
                            if stack.is_empty() && input.closing_delimiters().contains(&token) =>
                        {
                            // Don't consume the delimiter; parent will consume it
                            input.set_position(save);

                            let text_range = start..save;
                            result.nodes.push(cst::Node::token(
                                TokenKind::SKIPPED,
                                input.content(text_range.utf8()),
                            ));
                            input.emit(ParseError {
                                text_range,
                                tokens_that_would_have_allowed_more_progress: result
                                    .expected_tokens
                                    .clone(),
                            });

                            // TODO: Unwind the parse stack, rather than returning match here
                            return ParserResult::r#match(result.nodes, result.expected_tokens);
                        }
                        // Found the local closing delimiter, pop the stack
                        Some(token) if stack.last() == Some(&token) => {
                            stack.pop();
                        }
                        // Found a local opening delimiter, push onto stack
                        Some(token) if delimiters.iter().any(|(open, _)| token == *open) => {
                            stack.push(token);
                        }
                        Some(..) => {} // Keep eating (eventually hits EOF)
                        // EOF
                        None => {
                            // TODO: Investigate whether we can fully consume here

                            let text_range = start..save;
                            result.nodes.push(cst::Node::token(
                                TokenKind::SKIPPED,
                                input.content(text_range.utf8()),
                            ));
                            input.emit(ParseError {
                                text_range,
                                tokens_that_would_have_allowed_more_progress: result
                                    .expected_tokens
                                    .clone(),
                            });

                            return ParserResult::r#match(result.nodes, result.expected_tokens);
                        }
                    }
                }
            }
            // TODO: What to do about no match?
            result => result,
        }
    }
}
