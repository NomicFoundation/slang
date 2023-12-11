use std::ops::ControlFlow;

use super::parser_result::{Match, ParserResult, PrattElement, SkippedUntil};
use crate::cst;
use crate::kinds::TokenKind;

/// Keeps accumulating parses sequentially until it hits an incomplete or no match.
#[must_use]
#[derive(Default)]
pub struct SequenceHelper {
    result: State,
}

#[derive(Default, Debug)]
enum State {
    #[default]
    Empty,
    Running(ParserResult),
}

impl SequenceHelper {
    pub fn new() -> Self {
        SequenceHelper {
            result: State::Empty,
        }
    }

    /// Whether the sequence cannot make more progress.
    pub fn is_done(&self) -> bool {
        matches!(
            self.result,
            State::Running(ParserResult::IncompleteMatch(..) | ParserResult::NoMatch(..))
        )
    }

    #[allow(clippy::too_many_lines)] // Big switch that purely defines the sequence logic
    /// Attempts to append the next result until we hit an incomplete/no match.
    fn attempt_append(&mut self, next_result: ParserResult) {
        match self.result {
            // Base case: we were just constructed, just take the next result
            State::Empty => self.result = State::Running(next_result),
            State::Running(ref mut result) => match (result, next_result) {
                // Can't proceed further, return what we have
                (ParserResult::IncompleteMatch(..) | ParserResult::NoMatch(..), _) => {
                    debug_assert!(self.is_done());
                }

                // If the accumulated result is valid, but empty (e.g. we accepted an empty optional)
                // just take the next result
                (ParserResult::Match(running), next) if running.nodes.is_empty() => {
                    self.result = State::Running(next);
                }
                // Keep accepting or convert into PrattOperatorMatch
                (ParserResult::Match(running), ParserResult::Match(next)) => {
                    running.nodes.extend(next.nodes);
                    running.expected_tokens = next.expected_tokens;
                }
                (ParserResult::Match(running), ParserResult::PrattOperatorMatch(next)) => {
                    let mut children = vec![PrattElement::Expression {
                        nodes: std::mem::take(&mut running.nodes),
                    }];
                    children.extend(next.elements);
                    self.result = State::Running(ParserResult::pratt_operator_match(children));
                }
                // End of a valid sequence, finish with an incomplete match
                (ParserResult::Match(running), ParserResult::IncompleteMatch(next)) => {
                    running.nodes.extend(next.nodes);
                    self.result = State::Running(ParserResult::incomplete_match(
                        std::mem::take(&mut running.nodes),
                        next.expected_tokens,
                    ));
                }
                (ParserResult::Match(running), ParserResult::NoMatch(next)) => {
                    running.expected_tokens.extend(next.expected_tokens);

                    self.result = State::Running(ParserResult::incomplete_match(
                        std::mem::take(&mut running.nodes),
                        std::mem::take(&mut running.expected_tokens),
                    ));
                }
                // Keep accepting or convert Match -> PrattOperatorMatch
                (ParserResult::PrattOperatorMatch(running), ParserResult::Match(next)) => {
                    if !next.nodes.is_empty() {
                        running
                            .elements
                            .push(PrattElement::Expression { nodes: next.nodes });
                    }
                }
                (ParserResult::PrattOperatorMatch(cur), ParserResult::PrattOperatorMatch(next)) => {
                    cur.elements.extend(next.elements);
                }
                // End of a valid sequence, finish with an incomplete match
                (ParserResult::PrattOperatorMatch(cur), ParserResult::IncompleteMatch(next)) => {
                    self.result = State::Running(ParserResult::incomplete_match(
                        std::mem::take(&mut cur.elements)
                            .into_iter()
                            .flat_map(PrattElement::into_nodes)
                            .chain(next.nodes)
                            .collect(),
                        next.expected_tokens,
                    ));
                }
                (ParserResult::PrattOperatorMatch(cur), ParserResult::NoMatch(next)) => {
                    self.result = State::Running(ParserResult::incomplete_match(
                        std::mem::take(&mut cur.elements)
                            .into_iter()
                            .flat_map(PrattElement::into_nodes)
                            .collect(),
                        next.expected_tokens,
                    ));
                }
                // Enter recovery mode
                (ParserResult::Match(running), ParserResult::SkippedUntil(mut skipped)) => {
                    running.nodes.extend(std::mem::take(&mut skipped.nodes));
                    self.result = State::Running(ParserResult::SkippedUntil(SkippedUntil {
                        nodes: std::mem::take(&mut running.nodes),
                        ..skipped
                    }));
                }

                (ParserResult::PrattOperatorMatch(_), ParserResult::SkippedUntil(_)) =>
                    unreachable!("Error recovery happens outside precedence parsing"),

                // Try to recover until we hit an expected boundary token.
                // If the sequence is unwinding, then a subsequent non-empty match must mean that
                // we found the expected token, so we can stop recovering.
                (ParserResult::SkippedUntil(running), ParserResult::Match(next)) => {
                    if next.nodes.is_empty() {
                        return;
                    }

                    let tokens: Vec<_> =
                        next.nodes.iter().filter_map(cst::Node::as_token).collect();
                    let mut rules = next.nodes.iter().filter_map(cst::Node::as_rule);

                    let is_single_token_with_trivia =
                        tokens.len() == 1 && rules.all(|rule| rule.kind.is_trivia());
                    let next_token = tokens.first().map(|token| token.kind);

                    // NOTE: We only support skipping to a single token (optionally with trivia)
                    debug_assert!(is_single_token_with_trivia);
                    debug_assert_eq!(next_token, Some(running.found));

                    running.nodes.push(cst::Node::token(
                        TokenKind::SKIPPED,
                        std::mem::take(&mut running.skipped),
                    ));
                    running.nodes.extend(next.nodes);

                    self.result = State::Running(ParserResult::Match(Match {
                        nodes: std::mem::take(&mut running.nodes),
                        expected_tokens: next.expected_tokens,
                    }));
                }
                // If the sequence is unwinding and and we didn't find a match, then it means
                // that we recovered past it and we need to push the recovery up.
                (ParserResult::SkippedUntil(_), ParserResult::NoMatch(next)) => {
                    assert!(
                        matches!(next.expected_tokens[..], [_]),
                        "Only a single token parse can immediately follow SkippedUntil in sequences"
                    );
                }
                (ParserResult::SkippedUntil(_), _) => unreachable!(
                    "Only a single token parse can immediately follow SkippedUntil in sequences and these can either be Match or NoMatch"
                ),
            },
        }
    }

    /// Executes a closure that allows the caller to drive the sequence parse.
    ///
    /// Useful when you want to eagerly return a result from the parse function (e.g. when we can't make more progress).
    pub fn run(f: impl FnOnce(Self) -> ControlFlow<ParserResult, Self>) -> ParserResult {
        match f(SequenceHelper::default()) {
            ControlFlow::Break(result) => result,
            ControlFlow::Continue(helper) => helper.unwrap_result(),
        }
    }

    /// Aggregates a parse result into the sequence. If we cannot make progress, returns the accumulated match.
    pub fn elem(&mut self, value: ParserResult) -> ControlFlow<ParserResult, &mut Self> {
        self.attempt_append(value);

        if self.is_done() {
            ControlFlow::Break(self.take_result())
        } else {
            ControlFlow::Continue(self)
        }
    }

    /// Finishes the sequence parse, returning the accumulated match.
    pub fn finish(self) -> ControlFlow<ParserResult, Self> {
        ControlFlow::Break(self.unwrap_result())
    }

    fn take_result(&mut self) -> ParserResult {
        match std::mem::take(&mut self.result) {
            State::Empty => panic!("SequenceHelper was not driven"),
            State::Running(result) => result,
        }
    }

    fn unwrap_result(self) -> ParserResult {
        match self.result {
            State::Empty => panic!("SequenceHelper was not driven"),
            State::Running(result) => result,
        }
    }
}
