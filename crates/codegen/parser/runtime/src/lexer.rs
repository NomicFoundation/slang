use crate::{
    cst,
    kinds::{LexicalContext, TokenKind},
    support::{ParserContext, ParserResult},
};

// Ensure that the `LexicalContext` enum is `repr(u8)`.
// Workaround until repr(u8) enums can be used as const params.
const _ASSERT_CONTEXT_IS_REPR_U8: fn() = || {
    let _ = core::mem::transmute::<u8, LexicalContext>;
};

pub trait Lexer {
    // Generated by the templating engine
    #[doc(hidden)]
    fn next_token<const LEX_CTX: u8>(&self, input: &mut ParserContext) -> Option<TokenKind>;
    // NOTE: These are context-insensitive
    #[doc(hidden)]
    fn leading_trivia(&self, input: &mut ParserContext) -> ParserResult;
    #[doc(hidden)]
    fn trailing_trivia(&self, input: &mut ParserContext) -> ParserResult;
    #[doc(hidden)]
    fn delimiters<const LEX_CTX: u8>() -> &'static [(TokenKind, TokenKind)];

    fn peek_token<const LEX_CTX: u8>(&self, input: &mut ParserContext) -> Option<TokenKind> {
        let start = input.position();
        let token = self.next_token::<LEX_CTX>(input);
        input.set_position(start);
        token
    }

    fn parse_token<const LEX_CTX: u8>(
        &self,
        input: &mut ParserContext,
        kind: TokenKind,
    ) -> ParserResult {
        let start = input.position();
        if self.next_token::<LEX_CTX>(input) != Some(kind) {
            input.set_position(start);
            return ParserResult::no_match(vec![kind]);
        }
        let end = input.position();

        ParserResult::r#match(
            vec![cst::Node::token(kind, input.content(start.utf8..end.utf8))],
            vec![],
        )
    }

    fn parse_token_with_trivia<const LEX_CTX: u8>(
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
        if self.next_token::<LEX_CTX>(input) != Some(kind) {
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
