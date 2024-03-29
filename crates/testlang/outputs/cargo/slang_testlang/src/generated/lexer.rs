// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst::{self, LabeledNode};
use crate::kinds::{IsLexicalContext, TokenKind};
use crate::parser_support::{ParserContext, ParserResult};

/// Whether a keyword has been scanned and if so, whether it is reserved (unusable as an identifier)
/// or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordScan {
    /// The keyword is not present.
    Absent,
    /// The keyword is present, but is not reserved.
    #[allow(unused)]
    Present(TokenKind),
    /// The keyword is present and is reserved.
    Reserved(TokenKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScannedToken {
    Single(TokenKind),
    IdentifierOrKeyword {
        identifier: TokenKind,
        kw: KeywordScan,
    },
}

impl ScannedToken {
    pub fn accepted_as(self, expected: TokenKind) -> bool {
        match self {
            Self::Single(kind) => kind == expected,
            Self::IdentifierOrKeyword { identifier, kw } => match kw {
                KeywordScan::Reserved(kind) => kind == expected,
                KeywordScan::Present(kind) => kind == expected || identifier == expected,
                KeywordScan::Absent => identifier == expected,
            },
        }
    }

    /// Returns the most general token kind that can be accepted for the scanned token.
    ///
    /// If the scanned token is an identifier, returns the specific keyword kind if the keyword is reserved,
    /// otherwise returns the general identifier kind. For other tokens, returns the token kind itself.
    pub fn unambiguous(self) -> TokenKind {
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
    fn next_token<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
    ) -> Option<ScannedToken>;
    // NOTE: These are context-insensitive
    #[doc(hidden)]
    fn leading_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult;
    #[doc(hidden)]
    fn trailing_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult;
    #[doc(hidden)]
    /// Returns valid grouping delimiters in the given lexical context.
    fn delimiters<LexCtx: IsLexicalContext>() -> &'static [(TokenKind, TokenKind)];

    /// Peeks the next token, including trivia. Does not advance the input.
    fn peek_token<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
    ) -> Option<ScannedToken> {
        let start = input.position();
        let token = self.next_token::<LexCtx>(input);
        input.set_position(start);
        token
    }

    /// Peeks the next significant (i.e. non-trivia) token. Does not advance the input.
    fn peek_token_with_trivia<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
    ) -> Option<ScannedToken> {
        let start = input.position();

        let _ = self.leading_trivia(input);
        let token = self.next_token::<LexCtx>(input);

        input.set_position(start);
        token
    }

    /// Attempts to consume the next expected token. Advances the input only if the token matches.
    fn parse_token<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
        kind: TokenKind,
    ) -> ParserResult {
        let start = input.position();
        if !self
            .next_token::<LexCtx>(input)
            .is_some_and(|t| t.accepted_as(kind))
        {
            input.set_position(start);
            return ParserResult::no_match(vec![kind]);
        }
        let end = input.position();

        ParserResult::r#match(
            vec![LabeledNode::anonymous(cst::Node::token(
                kind,
                input.content(start.utf8..end.utf8),
            ))],
            vec![],
        )
    }

    /// Attempts to consume the next significant token including both leading and trailing trivia.
    /// Advances the input only if the token matches.
    fn parse_token_with_trivia<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
        kind: TokenKind,
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
            .next_token::<LexCtx>(input)
            .is_some_and(|t| t.accepted_as(kind))
        {
            input.set_position(restore);
            return ParserResult::no_match(vec![kind]);
        }
        let end = input.position();
        children.push(LabeledNode::anonymous(cst::Node::token(
            kind,
            input.content(start.utf8..end.utf8),
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
