// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst;
use crate::kinds::{IsLexicalContext, TokenKind};
use crate::lexer::Lexer;
use crate::parse_error::ParseError;
use crate::support::ParserResult;
use crate::text_index::{TextRange, TextRangeExtensions as _};

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
    input: &mut ParserContext<'_>,
    parse: impl Fn(&mut ParserContext<'_>) -> ParserResult,
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
    pub fn recover_until_with_nested_delims<L: Lexer, LexCtx: IsLexicalContext>(
        self,
        input: &mut ParserContext<'_>,
        lexer: &L,
        expected: TokenKind,
        recover_from_no_match: RecoverFromNoMatch,
    ) -> ParserResult {
        let before_recovery = input.position();

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
            ParserResult::Match(result)
                if lexer.peek_token_with_trivia::<LexCtx>(input) != Some(expected) =>
            {
                (result.nodes, result.expected_tokens, ParseResultKind::Match)
            }
            ParserResult::NoMatch(result) if recover_from_no_match.as_bool() => {
                (vec![], result.expected_tokens, ParseResultKind::NoMatch)
            }
            // No need to recover, so just return as-is.
            _ => return self,
        };

        let leading_trivia = opt_parse(input, |input| lexer.leading_trivia(input));

        match skip_until_with_nested_delims::<_, LexCtx>(input, lexer, expected) {
            Some((found, skipped_range)) => {
                nodes.extend(leading_trivia);
                if matches!(result_kind, ParseResultKind::Match) {
                    expected_tokens.push(expected);
                }

                let skipped = input.content(skipped_range.utf8());

                input.emit(ParseError {
                    text_range: skipped_range,
                    tokens_that_would_have_allowed_more_progress: expected_tokens.clone(),
                });

                ParserResult::SkippedUntil(SkippedUntil {
                    nodes,
                    expected,
                    skipped,
                    found,
                })
            }
            // Not found till EOF, revert any recovery attempt
            None => {
                input.set_position(before_recovery);

                match result_kind {
                    ParseResultKind::Match => ParserResult::r#match(nodes, expected_tokens),
                    ParseResultKind::Incomplete => {
                        ParserResult::incomplete_match(nodes, expected_tokens)
                    }
                    ParseResultKind::NoMatch => ParserResult::no_match(expected_tokens),
                }
            }
        }
    }
}

/// Skips tokens until a given token is found or until we hit a closing delimiter that's expected by an outer parse.
/// Respects nested delimiters, i.e. the `expected` token is only accepted if it's not nested inside.
/// Does not consume the `expected` token.
///
/// Returns the found token and the range of skipped tokens on success.
pub fn skip_until_with_nested_delims<L: Lexer, LexCtx: IsLexicalContext>(
    input: &mut ParserContext<'_>,
    lexer: &L,
    until: TokenKind,
) -> Option<(TokenKind, TextRange)> {
    let delims = L::delimiters::<LexCtx>();

    let start = input.position();

    let mut local_delims = vec![];
    loop {
        let save = input.position();
        match lexer.next_token::<LexCtx>(input) {
            // If we're not skipping past a local delimited group (delimiter stack is empty),
            // we can unwind on a token that's expected by us or by our ancestor.
            Some(token)
                if local_delims.is_empty()
                    && (token == until || input.closing_delimiters().contains(&token)) =>
            {
                // Don't consume the delimiter; parent will consume it
                input.set_position(save);

                return Some((token, start..save));
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
            // EOF, revert the cursor
            None => {
                input.set_position(start);

                return None;
            }
        }
    }
}
