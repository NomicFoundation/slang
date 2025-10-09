use std::mem;
use std::ops::ControlFlow;

use crate::cst::{Node, TerminalKindExtensions, TextIndex};
use crate::parser::parser_support::context::{Marker, ParserContext};
use crate::parser::parser_support::ParserResult;
use crate::parser::ParseError;

/// Starting from a given position in the input, this helper will try to pick (and remember) a best match. Settles on
/// a first full match if possible, otherwise on the best incomplete match.
#[must_use]
pub struct ChoiceHelper {
    result: ParserResult,
    start_position: Marker,
    // Because we backtrack after every non-final pick, we store the progress
    // and the emitted errors from the time of a best pick, so that we can return to it later.
    last_progress: usize,
    recovered_errors: Vec<ParseError>,
}

impl ChoiceHelper {
    pub fn new(input: &mut ParserContext<'_>) -> Self {
        Self {
            result: ParserResult::default(),
            start_position: input.mark(),
            recovered_errors: vec![],
            last_progress: input.position(),
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
    fn attempt_pick(&mut self, input: &mut ParserContext<'_>, next_result: ParserResult) {
        let better_pick = match (&mut self.result, &next_result) {
            // We settle for the first full match.
            (ParserResult::Match(running), _) if running.is_full_recursive() => {
                debug_assert!(self.is_done());
                return;
            }
            (ParserResult::PrattOperatorMatch(..), _) => {
                debug_assert!(self.is_done());
                return;
            }

            // Still no match, extend the possible expected terminals.
            (ParserResult::NoMatch(running), ParserResult::NoMatch(next)) => {
                running
                    .expected_terminals
                    .extend(next.expected_terminals.clone());
                false
            }
            // Otherwise, we already have some match, so we ignore a next missing one.
            (_, ParserResult::NoMatch(..)) => false,

            // Try to improve our match.
            (_, ParserResult::Match(next)) if next.is_full_recursive() => true,
            (_, ParserResult::PrattOperatorMatch(..)) => true,
            // Optimize for matches that have a longer span of non-skipped terminals.
            (cur, next) => total_not_skipped_span(cur) < total_not_skipped_span(next),
        };

        // Store currently accumulated errors if we had a better pick.
        // We rewind the stream with each new consideration, so we need a way to come back
        // to the errors that were accumulated at the time of the best pick.
        if better_pick {
            self.result = next_result;
            self.recovered_errors = input.errors_since(self.start_position).to_vec();
            self.last_progress = input.position();
        }
    }

    /// Executes a closure that allows the caller to drive the choice parse.
    ///
    /// Useful when you want to eagerly return a result from the parse function (e.g. when the choice was fully matched).
    pub fn run(
        input: &mut ParserContext<'_>,
        f: impl FnOnce(Self, &mut ParserContext<'_>) -> ControlFlow<ParserResult, Self>,
    ) -> ParserResult {
        match f(ChoiceHelper::new(input), input) {
            ControlFlow::Break(result) => result,
            ControlFlow::Continue(..) => {
                panic!("ChoiceHelper not `finish`()-ed in the `run` closure")
            }
        }
    }

    /// Aggregates a choice result into the accumulator.
    ///
    /// If a value is considered as a full match, it is returned, otherwise we backtrack and continue.
    pub fn consider(
        &mut self,
        input: &mut ParserContext<'_>,
        value: ParserResult,
    ) -> ControlFlow<ParserResult, &mut ChoiceHelper> {
        self.attempt_pick(input, value);

        if self.is_done() {
            ControlFlow::Break(mem::take(&mut self.result))
        } else {
            input.rewind(self.start_position);
            ControlFlow::Continue(self)
        }
    }

    /// Finishes the choice parse, returning the accumulated match.
    pub fn finish(self, input: &mut ParserContext<'_>) -> ControlFlow<ParserResult, Self> {
        assert!(!self.is_done());
        // We didn't break early, so undo the rewind that has happened in the meantime.
        input.set_position(self.last_progress);
        input.extend_errors(self.recovered_errors);

        ControlFlow::Break(self.result)
    }
}

/// Returns the total length of the span of terminals that were not skipped.
pub fn total_not_skipped_span(result: &ParserResult) -> usize {
    let nodes = match result {
        ParserResult::Match(match_) => &match_.nodes,
        ParserResult::IncompleteMatch(incomplete_match) => &incomplete_match.nodes,
        ParserResult::SkippedUntil(skipped) => &skipped.nodes,
        ParserResult::NoMatch(_) => &[][..],
        ParserResult::PrattOperatorMatch(_) => unreachable!(
            "PrattOperatorMatch is always considered a better pick, so it should never be considered here"
        ),
    };

    nodes
        .iter()
        .flat_map(|edge| {
            edge.node
                .clone()
                .create_cursor(TextIndex::ZERO)
                .remaining_nodes()
        })
        .filter_map(|edge| match edge.node {
            Node::Terminal(terminal) if terminal.kind.is_valid() => Some(terminal.text.len()),
            _ => None,
        })
        .sum()
}
