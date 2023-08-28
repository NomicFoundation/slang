use super::parser_result::ParserResult;

pub struct OptionalHelper;

impl OptionalHelper {
    pub fn transform(result: ParserResult) -> ParserResult {
        match result {
            // If there's absolutely no match, we treat it as a match (for the purposes of the Result algebra)
            // but we bubble up the information which tokens would have allowed more progress.
            ParserResult::NoMatch(no_match) => {
                ParserResult::r#match(vec![], no_match.expected_tokens)
            }
            // ... otherwise we return the result as-is
            ParserResult::Match(_)
            | ParserResult::PrattOperatorMatch(_)
            | ParserResult::IncompleteMatch(_) => result,
        }
    }
}
