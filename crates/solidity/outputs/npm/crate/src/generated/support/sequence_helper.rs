// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::ops::ControlFlow;

use crate::{cst, kinds::TokenKind};

use super::parser_result::{Match, ParserResult, PrattElement, SkippedUntil};

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

    /// Attempts to append the next result until we hit an incomplete/no match.
    fn attempt_append(&mut self, next_result: ParserResult) {
        match self.result {
            // Base case: we were just constructed, just take the next result
            State::Empty => self.result = State::Running(next_result),
            State::Running(ref mut result) => match (result, next_result) {
                // Can't proceed further, return what we have
                (ParserResult::IncompleteMatch(..) | ParserResult::NoMatch(..), _) => {
                    debug_assert!(self.is_done());
                    return;
                }

                // If the accumulated result is valid, but empty (e.g. we accepted an empty optional)
                // just take the next result
                (ParserResult::Match(running), next @ _) if running.nodes.is_empty() => {
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
                            .flat_map(|pratt| pratt.to_nodes())
                            .chain(next.nodes.into_iter())
                            .collect(),
                        next.expected_tokens,
                    ));
                }
                (ParserResult::PrattOperatorMatch(cur), ParserResult::NoMatch(next)) => {
                    self.result = State::Running(ParserResult::incomplete_match(
                        std::mem::take(&mut cur.elements)
                            .into_iter()
                            .flat_map(|pratt| pratt.to_nodes())
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
                (ParserResult::PrattOperatorMatch(_), ParserResult::SkippedUntil(_)) => todo!(),
                // See if we can recover
                (ParserResult::SkippedUntil(running), ParserResult::Match(next)) => {
                    let tokens: Vec<_> =
                        next.nodes.iter().filter_map(cst::Node::as_token).collect();
                    let mut rules = next.nodes.iter().filter_map(cst::Node::as_rule);

                    let is_single_token_with_trivia =
                        tokens.len() == 1 && rules.all(|rule| rule.kind.is_trivia());
                    let found_token = tokens.first().copied();

                    // We can only recover if the next node is a single token (with trivia) that's expected by us
                    // or an ancestor
                    if is_single_token_with_trivia
                        && found_token.map(|t| t.kind) == Some(running.expected)
                    {
                        running.nodes.push(cst::Node::token(
                            TokenKind::SKIPPED,
                            std::mem::take(&mut running.skipped),
                        ));
                        running.nodes.extend(next.nodes);

                        self.result = State::Running(ParserResult::Match(Match {
                            nodes: std::mem::take(&mut running.nodes),
                            expected_tokens: next.expected_tokens,
                        }));
                    } else if is_single_token_with_trivia {
                        running.nodes.push(cst::Node::token(
                            TokenKind::SKIPPED,
                            std::mem::take(&mut running.skipped),
                        ));
                        running.nodes.extend(next.nodes);

                        self.result = State::Running(ParserResult::Match(Match {
                            nodes: std::mem::take(&mut running.nodes),
                            expected_tokens: next.expected_tokens,
                        }));
                    } else {
                        todo!()
                    }
                }
                // Otherwise, let the outer parse deal with that
                (ParserResult::SkippedUntil(_), _) => {
                    return;
                }
            },
        }
    }

    /// Executes a closure that allows the caller to drive the sequence parse.
    ///
    /// Useful when you want to eagerly return a result from the parse function (e.g. when we can't make more progress).
    ///
    /// Usage:
    /// ```no_run
    /// # use codegen_parser_runtime::support::{ParserResult, SequenceHelper};
    /// # fn parse_something() -> ParserResult { ParserResult::r#match(vec![], vec![]) }
    /// # fn parse_another() -> ParserResult { ParserResult::r#match(vec![], vec![]) }
    /// SequenceHelper::run(|mut sequence| {
    ///     sequence.elem(parse_something())?;
    ///     sequence.elem(parse_another())?;
    ///     sequence.finish()
    /// });
    /// ```
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
