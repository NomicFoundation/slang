use crate::cst::{Edge, IsLexicalContext, TerminalKind, TextRange, TextRangeExtensions};
use crate::parser::lexer::{Lexer, ScannedTerminal};
use crate::parser::parser_support::context::ParserContext;
use crate::parser::parser_support::parser_result::SkippedUntil;
use crate::parser::parser_support::ParserResult;
use crate::parser::ParseError;

fn opt_parse(
    input: &mut ParserContext<'_>,
    parse: impl Fn(&mut ParserContext<'_>) -> ParserResult,
) -> Vec<Edge> {
    let start = input.position();
    if let ParserResult::Match(r#match) = parse(input) {
        r#match.nodes
    } else {
        input.set_position(start);
        vec![]
    }
}

impl ParserResult {
    /// For partial matches (partial prefix match or if the next terminal after the match is not expected)
    /// attempts to skip terminals until a given terminal is found or until we hit a delimiter that's expected
    /// by an outer parse. Returns [`ParserResult::SkippedUntil`] on success.
    ///
    /// Respects nested delimiters, i.e. the `expected` terminal is only accepted if it's not nested inside.
    /// Does not consume the `expected` terminal.
    #[must_use]
    pub(crate) fn recover_until_with_nested_delims<L: Lexer, LexCtx: IsLexicalContext>(
        self,
        input: &mut ParserContext<'_>,
        lexer: &L,
        expected: TerminalKind,
    ) -> ParserResult {
        enum ParseResultKind {
            Match,
            Incomplete,
            NoMatch,
        }

        let before_recovery = input.position();

        let (mut nodes, mut expected_terminals, result_kind) = match self {
            ParserResult::IncompleteMatch(result) => (
                result.nodes,
                result.expected_terminals,
                ParseResultKind::Incomplete,
            ),
            ParserResult::Match(result)
                if lexer
                    .peek_terminal_with_trivia::<LexCtx>(input)
                    .map(ScannedTerminal::unambiguous)
                    != Some(expected) =>
            {
                (
                    result.nodes,
                    result.expected_terminals,
                    ParseResultKind::Match,
                )
            }
            ParserResult::NoMatch(result) => {
                (vec![], result.expected_terminals, ParseResultKind::NoMatch)
            }
            // No need to recover, so just return as-is.
            _ => return self,
        };

        let leading_trivia = opt_parse(input, |input| lexer.leading_trivia(input));

        if let Some((found, skipped_range)) =
            skip_until_with_nested_delims::<_, LexCtx>(input, lexer, expected)
        {
            nodes.extend(leading_trivia);
            if matches!(result_kind, ParseResultKind::Match) {
                expected_terminals.push(expected);
            }

            let skipped = input.content(skipped_range.utf8()).to_owned();

            input.emit(ParseError::create(
                skipped_range,
                expected_terminals.clone(),
            ));

            ParserResult::SkippedUntil(SkippedUntil {
                nodes,
                skipped,
                found,
                expected,
            })
        } else {
            // Not found till EOF, revert any recovery attempt
            input.set_position(before_recovery);

            match result_kind {
                ParseResultKind::Match => ParserResult::r#match(nodes, expected_terminals),
                ParseResultKind::Incomplete => {
                    ParserResult::incomplete_match(nodes, expected_terminals)
                }
                ParseResultKind::NoMatch => ParserResult::no_match(expected_terminals),
            }
        }
    }
}

/// Skips terminals until a given terminal is found or until we hit a closing delimiter that's expected by an outer parse.
/// Respects nested delimiters, i.e. the `expected` terminal is only accepted if it's not nested inside.
/// Does not consume the `expected` terminal.
///
/// Returns the found terminal and the range of skipped terminals on success.
pub(crate) fn skip_until_with_nested_delims<L: Lexer, LexCtx: IsLexicalContext>(
    input: &mut ParserContext<'_>,
    lexer: &L,
    until: TerminalKind,
) -> Option<(TerminalKind, TextRange)> {
    let delims = L::delimiters::<LexCtx>();

    let start = input.position();

    let mut local_delims = vec![];
    loop {
        let save = input.position();
        match lexer
            .next_terminal::<LexCtx>(input)
            .map(ScannedTerminal::unambiguous)
        {
            // If we're not skipping past a local delimited group (delimiter stack is empty),
            // we can unwind on a terminal that's expected by us or by our ancestor.
            Some(terminal)
                if local_delims.is_empty()
                    && (terminal == until || input.closing_delimiters().contains(&terminal)) =>
            {
                // Don't consume the delimiter; parent will consume it
                input.set_position(save);

                let start_index = input.text_index_at(start);
                let save_index = input.text_index_at(save);
                return Some((terminal, start_index..save_index));
            }
            // Found the local closing delimiter, pop the stack
            Some(terminal) if local_delims.last() == Some(&terminal) => {
                local_delims.pop();
            }
            Some(terminal) => {
                // Found a local opening delimiter, skip until we find a closing one
                if let Some((_, close)) = delims.iter().find(|(op, _)| terminal == *op) {
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
