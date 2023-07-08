// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{
    cst,
    parser_result::{ParserResult, PrattOperatorMatch},
};

impl ParserResult {
    pub fn incorporate_sequence_result(&mut self, next_result: ParserResult) -> bool {
        if let ParserResult::Match(running_match) = self {
            match next_result {
                ParserResult::Match(r#match) => {
                    if r#match.nodes.len() > 0 {
                        running_match.nodes.extend(r#match.nodes);
                        running_match.tokens_that_would_have_allowed_more_progress =
                            r#match.tokens_that_would_have_allowed_more_progress;
                    } else {
                        running_match
                            .tokens_that_would_have_allowed_more_progress
                            .extend(r#match.tokens_that_would_have_allowed_more_progress);
                    }
                    return true;
                }
                ParserResult::IncompleteMatch(incomplete_match) => {
                    running_match.nodes.extend(incomplete_match.nodes);
                    let mut nodes = vec![];
                    std::mem::swap(&mut running_match.nodes, &mut nodes);
                    *self = ParserResult::incomplete_match(
                        nodes,
                        incomplete_match.tokens_that_would_have_allowed_more_progress,
                    );
                }
                ParserResult::NoMatch(no_match) if running_match.nodes.len() > 0 => {
                    running_match
                        .tokens_that_would_have_allowed_more_progress
                        .extend(no_match.tokens_that_would_have_allowed_more_progress);
                    let mut nodes = vec![];
                    std::mem::swap(&mut running_match.nodes, &mut nodes);
                    let mut tokens_that_would_have_allowed_more_progress = vec![];
                    std::mem::swap(
                        &mut running_match.tokens_that_would_have_allowed_more_progress,
                        &mut tokens_that_would_have_allowed_more_progress,
                    );
                    *self = ParserResult::incomplete_match(
                        nodes,
                        tokens_that_would_have_allowed_more_progress,
                    );
                }
                ParserResult::NoMatch(_) => {
                    *self = next_result;
                }
                ParserResult::PrattOperatorMatch(_) => {
                    unreachable!("Pratt operators are not supported in sequence expressions")
                }
            }
        } else {
            unreachable!("Sequence has continued to run after it has already failed")
        }
        return false;
    }

    pub fn incorporate_choice_result(&mut self, next_result: ParserResult) -> bool {
        match next_result {
            ParserResult::Match(_) => {
                *self = next_result;
                return true;
            }
            ParserResult::IncompleteMatch(_) => match self {
                ParserResult::IncompleteMatch(_) => {
                    if next_result.is_better_match_than(self) {
                        *self = next_result;
                    }
                }
                ParserResult::NoMatch(_) => *self = next_result,
                ParserResult::Match(_) => {
                    unreachable!("Choice has continued to run after it has already succeeded")
                }
                ParserResult::PrattOperatorMatch(_) => {
                    unreachable!("Pratt operators are not supported in choice expressions")
                }
            },
            ParserResult::NoMatch(no_match) => {
                if let ParserResult::NoMatch(running_no_match) = self {
                    running_no_match
                        .tokens_that_would_have_allowed_more_progress
                        .extend(no_match.tokens_that_would_have_allowed_more_progress)
                }
            }
            ParserResult::PrattOperatorMatch(_) => {
                unreachable!("Pratt operators are not supported in choice expressions")
            }
        }
        return false;
    }

    pub fn incorporate_zero_or_more_result(&mut self, result: ParserResult) -> bool {
        if let ParserResult::Match(running_match) = self {
            match result {
                ParserResult::Match(r#match) => {
                    running_match.nodes.extend(r#match.nodes);
                    return true;
                }
                ParserResult::IncompleteMatch(r#match) => {
                    running_match.nodes.extend(r#match.nodes);
                    let mut nodes = vec![];
                    std::mem::swap(&mut nodes, &mut running_match.nodes);
                    *self = ParserResult::incomplete_match(
                        nodes,
                        r#match.tokens_that_would_have_allowed_more_progress,
                    );
                }
                ParserResult::NoMatch(no_match) => {
                    let mut nodes = vec![];
                    std::mem::swap(&mut nodes, &mut running_match.nodes);
                    *self = ParserResult::r#match(
                        nodes,
                        no_match.tokens_that_would_have_allowed_more_progress,
                    );
                }
                ParserResult::PrattOperatorMatch(_) => {
                    unreachable!("Pratt operators are not supported in zero-or-more expressions")
                }
            }
        } else {
            unreachable!("ZeroOrMore has continued to run after it has already failed")
        }
        return false;
    }

    pub fn incorporate_one_or_more_result(&mut self, result: ParserResult) -> bool {
        if let ParserResult::Match(running_match) = self {
            match result {
                ParserResult::Match(r#match) => {
                    running_match.nodes.extend(r#match.nodes);
                    return true;
                }
                ParserResult::IncompleteMatch(r#match) => {
                    running_match.nodes.extend(r#match.nodes);
                    let mut nodes = vec![];
                    std::mem::swap(&mut nodes, &mut running_match.nodes);
                    *self = ParserResult::incomplete_match(
                        nodes,
                        r#match.tokens_that_would_have_allowed_more_progress,
                    );
                }
                ParserResult::NoMatch(no_match) if running_match.nodes.len() > 0 => {
                    let mut nodes = vec![];
                    std::mem::swap(&mut nodes, &mut running_match.nodes);
                    *self = ParserResult::r#match(
                        nodes,
                        no_match.tokens_that_would_have_allowed_more_progress,
                    );
                }
                ParserResult::NoMatch(_) => {
                    *self = result;
                }
                ParserResult::PrattOperatorMatch(_) => {
                    unreachable!("Pratt operators are not supported in one-or-more expressions")
                }
            }
        } else {
            unreachable!("OneOrMore has continued to run after it has already failed")
        }
        return false;
    }
}

pub fn transform_option_result(result: ParserResult) -> ParserResult {
    match result {
        ParserResult::Match(_) | ParserResult::IncompleteMatch(_) => result,
        ParserResult::NoMatch(no_match) => ParserResult::r#match(
            vec![],
            no_match.tokens_that_would_have_allowed_more_progress,
        ),
        ParserResult::PrattOperatorMatch(_) => {
            unreachable!("Pratt operators are not supported in optional expressions")
        }
    }
}

pub fn reduce_pratt_elements<F>(operator_argument_transformer: F, elements: &mut Vec<ParserResult>)
where
    F: Fn(Vec<cst::Node>) -> Vec<cst::Node>,
{
    let mut i = 0;
    while elements.len() > 1 {
        if let ParserResult::PrattOperatorMatch(PrattOperatorMatch {
            right_binding_power,
            left_binding_power,
            ..
        }) = &elements[i]
        {
            let next_left_binding_power = if elements.len() == i + 1 {
                // ... operator
                0
            } else if let ParserResult::PrattOperatorMatch(PrattOperatorMatch {
                left_binding_power,
                ..
            }) = &elements[i + 1]
            {
                // ... operator operator ...?
                *left_binding_power
            } else if elements.len() == i + 2 {
                // ... operator expr
                0
            } else if let ParserResult::PrattOperatorMatch(PrattOperatorMatch {
                left_binding_power,
                ..
            }) = &elements[i + 2]
            {
                // ... operator expr operator ...?
                *left_binding_power
            } else {
                unreachable!("This is a malformed pratt parser sequence")
            };

            if *right_binding_power <= next_left_binding_power {
                i += 1;
                continue;
            }

            if *right_binding_power == 255 {
                let left = elements.remove(i - 1);
                let op = elements.remove(i - 1);
                if let (
                    ParserResult::Match(left),
                    ParserResult::PrattOperatorMatch(PrattOperatorMatch {
                        nodes,
                        operator_kind,
                        ..
                    }),
                ) = (left, op)
                {
                    let mut children = vec![];
                    children.extend(operator_argument_transformer(left.nodes));
                    children.extend(nodes);
                    elements.insert(
                        i - 1,
                        ParserResult::r#match(
                            vec![cst::Node::rule(operator_kind, children)],
                            vec![],
                        ),
                    );
                    i = i.saturating_sub(2);
                } else {
                    unreachable!("This is a malformed pratt parser sequence")
                }
            } else if *left_binding_power == 255 {
                let op = elements.remove(i);
                let right = elements.remove(i);
                if let (
                    ParserResult::PrattOperatorMatch(PrattOperatorMatch {
                        nodes,
                        operator_kind,
                        ..
                    }),
                    ParserResult::Match(right),
                ) = (op, right)
                {
                    let mut children = vec![];
                    children.extend(nodes);
                    children.extend(operator_argument_transformer(right.nodes));
                    elements.insert(
                        i,
                        ParserResult::r#match(
                            vec![cst::Node::rule(operator_kind, children)],
                            vec![],
                        ),
                    );
                    i = i.saturating_sub(1);
                } else {
                    unreachable!("This is a malformed pratt parser sequence")
                }
            } else if 3 <= elements.len() {
                let left = elements.remove(i - 1);
                let op = elements.remove(i - 1);
                let right = elements.remove(i - 1);
                if let (
                    ParserResult::Match(left),
                    ParserResult::PrattOperatorMatch(PrattOperatorMatch {
                        nodes,
                        operator_kind,
                        ..
                    }),
                    ParserResult::Match(right),
                ) = (left, op, right)
                {
                    let mut children = vec![];
                    children.extend(operator_argument_transformer(left.nodes));
                    children.extend(nodes);
                    children.extend(operator_argument_transformer(right.nodes));
                    elements.insert(
                        i - 1,
                        ParserResult::r#match(
                            vec![cst::Node::rule(operator_kind, children)],
                            vec![],
                        ),
                    );
                    i = i.saturating_sub(2);
                } else {
                    unreachable!("This is a malformed pratt parser sequence")
                }
            } else {
                // We have not enough elements because of an previous error
                let left = elements.remove(i - 1);
                let op = elements.remove(i - 1);
                if let (
                    ParserResult::Match(left),
                    ParserResult::PrattOperatorMatch(PrattOperatorMatch {
                        nodes,
                        operator_kind,
                        ..
                    }),
                ) = (left, op)
                {
                    let mut children = vec![];
                    children.extend(operator_argument_transformer(left.nodes));
                    children.extend(nodes);
                    elements.insert(
                        i - 1,
                        ParserResult::incomplete_match(
                            vec![cst::Node::rule(operator_kind, children)],
                            vec![],
                        ),
                    );
                    i = i.saturating_sub(2);
                } else {
                    unreachable!("This is a malformed pratt parser sequence")
                }
            }
        } else {
            i += 1;
        }
    }
}
