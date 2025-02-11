use std::ops::ControlFlow;

use crate::cst::{Edge, EdgeLabel, Node, TerminalKind, TerminalKindExtensions};
use crate::parser::parser_support::parser_result::{
    Match, ParserResult, PrattElement, SkippedUntil,
};

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
                    running.expected_terminals = next.expected_terminals;
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
                        next.expected_terminals,
                    ));
                }
                (ParserResult::Match(running), ParserResult::NoMatch(next)) => {
                    running.expected_terminals.extend(next.expected_terminals);

                    self.result = State::Running(ParserResult::incomplete_match(
                        std::mem::take(&mut running.nodes),
                        std::mem::take(&mut running.expected_terminals),
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
                            .flat_map(|e| e.into_nodes_with_label(EdgeLabel::Unrecognized))
                            .chain(next.nodes)
                            .collect(),
                        next.expected_terminals,
                    ));
                }
                (ParserResult::PrattOperatorMatch(cur), ParserResult::NoMatch(next)) => {
                    self.result = State::Running(ParserResult::incomplete_match(
                        std::mem::take(&mut cur.elements)
                            .into_iter()
                            .flat_map(|e| e.into_nodes_with_label(EdgeLabel::Unrecognized))
                            .collect(),
                        next.expected_terminals,
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

                (ParserResult::PrattOperatorMatch(running), ParserResult::SkippedUntil(skipped)) => {
                    let nodes: Vec<_> = std::mem::take(&mut running.elements)
                            .into_iter()
                            .flat_map(|e| e.into_nodes_with_label(EdgeLabel::Unrecognized))
                            .chain(skipped.nodes)
                            .collect();

                    self.result = State::Running(ParserResult::SkippedUntil(SkippedUntil {
                        nodes,
                        ..skipped
                    }));
                }

                // Try to recover until we hit an expected boundary terminal.
                // If the sequence is unwinding, then a subsequent non-empty match must mean that
                // we found the expected terminal, so we can stop recovering.
                (ParserResult::SkippedUntil(running), ParserResult::Match(next)) => {
                    if next.nodes.is_empty() {
                        return;
                    }

                    // We only support skipping to a single, significant terminal.
                    // Sanity check that we are recovering to the expected one.
                    let next_terminal = next.nodes.iter().try_fold(None, |acc, node| {
                        match &**node {
                            Node::Terminal(terminal) if terminal.kind.is_trivia() => Ok(acc),
                            Node::Terminal(terminal) => {
                                if acc.is_none() {
                                    Ok(Some(terminal.kind))
                                } else {
                                    debug_assert!(false, "Recovery skipped to multiple terminals: {acc:?}, {terminal:?}");
                                    Err(())
                                }
                            }
                            Node::Nonterminal(node) => {
                                debug_assert!(false, "Recovery skipped to a nonterminal: {node:?}");
                                Err(())
                            }
                        }
                    });
                    debug_assert_eq!(next_terminal, Ok(Some(running.found)));

                    let (kind, label) = if running.skipped.is_empty() {
                        (TerminalKind::MISSING, EdgeLabel::Missing)
                    } else {
                        (TerminalKind::UNRECOGNIZED, EdgeLabel::Unrecognized)
                    };

                    running.nodes.push(Edge{label, node: Node::terminal(
                        kind,
                        std::mem::take(&mut running.skipped),
                    )});
                    running.nodes.extend(next.nodes);

                    self.result = State::Running(ParserResult::Match(Match {
                        nodes: std::mem::take(&mut running.nodes),
                        expected_terminals: next.expected_terminals,
                    }));
                }
                // If the sequence is unwinding and we didn't find a match, then it means
                // that we recovered past it and we need to push the recovery up.
                (ParserResult::SkippedUntil(_), ParserResult::NoMatch(_)) => {
                    // Skip any possible subsequent expected elements in this sequence until
                    // we finally encounter the terminal we were looking for
                }
                (ParserResult::SkippedUntil(_), _) => unreachable!(
                    "Only a single terminal parse can immediately follow SkippedUntil in sequences and these can either be Match or NoMatch"
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

    /// Aggregates a parse result into the sequence. If we cannot make progress, returns the accumulated match.
    ///
    /// Shorthand for `self.elem(value.with_label(label))`.
    pub fn elem_labeled(
        &mut self,
        label: EdgeLabel,
        value: ParserResult,
    ) -> ControlFlow<ParserResult, &mut Self> {
        self.elem(value.with_label(label))
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
