use crate::{kinds::TokenKind, lexer::Lexer};

use super::{ParserContext, ParserResult, SequenceHelper, ZeroOrMoreHelper};

pub struct SeparatedHelper;

impl SeparatedHelper {
    pub fn run<const LEX_CTX: u8>(
        input: &mut ParserContext,
        body_parser: impl Fn(&mut ParserContext) -> ParserResult,
        separator: TokenKind,
        lexer: &impl Lexer,
    ) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(body_parser(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(lexer.parse_token_with_trivia::<LEX_CTX>(input, separator))?;
                    seq.elem(body_parser(input))?;
                    seq.finish()
                })
            }));
            seq.finish()
        })
    }
}
