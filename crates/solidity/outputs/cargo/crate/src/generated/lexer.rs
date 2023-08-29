// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::{
    cst,
    kinds::{LexicalContext, TokenKind},
    support::{ParserResult, Stream},
    text_index::TextRange,
};

// Ensure that the `LexicalContext` enum is `repr(u8)`.
// Workaround until repr(u8) enums can be used as const params.
const _ASSERT_CONTEXT_IS_REPR_U8: fn() = || {
    let _ = core::mem::transmute::<u8, LexicalContext>;
};

pub trait Lexer {
    // Generated by the templating engine
    #[doc(hidden)]
    fn next_token<const LEX_CTX: u8>(&self, stream: &mut Stream) -> Option<TokenKind>;
    // NOTE: These are context-insensitive
    #[doc(hidden)]
    fn leading_trivia(&self, stream: &mut Stream) -> ParserResult;
    #[doc(hidden)]
    fn trailing_trivia(&self, stream: &mut Stream) -> ParserResult;

    fn parse_token<const LEX_CTX: u8>(&self, stream: &mut Stream, kind: TokenKind) -> ParserResult {
        let start = stream.position();
        if self.next_token::<LEX_CTX>(stream) != Some(kind) {
            stream.set_position(start);
            return ParserResult::no_match(vec![kind]);
        }
        let end = stream.position();

        ParserResult::r#match(
            vec![cst::Node::token(kind, stream.content(start.utf8..end.utf8))],
            vec![],
        )
    }

    fn parse_token_with_trivia<const LEX_CTX: u8>(
        &self,
        stream: &mut Stream,
        kind: TokenKind,
    ) -> ParserResult {
        let mut children = vec![];

        let restore = stream.position();
        if let ParserResult::Match(r#match) = self.leading_trivia(stream) {
            children.extend(r#match.nodes);
        } else {
            stream.set_position(restore);
        }

        let start = stream.position();
        if self.next_token::<LEX_CTX>(stream) != Some(kind) {
            stream.set_position(restore);
            return ParserResult::no_match(vec![kind]);
        }
        let end = stream.position();
        children.push(cst::Node::token(kind, stream.content(start.utf8..end.utf8)));

        let restore = stream.position();
        if let ParserResult::Match(r#match) = self.trailing_trivia(stream) {
            children.extend(r#match.nodes);
        } else {
            stream.set_position(restore);
        }

        ParserResult::r#match(children, vec![])
    }

    fn skip_tokens_until<const LEX_CTX: u8>(
        &self,
        stream: &mut Stream,
        expected: TokenKind,
    ) -> Option<TextRange> {
        let start = stream.position();

        loop {
            let restore = stream.position();

            match self.next_token::<LEX_CTX>(stream) {
                // Keep eating tokens until we find the expected one
                Some(kind) if kind != expected => {}
                // Token found, return the skipped range but don't consume the expected token
                Some(..) => {
                    stream.set_position(restore);

                    break Some(start..restore);
                }
                // Not found till EOF
                None => {
                    stream.set_position(start);

                    break None;
                }
            }
        }
    }
}
