use super::{parser_result::ParserResult, stream::Stream};

pub struct RepetitionHelper<const MIN_COUNT: usize>;

pub type ZeroOrMoreHelper = RepetitionHelper<0>;
pub type OneOrMoreHelper = RepetitionHelper<1>;

impl<const MIN_COUNT: usize> RepetitionHelper<MIN_COUNT> {
    pub fn run<F: Fn(&mut Stream) -> ParserResult>(stream: &mut Stream, parser: F) -> ParserResult {
        if MIN_COUNT > 1 {
            unimplemented!("RepetionHelper only supports min_count of 0 or 1")
        }

        let save = stream.position();
        let mut result = parser(stream);

        match &mut result {
            ParserResult::Match(_) => {}

            ParserResult::PrattOperatorMatch(_) => {}

            ParserResult::IncompleteMatch(incomplete_match) => {
                if MIN_COUNT == 0 {
                    stream.set_position(save);
                    return ParserResult::r#match(
                        vec![],
                        std::mem::take(
                            &mut incomplete_match.tokens_that_would_have_allowed_more_progress,
                        ),
                    );
                } else {
                    return result;
                }
            }

            ParserResult::NoMatch(no_match) => {
                if MIN_COUNT == 0 {
                    stream.set_position(save);
                    return ParserResult::r#match(
                        vec![],
                        std::mem::take(&mut no_match.tokens_that_would_have_allowed_more_progress),
                    );
                } else {
                    return result;
                }
            }
        }

        loop {
            let save = stream.position();
            let mut next_result = parser(stream);

            match result {
                ParserResult::Match(ref mut current_match) => match &mut next_result {
                    ParserResult::Match(r#match) => {
                        current_match
                            .nodes
                            .extend(std::mem::take(&mut r#match.nodes));
                        current_match.tokens_that_would_have_allowed_more_progress = std::mem::take(
                            &mut r#match.tokens_that_would_have_allowed_more_progress,
                        );
                    }

                    ParserResult::PrattOperatorMatch(_) => unreachable!(
                        "PrattOperatorMatch seen while repeating Matches in RepetionHelper"
                    ),

                    ParserResult::IncompleteMatch(incomplete_match) => {
                        stream.set_position(save);
                        current_match.tokens_that_would_have_allowed_more_progress = std::mem::take(
                            &mut incomplete_match.tokens_that_would_have_allowed_more_progress,
                        );
                        return result;
                    }

                    ParserResult::NoMatch(no_match) => {
                        stream.set_position(save);
                        current_match.tokens_that_would_have_allowed_more_progress = std::mem::take(
                            &mut no_match.tokens_that_would_have_allowed_more_progress,
                        );
                        return result;
                    }
                },

                ParserResult::PrattOperatorMatch(ref mut current_match) => {
                    match &mut next_result {
                        ParserResult::Match(_) => unreachable!(
                            "Match seen while repeating PrattOperatorMatches in RepetionHelper"
                        ),

                        ParserResult::PrattOperatorMatch(r#match) => {
                            current_match
                                .nodes
                                .extend(std::mem::take(&mut r#match.nodes));
                        }

                        ParserResult::IncompleteMatch(_) => {
                            stream.set_position(save);
                            return result;
                        }

                        ParserResult::NoMatch(_) => {
                            stream.set_position(save);
                            return result;
                        }
                    };
                }

                ParserResult::IncompleteMatch(_) => {
                    unreachable!("IncompleteMatch is never constructed")
                }

                ParserResult::NoMatch(_) => {
                    unreachable!("NoMatch is never constructed")
                }
            }
        }
    }
}
