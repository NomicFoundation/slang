use super::{ParserContext, ParserResult, SequenceHelper, ZeroOrMoreHelper};

pub struct SeparatedHelper;

impl SeparatedHelper {
    pub fn run(
        input: &mut ParserContext,
        body_parser: impl Fn(&mut ParserContext) -> ParserResult,
        separator_parser: impl Fn(&mut ParserContext) -> ParserResult,
    ) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(body_parser(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(separator_parser(input))?;
                    seq.elem(body_parser(input))?;
                    seq.finish()
                })
            }));
            seq.finish()
        })
    }
}
