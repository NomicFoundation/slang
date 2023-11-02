// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::{
    cst,
    kinds::{IsLexicalContext, TokenKind},
    support::{ParserContext, ParserResult},
};

pub trait Lexer {
    // Generated by the templating engine
    #[doc(hidden)]
    fn next_token<LexCtx: IsLexicalContext>(&self, input: &mut ParserContext) -> Option<TokenKind>;
    // NOTE: These are context-insensitive
    #[doc(hidden)]
    fn leading_trivia(&self, input: &mut ParserContext) -> ParserResult;
    #[doc(hidden)]
    fn trailing_trivia(&self, input: &mut ParserContext) -> ParserResult;
    #[doc(hidden)]
    /// Returns valid grouping delimiters in the given lexical context.
    fn delimiters<LexCtx: IsLexicalContext>() -> &'static [(TokenKind, TokenKind)];

    /// Peeks the next token, including trivia. Does not advance the input.
    fn peek_token<LexCtx: IsLexicalContext>(&self, input: &mut ParserContext) -> Option<TokenKind> {
        let start = input.position();
        let token = self.next_token::<LexCtx>(input);
        input.set_position(start);
        token
    }

    /// Peeks the next significant (i.e. non-trivia) token. Does not advance the input.
    fn peek_token_with_trivia<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext,
    ) -> Option<TokenKind> {
        let start = input.position();

        let _ = self.leading_trivia(input);
        let token = self.next_token::<LexCtx>(input);

        input.set_position(start);
        token
    }

    /// Attempts to consume the next expected token. Advances the input only if the token matches.
    fn parse_token<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext,
        kind: TokenKind,
    ) -> ParserResult {
        let start = input.position();
        if self.next_token::<LexCtx>(input) != Some(kind) {
            input.set_position(start);
            return ParserResult::no_match(vec![kind]);
        }
        let end = input.position();

        ParserResult::r#match(
            vec![cst::Node::token(kind, input.content(start.utf8..end.utf8))],
            vec![],
        )
    }

    /// Attempts to consume the next significant token including both leading and trailing trivia.
    /// Advances the input only if the token matches.
    fn parse_token_with_trivia<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext,
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
        if self.next_token::<LexCtx>(input) != Some(kind) {
            input.set_position(restore);
            return ParserResult::no_match(vec![kind]);
        }
        let end = input.position();
        children.push(cst::Node::token(kind, input.content(start.utf8..end.utf8)));

        let restore = input.position();
        if let ParserResult::Match(r#match) = self.trailing_trivia(input) {
            children.extend(r#match.nodes);
        } else {
            input.set_position(restore);
        }

        ParserResult::r#match(children, vec![])
    }
}
