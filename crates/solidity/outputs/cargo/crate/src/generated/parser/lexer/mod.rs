// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst::{Edge, IsLexicalContext, Node, TerminalKind};
use crate::parser::parser_support::{ParserContext, ParserResult};

/// Whether a keyword has been scanned and if so, whether it is reserved (unusable as an identifier)
/// or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordScan {
    /// The keyword is not present.
    Absent,
    /// The keyword is present, but is not reserved.
    #[allow(unused)]
    Present(TerminalKind),
    /// The keyword is present and is reserved.
    Reserved(TerminalKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScannedTerminal {
    Single(TerminalKind),
    IdentifierOrKeyword {
        identifier: TerminalKind,
        kw: KeywordScan,
    },
}

impl ScannedTerminal {
    pub fn accepted_as(self, expected: TerminalKind) -> bool {
        match self {
            Self::Single(kind) => kind == expected,
            Self::IdentifierOrKeyword { identifier, kw } => match kw {
                KeywordScan::Reserved(kind) => kind == expected,
                KeywordScan::Present(kind) => kind == expected || identifier == expected,
                KeywordScan::Absent => identifier == expected,
            },
        }
    }

    /// Returns the most general terminal kind that can be accepted for the scanned terminal.
    ///
    /// If the scanned terminal is an identifier, returns the specific keyword kind if the keyword is reserved,
    /// otherwise returns the general identifier kind. For other terminals, returns the terminal kind itself.
    pub fn unambiguous(self) -> TerminalKind {
        match self {
            Self::Single(kind) => kind,
            Self::IdentifierOrKeyword { identifier, kw } => match kw {
                KeywordScan::Reserved(kind) => kind,
                // Ambiguous; prefer using the more general identifier
                KeywordScan::Present(..) => identifier,
                KeywordScan::Absent => identifier,
            },
        }
    }
}

pub(crate) trait Lexer {
    // Generated by the templating engine
    #[doc(hidden)]
    fn next_terminal<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
    ) -> Option<ScannedTerminal>;
    // NOTE: These are context-insensitive
    #[doc(hidden)]
    fn leading_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult;
    #[doc(hidden)]
    fn trailing_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult;
    #[doc(hidden)]
    /// Returns valid grouping delimiters in the given lexical context.
    fn delimiters<LexCtx: IsLexicalContext>() -> &'static [(TerminalKind, TerminalKind)];

    /// Peeks the next significant (i.e. non-trivia) terminal. Does not advance the input.
    fn peek_terminal_with_trivia<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
    ) -> Option<ScannedTerminal> {
        let start = input.position();

        let _ = self.leading_trivia(input);
        let terminal = self.next_terminal::<LexCtx>(input);

        input.set_position(start);
        terminal
    }

    /// Attempts to consume the next expected terminal. Advances the input only if the terminal matches.
    fn parse_terminal<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
        kind: TerminalKind,
    ) -> ParserResult {
        let start = input.position();
        if !self
            .next_terminal::<LexCtx>(input)
            .is_some_and(|t| t.accepted_as(kind))
        {
            input.set_position(start);
            return ParserResult::no_match(vec![kind]);
        }
        let end = input.position();

        ParserResult::r#match(
            vec![Edge::root(Node::terminal(
                kind,
                input.content(start..end).to_owned(),
            ))],
            vec![],
        )
    }

    /// Attempts to consume the next significant terminal including both leading and trailing trivia.
    /// Advances the input only if the terminal matches.
    fn parse_terminal_with_trivia<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
        kind: TerminalKind,
    ) -> ParserResult {
        let mut children = vec![];

        let restore = input.position();
        if let ParserResult::Match(r#match) = self.leading_trivia(input) {
            children.extend(r#match.nodes);
        } else {
            input.set_position(restore);
        }

        let start = input.position();
        if !self
            .next_terminal::<LexCtx>(input)
            .is_some_and(|t| t.accepted_as(kind))
        {
            input.set_position(restore);
            return ParserResult::no_match(vec![kind]);
        }
        let end = input.position();
        children.push(Edge::root(Node::terminal(
            kind,
            input.content(start..end).to_owned(),
        )));

        let restore = input.position();
        if let ParserResult::Match(r#match) = self.trailing_trivia(input) {
            children.extend(r#match.nodes);
        } else {
            input.set_position(restore);
        }

        ParserResult::r#match(children, vec![])
    }
}
