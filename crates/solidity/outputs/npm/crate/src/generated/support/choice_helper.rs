// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::ops::ControlFlow;

use crate::parse_error::ParseError;

use super::{context::Marker, ParserContext, ParserResult};

/// Starting from a given position in the input, this helper will try to pick (and remember) a best match. Settles on
/// a first full match if possible, otherwise on the best incomplete match.
#[must_use]
pub struct ChoiceHelper {
    result: ParserResult,
    start_position: Marker,
    recovered_errors: Vec<ParseError>,
}

impl ChoiceHelper {
    pub fn new(input: &mut ParserContext) -> Self {
        Self {
            result: ParserResult::no_match(vec![]),
            start_position: input.mark(),
            recovered_errors: vec![],
        }
    }

    /// Whether the choice has found and settled on a full match.
    pub fn is_done(&self) -> bool {
        match &self.result {
            ParserResult::Match(r#match) if r#match.is_full_recursive() => true,
            ParserResult::PrattOperatorMatch(..) => true,
            _ => false,
        }
    }

    /// Store the next result if it's a better match; otherwise, we retain the existing one.
    fn attempt_pick(&mut self, input: &mut ParserContext, next_result: ParserResult) {
        let mut better_pick = false;

        match (&mut self.result, next_result) {
            // We settle for the first full match.
            (ParserResult::Match(running), _) if running.is_full_recursive() => {
                debug_assert!(self.is_done());
                return;
            }
            (ParserResult::PrattOperatorMatch(..), _) => {
                debug_assert!(self.is_done());
                return;
            }

            // Still no match, extend the possible expected tokens.
            (ParserResult::NoMatch(running), ParserResult::NoMatch(next)) => {
                running.expected_tokens.extend(next.expected_tokens)
            }
            // Otherwise, we have some match and we ignore a missing next one.
            (ParserResult::IncompleteMatch(..), ParserResult::NoMatch(..)) => {}
            (ParserResult::SkippedUntil(..), ParserResult::NoMatch(..)) => {}
            (ParserResult::Match(..), ParserResult::NoMatch(..)) => {}

            // Try to improve our match.
            // If the match has been recovered and is not full, optimize for the greatest matching span.
            (ParserResult::Match(running), ParserResult::Match(next))
                if !running.is_full_recursive() =>
            {
                if next.matching_recursive() > running.matching_recursive() {
                    self.result = ParserResult::Match(next);
                    better_pick = true;
                }
            }
            (ParserResult::Match(running), ParserResult::IncompleteMatch(next))
                if !running.is_full_recursive() =>
            {
                if next.matching_recursive() > running.matching_recursive() {
                    self.result = ParserResult::IncompleteMatch(next);
                }
            }
            (ParserResult::Match(running), ParserResult::SkippedUntil(next))
                if !running.is_full_recursive() =>
            {
                if next.matching_recursive() > running.matching_recursive() {
                    self.result = ParserResult::SkippedUntil(next);
                }
            }
            // If we only have incomplete matches and the next covers more bytes, then we take it...
            (ParserResult::IncompleteMatch(running), ParserResult::IncompleteMatch(next)) => {
                if next.covers_more_than(&running) {
                    self.result = ParserResult::IncompleteMatch(next);
                    better_pick = true;
                }
            }
            (ParserResult::IncompleteMatch(running), ParserResult::Match(next))
                if !next.is_full_recursive() =>
            {
                if next.matching_recursive() > running.matching_recursive() {
                    self.result = ParserResult::Match(next);
                    better_pick = true;
                }
            }
            (ParserResult::SkippedUntil(running), ParserResult::SkippedUntil(next)) => {
                if next.matching_recursive() > running.matching_recursive() {
                    self.result = ParserResult::SkippedUntil(next);
                    better_pick = true;
                }
            }
            // Otherwise, the next match will always be better.
            (_, next) => {
                self.result = next;
                better_pick = true;
            }
        }

        // Store currently accumulated errors if we had a better pick.
        // We rewind the stream with each new consideration, so we need a way to come back
        // to the errors that were accumulated at the time of the best pick.
        if better_pick {
            self.recovered_errors = input.errors_since(self.start_position).to_vec();
        }
    }

    /// Executes a closure that allows the caller to drive the choice parse.
    ///
    /// Useful when you want to eagerly return a result from the parse function (e.g. when the choice was fully matched).
    ///
    /// Usage:
    /// ```no_run
    /// # use codegen_parser_runtime::support::{ParserResult, ChoiceHelper, Stream};
    /// # fn parse_something() -> ParserResult { ParserResult::r#match(vec![], vec![]) }
    /// # fn parse_another() -> ParserResult { ParserResult::r#match(vec![], vec![]) }
    /// ChoiceHelper::run(input, |mut choice| {
    ///     choice.consider(parse_something()).pick_or_backtrack(input)?;
    ///     choice.consider(parse_another()).pick_or_backtrack(input)?;
    ///     choice.finish(input)
    /// });
    /// ```
    pub fn run(
        input: &mut ParserContext,
        f: impl FnOnce(Self, &mut ParserContext) -> ControlFlow<ParserResult, Self>,
    ) -> ParserResult {
        match f(ChoiceHelper::new(input), input) {
            ControlFlow::Break(result) => result,
            ControlFlow::Continue(..) => panic!("ChoiceHelper not finish()-ed in the run closure"),
        }
    }

    /// Aggregates a choice result into the accumulator.
    ///
    /// Returns a [`Choice`] struct that can be used to either pick the value or backtrack the input.
    pub fn consider(
        &mut self,
        input: &mut ParserContext,
        value: ParserResult,
    ) -> ControlFlow<ParserResult, &mut ChoiceHelper> {
        self.attempt_pick(input, value);

        if self.is_done() {
            ControlFlow::Break(self.take_result())
        } else {
            input.rewind(self.start_position);
            ControlFlow::Continue(self)
        }
    }

    /// Finishes the choice parse, returning the accumulated match.
    pub fn finish(self, input: &mut ParserContext) -> ControlFlow<ParserResult, Self> {
        ControlFlow::Break(self.unwrap_result(input))
    }

    fn take_result(&mut self) -> ParserResult {
        assert!(
            self.is_done(),
            "We only short-circuit Choice when we have a full match"
        );

        std::mem::replace(&mut self.result, ParserResult::no_match(vec![]))
    }

    fn unwrap_result(self, input: &mut ParserContext) -> ParserResult {
        // When finalizing a choice, we must advance the stream to the end of the match
        if !self.is_done() {
            match &self.result {
                ParserResult::IncompleteMatch(incomplete_match) => {
                    incomplete_match.consume_stream(input)
                }
                ParserResult::Match(match_) if !match_.is_full_recursive() => {
                    for node in &match_.nodes {
                        for _ in 0..node.text_len().char {
                            input.next();
                        }
                    }
                    // Inject the accumulated errors at the time of our best pick.
                    input.extend_errors(self.recovered_errors);
                }
                ParserResult::SkippedUntil(skipped) => {
                    for node in &skipped.nodes {
                        for _ in 0..node.text_len().char {
                            input.next();
                        }
                    }
                    for _ in skipped.skipped.chars() {
                        input.next();
                    }
                    input.extend_errors(self.recovered_errors);
                }
                _ => {}
            }
        }

        self.result
    }
}
