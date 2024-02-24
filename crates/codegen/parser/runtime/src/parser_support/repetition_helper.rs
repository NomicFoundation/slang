use crate::parser_support::context::ParserContext;
use crate::parser_support::parser_result::{IncompleteMatch, NoMatch, ParserResult, PrattElement};

pub struct RepetitionHelper<const MIN_COUNT: usize>;

pub type ZeroOrMoreHelper = RepetitionHelper<0>;
pub type OneOrMoreHelper = RepetitionHelper<1>;

impl<const MIN_COUNT: usize> RepetitionHelper<MIN_COUNT> {
    pub fn run<F: Fn(&mut ParserContext<'_>) -> ParserResult>(
        input: &mut ParserContext<'_>,
        parser: F,
    ) -> ParserResult {
        if MIN_COUNT > 1 {
            unimplemented!("RepetitionHelper only supports min_count of 0 or 1")
        }

        let mut accum = match parser(input) {
            // First item parsed correctly
            | result @ ParserResult::Match(_) => result,
            | result @ ParserResult::PrattOperatorMatch(_) => result,
            // Or recovering, push it upwards
            | result @ ParserResult::SkippedUntil(_) => return result,

            // Couldn't get a full match but we allow 0 items - return an empty match
            // so the parse is considered valid but note the expected tokens
            | ParserResult::NoMatch(NoMatch { expected_tokens }) if MIN_COUNT == 0 => {
                return ParserResult::r#match(vec![], expected_tokens);
            }
            // Don't try repeating if we don't have a full match and we require at least one
            | incomplete_or_no_match => return incomplete_or_no_match,
        };

        loop {
            let save = input.mark();
            let next_result = parser(input);

            match (&mut accum, next_result) {
                | (ParserResult::Match(running), ParserResult::Match(next)) => {
                    running.nodes.extend(next.nodes);
                    running.expected_tokens = next.expected_tokens;
                }

                | (
                    ParserResult::PrattOperatorMatch(cur),
                    ParserResult::PrattOperatorMatch(next),
                ) => {
                    cur.elements.extend(next.elements);
                }

                | (ParserResult::Match(..), ParserResult::PrattOperatorMatch(..)) => unreachable!(
                    "PrattOperatorMatch seen while repeating Matches in RepetitionHelper"
                ),
                | (ParserResult::PrattOperatorMatch(..), ParserResult::Match(..)) => unreachable!(
                    "Match seen while repeating PrattOperatorMatches in RepetitionHelper"
                ),
                // Can't proceed further with a complete parse, so back up, return
                // the accumulated result and note the expected tokens
                | (
                    ParserResult::Match(running),
                    ParserResult::IncompleteMatch(IncompleteMatch {
                        expected_tokens, ..
                    })
                    | ParserResult::NoMatch(NoMatch { expected_tokens }),
                ) => {
                    input.rewind(save);
                    running.expected_tokens = expected_tokens;
                    return accum;
                }

                | (
                    ParserResult::PrattOperatorMatch(_),
                    ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_),
                ) => {
                    input.rewind(save);
                    return accum;
                }

                | (
                    ParserResult::PrattOperatorMatch(pratt),
                    ParserResult::SkippedUntil(mut skipped),
                ) => {
                    skipped.nodes = std::mem::take(&mut pratt.elements)
                        .into_iter()
                        .flat_map(PrattElement::into_nodes)
                        .chain(skipped.nodes)
                        .collect();

                    return ParserResult::SkippedUntil(skipped);
                }

                | (ParserResult::Match(running), ParserResult::SkippedUntil(mut skipped)) => {
                    running.nodes.extend(std::mem::take(&mut skipped.nodes));
                    skipped.nodes = std::mem::take(&mut running.nodes);

                    return ParserResult::SkippedUntil(skipped);
                }

                | (
                    ParserResult::IncompleteMatch(..)
                    | ParserResult::NoMatch(..)
                    | ParserResult::SkippedUntil(..),
                    _,
                ) => {
                    unreachable!("Variants never constructed")
                }
            }
        }
    }
}
