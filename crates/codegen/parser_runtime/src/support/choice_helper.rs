use super::{super::text_index::TextIndex, parser_result::ParserResult, stream::Stream};

pub struct ChoiceHelper {
    result: ParserResult,
    is_done: bool,
    start_position: TextIndex,
}

impl ChoiceHelper {
    pub fn new(stream: &mut Stream) -> Self {
        Self {
            result: ParserResult::no_match(vec![]),
            is_done: false,
            start_position: stream.position(),
        }
    }

    pub fn handle_next_result(&mut self, stream: &mut Stream, next_result: ParserResult) -> bool {
        if self.is_done {
            return true;
        }

        match &mut self.result {
            ParserResult::Match(_) => unreachable!("ChoiceHelper is done"),

            ParserResult::PrattOperatorMatch(_) => unreachable!("ChoiceHelper is done"),

            ParserResult::IncompleteMatch(ref mut running_result) => match next_result {
                ParserResult::Match(_) => {
                    self.result = next_result;
                }

                ParserResult::PrattOperatorMatch(_) => {
                    self.result = next_result;
                }

                ParserResult::IncompleteMatch(next_result) => {
                    if next_result.is_better_match_than(running_result) {
                        running_result.nodes = next_result.nodes;
                        running_result.tokens_that_would_have_allowed_more_progress =
                            next_result.tokens_that_would_have_allowed_more_progress;
                    }
                }

                ParserResult::NoMatch(_) => {}
            },

            ParserResult::NoMatch(ref mut running_result) => match next_result {
                ParserResult::Match(_) => {
                    self.result = next_result;
                }

                ParserResult::PrattOperatorMatch(_) => {
                    self.result = next_result;
                }

                ParserResult::IncompleteMatch(_) => {
                    self.result = next_result;
                }

                ParserResult::NoMatch(next_result) => running_result
                    .tokens_that_would_have_allowed_more_progress
                    .extend(next_result.tokens_that_would_have_allowed_more_progress),
            },
        }

        self.is_done = self.result.is_match();
        if !self.is_done {
            stream.set_position(self.start_position);
        }

        self.is_done
    }

    pub fn result(self, stream: &mut Stream) -> ParserResult {
        if let ParserResult::IncompleteMatch(incomplete_match) = &self.result {
            incomplete_match.consume_stream(stream);
        }
        self.result
    }
}
