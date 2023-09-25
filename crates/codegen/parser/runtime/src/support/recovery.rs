use crate::cst;
use crate::kinds::TokenKind;
use crate::parse_error::ParseError;
use crate::support::ParserResult;
use crate::text_index::TextRangeExtensions as _;

use super::parser_result::SkippedUntil;
use super::ParserContext;

/// An explicit parameter for the [`ParserResult::recover_until_with_nested_delims`] method.
pub enum RecoverFromNoMatch {
    Yes,
    No,
}

impl RecoverFromNoMatch {
    pub fn as_bool(&self) -> bool {
        matches!(self, RecoverFromNoMatch::Yes)
    }
}

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

impl ParserResult {
    /// For partial matches (partial prefix match or if the next token after the match is not expected)
    /// attempts to skip tokens until a given token is found or until we hit a delimiter that's expected
    /// by an outer parse. Returns [`ParserResult::SkippedUntil`] on success.
    ///
    /// Respects nested delimiters, i.e. the `expected` token is only accepted if it's not nested inside.
    /// Does not consume the `expected` token.
    pub fn recover_until_with_nested_delims(
        self,
        input: &mut ParserContext,
        next_token: impl Fn(&mut ParserContext) -> Option<TokenKind>,
        leading_trivia: impl Fn(&mut ParserContext) -> ParserResult,
        expected: TokenKind,
        delims: &[(TokenKind, TokenKind)],
        recover_from_no_match: RecoverFromNoMatch,
    ) -> ParserResult {
        let before_recovery = input.position();

        let mut peek_token_after_trivia = || {
            let start = input.position();

            opt_parse(input, &leading_trivia);
            let token = next_token(input);

            input.set_position(start);
            token
        };

        enum ParseResultKind {
            Match,
            Incomplete,
            NoMatch,
        }

        let (mut nodes, mut expected_tokens, result_kind) = match self {
            ParserResult::IncompleteMatch(result) => (
                result.nodes,
                result.expected_tokens,
                ParseResultKind::Incomplete,
            ),
            ParserResult::Match(result) if peek_token_after_trivia() != Some(expected) => {
                (result.nodes, result.expected_tokens, ParseResultKind::Match)
            }
            ParserResult::NoMatch(result) if recover_from_no_match.as_bool() => {
                (vec![], result.expected_tokens, ParseResultKind::NoMatch)
            }
            // No need to recover, so just return as-is.
            _ => return self,
        };

        let leading_trivia = opt_parse(input, &leading_trivia);
        let start = input.position();

        let mut local_delims = vec![];
        loop {
            let save = input.position();
            match next_token(input) {
                // If we're not skipping past a local delimited group (delimiter stack is empty),
                // we can unwind on a token that's expected by us or by our ancestor.
                Some(token)
                    if local_delims.is_empty()
                        && (token == expected || input.closing_delimiters().contains(&token)) =>
                {
                    nodes.extend(leading_trivia);
                    if matches!(result_kind, ParseResultKind::Match) {
                        expected_tokens.push(expected);
                    }

                    // Don't consume the delimiter; parent will consume it
                    input.set_position(save);

                    let skipped_range = start..save;
                    input.emit(ParseError {
                        text_range: skipped_range.clone(),
                        tokens_that_would_have_allowed_more_progress: expected_tokens.clone(),
                    });

                    return ParserResult::SkippedUntil(SkippedUntil {
                        nodes,
                        expected,
                        skipped: input.content(skipped_range.utf8()),
                        found: token,
                    });
                }
                // Found the local closing delimiter, pop the stack
                Some(token) if local_delims.last() == Some(&token) => {
                    local_delims.pop();
                }
                Some(token) => {
                    // Found a local opening delimiter, skip until we find a closing one
                    if let Some((_, close)) = delims.iter().find(|(op, _)| token == *op) {
                        local_delims.push(*close);
                    } else {
                        // Keep eating (eventually hits EOF)
                    }
                }
                // EOF, revert any recovery attempt
                None => {
                    input.set_position(before_recovery);

                    return match result_kind {
                        ParseResultKind::Match => ParserResult::r#match(nodes, expected_tokens),
                        ParseResultKind::Incomplete => {
                            ParserResult::incomplete_match(nodes, expected_tokens)
                        }
                        ParseResultKind::NoMatch => ParserResult::no_match(expected_tokens),
                    };
                }
            }
        }
    }
}
