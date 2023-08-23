use super::parser_result::ParserResult;

pub struct OptionalHelper;

impl OptionalHelper {
    pub fn transform(result: ParserResult) -> ParserResult {
        match result {
            ParserResult::Match(_)
            | ParserResult::PrattOperatorMatch(_)
            | ParserResult::IncompleteMatch(_) => result,
            ParserResult::NoMatch(no_match) => ParserResult::r#match(
                vec![],
                no_match.tokens_that_would_have_allowed_more_progress,
            ),
        }
    }
}
